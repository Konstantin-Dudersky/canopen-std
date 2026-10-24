mod pdo_mapping_item;

pub use pdo_mapping_item::PdoMappingItem;

use bitvec::{field::BitField, order::Lsb0, view::BitView};
use embedded_can::{ExtendedId, Id, StandardId};

use crate::{types::Value, Error};

/// Декодировать COB ID из словаря.
///
/// COB ID хранится в словаре как `unsigned32`.
///
/// бит 29 - тип адреса (Standard / Extended).
pub fn decode_cob_id(value: Value) -> Result<(bool, bool, Id), Error> {
    let raw = value.get_unsigned32()?;

    let bits = raw.view_bits::<Lsb0>();

    let frame_type = bits[29];

    let id = match frame_type {
        true => {
            let id_raw: u32 = bits[0..29].load_le();
            let id = ExtendedId::new(id_raw).ok_or(Error::InvalidCobId)?;
            Id::Extended(id)
        }
        false => {
            let id_raw: u16 = bits[0..11].load_le();
            let id = StandardId::new(id_raw).ok_or(Error::InvalidCobId)?;
            Id::Standard(id)
        }
    };

    Ok((bits[31], bits[30], id))
}

/// Закодировать COB ID в значение словаря.
///
/// Обратная операция к [`decode_cob_id`]. Результат — `unsigned32`.
///
/// бит 29 - тип адреса (Standard / Extended).
pub fn encode_cob_id(bit_31: bool, bit_30: bool, id: Id) -> Value {
    let mut raw = 0u32;

    let bits = raw.view_bits_mut::<Lsb0>();

    match id {
        Id::Extended(id) => {
            bits[0..29].store_le(id.as_raw());
            bits.set(29, true);
        }
        Id::Standard(id) => {
            bits[0..11].store_le(id.as_raw());
            bits.set(29, false);
        }
    }

    bits.set(30, bit_30);
    bits.set(31, bit_31);

    Value::Unsigned32(raw)
}
