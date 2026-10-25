#![allow(clippy::module_inception)]

mod object_dictionary;

mod rpdo_communication;
mod rpdo_mapping;
mod sdo_server;
mod tpdo_communication;
mod tpdo_mapping;

/// 0x1000
mod device_type;

/// 0x1001
mod error_register;

/// 0x1005
mod sync_message_cob_id;

/// 0x1006
mod comm_cycle_period;

/// 0x1008
mod manufacturer_device_name;

/// 0x1009
mod manufacturer_hardware_version;

/// 0x100A
mod manufacturer_software_version;

/// 0x1012
mod time_stamp_cob_id;

/// 0x1013
mod high_resolution_time_stamp;

/// 0x1016
mod consumer_heartbeat_time;

/// 0x1017
mod producer_heartbeat_time;

/// 0x1018
mod identity_object;

/// 0x1019
mod sync_counter_overflow;

/// 0x1029
mod error_behavior;

pub use {
    comm_cycle_period::CommCyclePeriod,
    consumer_heartbeat_time::{ConsumerHeartbeatTime, ConsumerHeartbeatTimeItem},
    device_type::DeviceType,
    error_behavior::{ErrorBehavior, ErrorBehaviorAction},
    error_register::ErrorRegister,
    identity_object::IdentityObject,
    manufacturer_device_name::ManufacturerDeviceName,
    manufacturer_hardware_version::ManufacturerHardwareVersion,
    manufacturer_software_version::ManufacturerSoftwareVersion,
    object_dictionary::ObjectDictionary,
    producer_heartbeat_time::ProducerHeartbeatTime,
    rpdo_communication::{RpdoCommunication, RpdoTransmissionType},
    rpdo_mapping::RpdoMapping,
    sdo_server::SdoServer,
    sync_counter_overflow::SyncCounterOverflow,
    sync_message_cob_id::SyncMessageCobId,
    tpdo_communication::{TpdoCommunication, TpdoCommunicationCobId, TpdoTransmissionType},
    tpdo_mapping::TpdoMapping,
};

use crate::types::{AccessType, Object, ObjectVar, Value};
