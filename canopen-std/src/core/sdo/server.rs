//! SDO server — services requests against an object dictionary.
//!
//! [`SdoServer`] is a sans-I/O state machine: feed it a decoded request frame
//! with [`SdoServer::handle`] and it reads or writes the supplied
//! [`ObjectDictionary`], returning the response frame to transmit (or `None`
//! for a client abort, which needs no reply). It handles expedited transfers
//! in a single exchange and drives segmented transfers — needed for values of
//! five to eight bytes, such as `UNSIGNED64` — across multiple frames, holding
//! the per-transfer state between calls.

use super::super::object_dictionary::Address;
use super::{
    CCS_DOWNLOAD_INITIATE, CCS_DOWNLOAD_SEGMENT, CCS_UPLOAD_INITIATE, CCS_UPLOAD_SEGMENT, CS_ABORT,
    CS_MASK, EXPEDITED, SEGMENT_DATA_MAX, SIZE_INDICATED, SdoAbortCode, SdoPayload, TOGGLE,
    decode_data_segment, decode_download_initiate_segmented, encode_abort, encode_data_segment,
    encode_download_response, encode_download_segment_response, encode_upload_expedited_response,
    encode_upload_initiate_segmented_response,
};
use crate::{
    Error,
    objects::ObjectDictionary,
    types::{DataType, MAX_STRING_LEN, Value},
};
use embedded_can::Id;

/// An in-progress segmented transfer held between frames.
///
/// The buffers are sized to [`MAX_STRING_LEN`], the largest value the server
/// transfers — eight bytes covers every numeric type, and the rest is headroom
/// for the variable-length (`VISIBLE_STRING` / `OCTET_STRING` / `DOMAIN`) types.
#[derive(Debug)]
enum Transfer {
    /// The client is reading a value larger than four bytes from us.
    Upload {
        addr: Address,
        data: [u8; MAX_STRING_LEN],
        len: usize,
        pos: usize,
        toggle: bool,
    },
    /// The client is writing a value larger than four bytes to us.
    Download {
        addr: Address,
        data_type: DataType,
        buf: Vec<u8>,
        declared: usize,
        toggle: bool,
    },
}

/// An SDO server bound to a node id, servicing requests against an object
/// dictionary.
#[derive(Debug)]
pub struct SdoServer {
    request_cob_id: Id,
    response_cob_id: Id,
    transfer: Option<Transfer>,
    last_write: Option<Address>,
}

impl SdoServer {
    /// Create a server for `node`.
    pub const fn new(request_cob_id: Id, response_cob_id: Id) -> Self {
        Self {
            request_cob_id,
            response_cob_id,
            transfer: None,
            last_write: None,
        }
    }

    /// The COB-ID this server receives requests on (`0x600 + node`).
    pub fn request_cob_id(&self) -> Id {
        self.request_cob_id
    }

    /// The COB-ID this server sends responses on (`0x580 + node`).
    pub fn response_cob_id(&self) -> Id {
        self.response_cob_id
    }

    /// Whether a segmented transfer is currently in progress.
    pub fn is_busy(&self) -> bool {
        self.transfer.is_some()
    }

    /// Take the address of the object most recently written by a completed SDO
    /// download, clearing it. Poll after [`SdoServer::handle`] to react to a
    /// master's writes (e.g. re-read configuration). `None` if nothing was
    /// written since the last call.
    pub fn take_write(&mut self) -> Option<Address> {
        self.last_write.take()
    }

    /// Handle an SDO request against `od`, returning the response to transmit.
    ///
    /// Returns `None` only for a client abort, which is unconfirmed. Any
    /// protocol or access error produces an SDO abort response frame.
    pub fn on_frame(&mut self, od: &mut ObjectDictionary, req: &[u8]) -> Option<SdoPayload> {
        let mut payload: SdoPayload = [0u8; 8];
        if req.len() > payload.len() {
            return None;
        }
        payload[..req.len()].copy_from_slice(req);

        match req[0] & CS_MASK {
            CCS_DOWNLOAD_INITIATE => self.on_download_initiate(od, &payload),
            CCS_UPLOAD_INITIATE => self.on_upload_initiate(od, &payload),
            CCS_DOWNLOAD_SEGMENT => self.on_download_segment(od, &payload),
            CCS_UPLOAD_SEGMENT => self.on_upload_segment(&payload),
            CS_ABORT => {
                self.transfer = None;
                None
            }
            _ => abort(req_address(&payload), SdoAbortCode::CommandInvalid),
        }
    }

    fn on_download_initiate(
        &mut self,
        od: &mut ObjectDictionary,
        req: &SdoPayload,
    ) -> Option<SdoPayload> {
        let addr = req_address(req);
        let (data_type, writable) = match od.get_entry(addr.index, addr.subindex) {
            Some(e) => (e.value.data_type(), e.access_type.is_writable()),
            None => return abort(addr, SdoAbortCode::ObjectDoesNotExist),
        };
        if !writable {
            return abort(addr, SdoAbortCode::WriteOfReadOnly);
        }

        if req[0] & EXPEDITED != 0 {
            // Expedited write: 0..=4 data bytes inline.
            let len = if req[0] & SIZE_INDICATED != 0 {
                4 - ((req[0] >> 2) & 0x03) as usize
            } else {
                data_type.fixed_size().unwrap_or(0)
            };
            // Expedited carries at most four data bytes; a larger fixed type
            // must use segmented transfer. Guard the slice against a malformed
            // frame (e.g. size-indication clear on an 8-byte object).
            if len > 4 {
                return abort(addr, SdoAbortCode::DataTypeMismatchLengthHigh);
            }
            // A fixed-size type must match exactly; a variable-length one takes
            // the indicated length as its (short) content.
            if let Some(fixed) = data_type.fixed_size() {
                if len != fixed {
                    return abort(addr, SdoAbortCode::DataTypeMismatchLengthHigh);
                }
            }
            let value = match Value::decode_le(data_type, &req[4..4 + len]) {
                Ok(v) => v,
                Err(_) => return abort(addr, SdoAbortCode::General),
            };
            write_value(od, addr, value, &mut self.last_write)
        } else {
            // Segmented write: declare the size now, receive segments later.
            let (_, size) = match decode_download_initiate_segmented(req) {
                Ok(x) => x,
                Err(_) => return abort(addr, SdoAbortCode::CommandInvalid),
            };
            // A fixed-size type must match; a variable-length one must fit the
            // buffer.
            let ok = match data_type.fixed_size() {
                Some(fixed) => size as usize == fixed,
                None => size as usize <= MAX_STRING_LEN,
            };
            if !ok {
                return abort(addr, SdoAbortCode::DataTypeMismatchLengthHigh);
            }
            self.transfer = Some(Transfer::Download {
                addr,
                data_type,
                buf: Vec::new(),
                declared: size as usize,
                toggle: false,
            });
            Some(encode_download_response(addr))
        }
    }

    fn on_upload_initiate(
        &mut self,
        od: &mut ObjectDictionary,
        req: &SdoPayload,
    ) -> Option<SdoPayload> {
        let addr = req_address(req);
        let value = match od.read(addr.index, addr.subindex) {
            Ok(v) => v,
            Err(Error::ObjectNotFound) => return abort(addr, SdoAbortCode::ObjectDoesNotExist),
            Err(Error::WriteOnly) => return abort(addr, SdoAbortCode::ReadOfWriteOnly),
            Err(_) => return abort(addr, SdoAbortCode::General),
        };
        let size = value.size();
        if (1..=4).contains(&size) {
            Some(encode_upload_expedited_response(addr, &value).expect("size 1..=4"))
        } else {
            let mut data = [0u8; MAX_STRING_LEN];
            value
                .encode_le(&mut data[..size])
                .expect("size <= MAX_STRING_LEN");
            self.transfer = Some(Transfer::Upload {
                addr,
                data,
                len: size,
                pos: 0,
                toggle: false,
            });
            Some(encode_upload_initiate_segmented_response(addr, size as u32))
        }
    }

    fn on_upload_segment(&mut self, req: &SdoPayload) -> Option<SdoPayload> {
        let want_toggle = req[0] & TOGGLE != 0;
        match self.transfer.take() {
            Some(Transfer::Upload {
                addr,
                data,
                len,
                pos,
                toggle,
            }) => {
                if want_toggle != toggle {
                    return abort(addr, SdoAbortCode::ToggleBitNotAlternated);
                }
                let remaining = len - pos;
                let n = remaining.min(SEGMENT_DATA_MAX);
                let last = remaining <= SEGMENT_DATA_MAX;
                let seg = encode_data_segment(&data[pos..pos + n], toggle, last).expect("1..=7");
                if !last {
                    self.transfer = Some(Transfer::Upload {
                        addr,
                        data,
                        len,
                        pos: pos + n,
                        toggle: !toggle,
                    });
                }
                Some(seg)
            }
            _ => abort(Address::new(0, 0), SdoAbortCode::CommandInvalid),
        }
    }

    fn on_download_segment(
        &mut self,
        od: &mut ObjectDictionary,
        req: &SdoPayload,
    ) -> Option<SdoPayload> {
        let seg = match decode_data_segment(req) {
            Ok(s) => s,
            Err(_) => return abort(Address::new(0, 0), SdoAbortCode::CommandInvalid),
        };
        match self.transfer.take() {
            Some(Transfer::Download {
                addr,
                data_type,
                mut buf,
                declared,
                toggle,
            }) => {
                if seg.toggle != toggle {
                    return abort(addr, SdoAbortCode::ToggleBitNotAlternated);
                }
                buf.extend_from_slice(seg.data);
                let ack = encode_download_segment_response(seg.toggle);
                if seg.last {
                    if buf.len() != declared {
                        return abort(addr, SdoAbortCode::DataTypeMismatchLengthLow);
                    }
                    let value = match Value::decode_le(data_type, &buf) {
                        Ok(v) => v,
                        Err(_) => return abort(addr, SdoAbortCode::General),
                    };
                    // The final segment is acknowledged with a segment
                    // response, not a download-initiate response.
                    match od.write(addr.index, addr.subindex, value) {
                        Ok(()) => {
                            self.last_write = Some(addr);
                            Some(ack)
                        }
                        Err(Error::ReadOnly) => abort(addr, SdoAbortCode::WriteOfReadOnly),
                        Err(_) => abort(addr, SdoAbortCode::General),
                    }
                } else {
                    self.transfer = Some(Transfer::Download {
                        addr,
                        data_type,
                        buf,
                        declared,
                        toggle: !toggle,
                    });
                    Some(ack)
                }
            }
            _ => abort(Address::new(0, 0), SdoAbortCode::CommandInvalid),
        }
    }
}

/// Write `value` to `od`, recording the address on success and mapping OD
/// errors to SDO abort responses.
fn write_value(
    od: &mut ObjectDictionary,
    addr: Address,
    value: Value,
    last_write: &mut Option<Address>,
) -> Option<SdoPayload> {
    match od.write(addr.index, addr.subindex, value) {
        Ok(()) => {
            *last_write = Some(addr);
            Some(encode_download_response(addr))
        }
        Err(Error::ReadOnly) => abort(addr, SdoAbortCode::WriteOfReadOnly),
        Err(Error::TypeMismatch) => abort(addr, SdoAbortCode::DataTypeMismatchLengthHigh),
        Err(_) => abort(addr, SdoAbortCode::General),
    }
}

fn req_address(req: &SdoPayload) -> Address {
    Address::new(u16::from_le_bytes([req[1], req[2]]), req[3])
}

fn abort(addr: Address, code: SdoAbortCode) -> Option<SdoPayload> {
    Some(encode_abort(addr, code))
}

#[cfg(test)]
mod tests {

    use embedded_can::StandardId;

    use crate::types::{AccessType, NodeId, ObjectVar};

    use super::*;
    fn server() -> SdoServer {
        let request_cob_id: Id = StandardId::new(0x601).unwrap().into();
        let response_cob_id: Id = StandardId::new(0x581).unwrap().into();
        SdoServer::new(request_cob_id, response_cob_id)
    }

    fn od() -> ObjectDictionary {
        let mut od = ObjectDictionary::new(NodeId::new(1).unwrap());
        od.insert_object(ObjectVar {
            index: 0x1000,
            name: "".into(),
            access_type: AccessType::Const,
            pdo_mapping: false,
            value: Value::Unsigned32(0x0000_0192),
        });
        od.insert_object(ObjectVar {
            index: 0x1017,
            name: "".into(),
            access_type: AccessType::Rw,
            pdo_mapping: false,
            value: Value::Unsigned16(1000),
        });

        od.insert_object(ObjectVar {
            index: 0x2000,
            name: "".into(),
            access_type: AccessType::Rw,
            pdo_mapping: false,
            value: Value::Unsigned64(0),
        });
        od
    }

    #[test]
    fn cob_ids_track_node() {
        let request_cob_id: Id = StandardId::new(0x605).unwrap().into();
        let response_cob_id: Id = StandardId::new(0x585).unwrap().into();
        let s = SdoServer::new(request_cob_id, response_cob_id);
        assert_eq!(s.request_cob_id(), StandardId::new(0x605).unwrap().into());
        assert_eq!(s.response_cob_id(), StandardId::new(0x585).unwrap().into());
    }

    #[test]
    fn expedited_read_returns_value() {
        let mut od = od();
        let req = super::super::encode_upload_request(Address::new(0x1000, 0));
        let resp = server().on_frame(&mut od, &req).unwrap();
        let (_, value) =
            super::super::decode_upload_expedited_response(&resp, DataType::Unsigned32).unwrap();
        assert_eq!(value, Value::Unsigned32(0x192));
    }

    #[test]
    fn expedited_write_updates_od() {
        let mut od = od();
        let req = super::super::encode_download_expedited(
            Address::new(0x1017, 0),
            &Value::Unsigned16(1234),
        )
        .unwrap();
        let resp = server().on_frame(&mut od, &req).unwrap();
        assert!(super::super::decode_download_response(&resp).is_ok());
        assert_eq!(od.read(0x1017, 0).unwrap(), Value::Unsigned16(1234));
    }

    #[test]
    fn take_write_reports_the_written_object_once() {
        let mut od = od();
        let mut s = server();
        // Nothing written yet.
        assert_eq!(s.take_write(), None);

        let req = super::super::encode_download_expedited(
            Address::new(0x1017, 0),
            &Value::Unsigned16(1234),
        )
        .unwrap();
        s.on_frame(&mut od, &req).unwrap();
        // The write is reported once, then cleared.
        assert_eq!(s.take_write(), Some(Address::new(0x1017, 0)));
        assert_eq!(s.take_write(), None);

        // A failed write (read-only object) is not reported.
        let ro =
            super::super::encode_download_expedited(Address::new(0x1000, 0), &Value::Unsigned32(1))
                .unwrap();
        s.on_frame(&mut od, &ro).unwrap();
        assert_eq!(s.take_write(), None);
    }

    #[test]
    fn read_missing_object_aborts() {
        let mut od = od();
        let req = super::super::encode_upload_request(Address::new(0x9999, 0));
        let resp = server().on_frame(&mut od, &req).unwrap();
        let (_, code) = super::super::decode_abort(&resp).unwrap();
        assert_eq!(code, 0x0602_0000);
    }

    #[test]
    fn write_read_only_aborts() {
        let mut od = od();
        let req =
            super::super::encode_download_expedited(Address::new(0x1000, 0), &Value::Unsigned32(1))
                .unwrap();
        let resp = server().on_frame(&mut od, &req).unwrap();
        let (_, code) = super::super::decode_abort(&resp).unwrap();
        assert_eq!(code, 0x0601_0002); // write of a read-only object
    }

    #[test]
    fn client_abort_clears_state_without_reply() {
        let mut od = od();
        let mut s = server();
        // Start a segmented download so the server is busy.
        let init = super::super::encode_download_initiate_segmented(Address::new(0x2000, 0), 8);
        s.on_frame(&mut od, &init).unwrap();
        assert!(s.is_busy());
        let abort_frame =
            super::super::encode_abort(Address::new(0x2000, 0), SdoAbortCode::General);
        assert!(s.on_frame(&mut od, &abort_frame).is_none());
        assert!(!s.is_busy());
    }
}
