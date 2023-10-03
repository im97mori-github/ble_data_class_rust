//! Data type parser module.

use super::{
    advertising_interval::{is_advertising_interval, AdvertisingInterval},
    advertising_interval_long::{is_advertising_interval_long, AdvertisingIntervalLong},
    appearance::{is_appearance, Appearance},
    big_info::{is_big_info, BigInfo},
    broadcast_code::{is_broadcast_code, BroadcastCode},
    channel_map_update_indication::{is_channel_map_update_indication, ChannelMapUpdateIndication},
    class_of_device::{is_class_of_device, ClassOfDevice},
    complete_list_of_128bit_service_uuids::{
        is_complete_list_of_128bit_service_uuids, CompleteListOf128BitServiceUuids,
    },
    complete_list_of_16bit_service_uuids::{
        is_complete_list_of_16bit_service_uuids, CompleteListOf16BitServiceUuids,
    },
    complete_list_of_32bit_service_uuids::{
        is_complete_list_of_32bit_service_uuids, CompleteListOf32BitServiceUuids,
    },
    complete_local_name::{is_complete_local_name, CompleteLocalName},
    encrypted_data::{is_encrypted_data, EncryptedData},
    flags::{is_flags, Flags},
    incomplete_list_of_128bit_service_uuids::{
        is_incomplete_list_of_128bit_service_uuids, IncompleteListOf128BitServiceUuids,
    },
    incomplete_list_of_16bit_service_uuids::{
        is_incomplete_list_of_16bit_service_uuids, IncompleteListOf16BitServiceUuids,
    },
    incomplete_list_of_32bit_service_uuids::{
        is_incomplete_list_of_32bit_service_uuids, IncompleteListOf32BitServiceUuids,
    },
    le_bluetooth_device_address::{is_le_bluetooth_device_address, LeBluetoothDeviceAddress},
    le_role::{is_le_role, LeRole},
    le_secure_connections_confirmation_value::{
        is_le_secure_connections_confirmation_value, LeSecureConnectionsConfirmationValue,
    },
    le_secure_connections_random_value::{
        is_le_secure_connections_random_value, LeSecureConnectionsRandomValue,
    },
    le_supported_features::{is_le_supported_features, LeSupportedFeatures},
    list_of_128bit_service_solicitation_uuids::{
        is_list_of_128bit_service_solicitation_uuids, ListOf128BitServiceSolicitationUUIDs,
    },
    list_of_16bit_service_solicitation_uuids::{
        is_list_of_16bit_service_solicitation_uuids, ListOf16BitServiceSolicitationUUIDs,
    },
    list_of_32bit_service_solicitation_uuids::{
        is_list_of_32bit_service_solicitation_uuids, ListOf32BitServiceSolicitationUUIDs,
    },
    manufacturer_specific_data::{is_manufacturer_specific_data, ManufacturerSpecificData},
    periodic_advertising_response_timing_information::{
        is_periodic_advertising_response_timing_information,
        PeriodicAdvertisingResponseTimingInformation,
    },
    peripheral_connection_interval_range::{
        is_peripheral_connection_interval_range, PeripheralConnectionIntervalRange,
    },
    public_target_address::{is_public_target_address, PublicTargetAddress},
    random_target_address::{is_random_target_address, RandomTargetAddress},
    secure_simple_pairing_hash_c192::{
        is_secure_simple_pairing_hash_c192, SecureSimplePairingHashC192,
    },
    secure_simple_pairing_hash_c256::{
        is_secure_simple_pairing_hash_c256, SecureSimplePairingHashC256,
    },
    secure_simple_pairing_randomizer_r192::{
        is_secure_simple_pairing_randomizer_r192, SecureSimplePairingRandomizerR192,
    },
    secure_simple_pairing_randomizer_r256::{
        is_secure_simple_pairing_randomizer_r256, SecureSimplePairingRandomizerR256,
    },
    security_manager_oob::{is_security_manager_oob, SecurityManagerOutOfBand},
    security_manager_tk_value::{is_security_manager_tk_value, SecurityManagerTkValue},
    service_data_128bit_uuid::{is_service_data_128bit_uuid, ServiceData128BitUUID},
    service_data_16bit_uuid::{is_service_data_16bit_uuid, ServiceData16BitUUID},
    service_data_32bit_uuid::{is_service_data_32bit_uuid, ServiceData32BitUUID},
    shortened_local_name::{is_shortened_local_name, ShortenedLocalName},
    tx_power_level::{is_tx_power_level, TxPowerLevel},
};

/// Data type parse result.
pub enum DataTypeParseResult {
    /// [`AdvertisingInterval`]'s [`TryFrom::try_from`] result.
    AdvertisingIntervalResult(Result<AdvertisingInterval, String>),

    /// [`AdvertisingIntervalLong`]'s [`TryFrom::try_from`] result.
    AdvertisingIntervalLongResult(Result<AdvertisingIntervalLong, String>),

    /// [`Appearance`]'s [`TryFrom::try_from`] result.
    AppearanceResult(Result<Appearance, String>),

    /// [`BigInfo`]'s [`TryFrom::try_from`] result.
    BigInfoResult(Result<BigInfo, String>),

    /// [`BroadcastCode`]'s [`TryFrom::try_from`] result.
    BroadcastCodeResult(Result<BroadcastCode, String>),

    /// [`ChannelMapUpdateIndication`]'s [`TryFrom::try_from`] result.
    ChannelMapUpdateIndicationResult(Result<ChannelMapUpdateIndication, String>),

    /// [`ClassOfDevice`]'s [`TryFrom::try_from`] result.
    ClassOfDeviceResult(Result<ClassOfDevice, String>),

    /// [`CompleteListOf128BitServiceUuids`]'s [`TryFrom::try_from`] result.
    CompleteListOf128BitServiceUuidsResult(Result<CompleteListOf128BitServiceUuids, String>),

    /// [`CompleteListOf16BitServiceUuids`]'s [`TryFrom::try_from`] result.
    CompleteListOf16BitServiceUuidsResult(Result<CompleteListOf16BitServiceUuids, String>),

    /// [`CompleteListOf32BitServiceUuids`]'s [`TryFrom::try_from`] result.
    CompleteListOf32BitServiceUuidsResult(Result<CompleteListOf32BitServiceUuids, String>),

    /// [`CompleteLocalName`]'s [`TryFrom::try_from`] result.
    CompleteLocalNameResult(Result<CompleteLocalName, String>),

    /// [`EncryptedData`]'s [`TryFrom::try_from`] result.
    EncryptedDataResult(Result<EncryptedData, String>),

    /// [`Flags`]'s [`TryFrom::try_from`] result.
    FlagsResult(Result<Flags, String>),

    /// [`IncompleteListOf128BitServiceUuids`]'s [`TryFrom::try_from`] result.
    IncompleteListOf128BitServiceUuidsResult(Result<IncompleteListOf128BitServiceUuids, String>),

    /// [`IncompleteListOf16BitServiceUuids`]'s [`TryFrom::try_from`] result.
    IncompleteListOf16BitServiceUuidsResult(Result<IncompleteListOf16BitServiceUuids, String>),

    /// [`IncompleteListOf32BitServiceUuids`]'s [`TryFrom::try_from`] result.
    IncompleteListOf32BitServiceUuidsResult(Result<IncompleteListOf32BitServiceUuids, String>),

    /// [`LeBluetoothDeviceAddress`]'s [`TryFrom::try_from`] result.
    LeBluetoothDeviceAddressResult(Result<LeBluetoothDeviceAddress, String>),

    /// [`LeRole`]'s [`TryFrom::try_from`] result.
    LeRoleResult(Result<LeRole, String>),

    /// [`LeSecureConnectionsConfirmationValue`]'s [`TryFrom::try_from`] result.
    LeSecureConnectionsConfirmationValueResult(
        Result<LeSecureConnectionsConfirmationValue, String>,
    ),

    /// [`LeSecureConnectionsRandomValue`]'s [`TryFrom::try_from`] result.
    LeSecureConnectionsRandomValueResult(Result<LeSecureConnectionsRandomValue, String>),

    /// [`LeSupportedFeatures`]'s [`TryFrom::try_from`] result.
    LeSupportedFeaturesResult(Result<LeSupportedFeatures, String>),

    /// [`ListOf128BitServiceSolicitationUUIDs`]'s [`TryFrom::try_from`] result.
    ListOf128BitServiceSolicitationUUIDsResult(
        Result<ListOf128BitServiceSolicitationUUIDs, String>,
    ),

    /// [`ListOf16BitServiceSolicitationUUIDs`]'s [`TryFrom::try_from`] result.
    ListOf16BitServiceSolicitationUUIDsResult(Result<ListOf16BitServiceSolicitationUUIDs, String>),

    /// [`ListOf32BitServiceSolicitationUUIDs`]'s [`TryFrom::try_from`] result.
    ListOf32BitServiceSolicitationUUIDsResult(Result<ListOf32BitServiceSolicitationUUIDs, String>),

    /// [`ManufacturerSpecificData`]'s [`TryFrom::try_from`] result.
    ManufacturerSpecificDataResult(Result<ManufacturerSpecificData, String>),

    /// [`PeriodicAdvertisingResponseTimingInformation`]'s [`TryFrom::try_from`] result.
    PeriodicAdvertisingResponseTimingInformationResult(
        Result<PeriodicAdvertisingResponseTimingInformation, String>,
    ),

    /// [`PeripheralConnectionIntervalRange`]'s [`TryFrom::try_from`] result.
    PeripheralConnectionIntervalRangeResult(Result<PeripheralConnectionIntervalRange, String>),

    /// [`PublicTargetAddress`]'s [`TryFrom::try_from`] result.
    PublicTargetAddressResult(Result<PublicTargetAddress, String>),

    /// [`RandomTargetAddress`]'s [`TryFrom::try_from`] result.
    RandomTargetAddressResult(Result<RandomTargetAddress, String>),

    /// [`SecureSimplePairingHashC192`]'s [`TryFrom::try_from`] result.
    SecureSimplePairingHashC192Result(Result<SecureSimplePairingHashC192, String>),

    /// [`SecureSimplePairingHashC256`]'s [`TryFrom::try_from`] result.
    SecureSimplePairingHashC256Result(Result<SecureSimplePairingHashC256, String>),

    /// [`SecureSimplePairingRandomizerR192`]'s [`TryFrom::try_from`] result.
    SecureSimplePairingRandomizerR192Result(Result<SecureSimplePairingRandomizerR192, String>),

    /// [`SecureSimplePairingRandomizerR256`]'s [`TryFrom::try_from`] result.
    SecureSimplePairingRandomizerR256Result(Result<SecureSimplePairingRandomizerR256, String>),

    /// [`SecurityManagerOutOfBand`]'s [`TryFrom::try_from`] result.
    SecurityManagerOutOfBandResult(Result<SecurityManagerOutOfBand, String>),

    /// [`SecurityManagerTkValue`]'s [`TryFrom::try_from`] result.
    SecurityManagerTkValueResult(Result<SecurityManagerTkValue, String>),

    /// [`ServiceData128BitUUID`]'s [`TryFrom::try_from`] result.
    ServiceData128BitUUIDResult(Result<ServiceData128BitUUID, String>),

    /// [`ServiceData16BitUUID`]'s [`TryFrom::try_from`] result.
    ServiceData16BitUUIDResult(Result<ServiceData16BitUUID, String>),

    /// [`ServiceData32BitUUID`]'s [`TryFrom::try_from`] result.
    ServiceData32BitUUIDResult(Result<ServiceData32BitUUID, String>),

    /// [`ShortenedLocalName`]'s [`TryFrom::try_from`] result.
    ShortenedLocalNameResult(Result<ShortenedLocalName, String>),

    /// [`TxPowerLevel`]'s [`TryFrom::try_from`] result.
    TxPowerLevelResult(Result<TxPowerLevel, String>),

    /// Occurs for unsupported data types.
    DataTypeParseErr(String),
}

impl DataTypeParseResult {
    /// Returns `true` if the result is [`DataTypeParseResult::AdvertisingIntervalResult`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{advertising_interval::AdvertisingInterval, parser::DataTypeParseResult};
    ///
    /// let advertising_interval = 0x01;
    /// let data: Vec<u8> = AdvertisingInterval::new(advertising_interval).into();
    /// assert!(DataTypeParseResult::from(&data).is_advertising_interval());
    ///
    /// let data: Vec<u8> = Vec::new();
    /// assert!(!DataTypeParseResult::from(&data).is_advertising_interval());
    /// ```
    pub fn is_advertising_interval(&self) -> bool {
        matches!(self, DataTypeParseResult::AdvertisingIntervalResult(_))
    }

    /// Returns `true` if the result is [`DataTypeParseResult::AdvertisingIntervalLongResult`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{advertising_interval_long::AdvertisingIntervalLong, parser::DataTypeParseResult};
    ///
    /// let advertising_interval_long: u32 = 0x01020304u32;
    /// let data: Vec<u8> = AdvertisingIntervalLong::new(true, advertising_interval_long).into();
    /// assert!(DataTypeParseResult::from(&data).is_advertising_interval_long());
    ///
    /// let data: Vec<u8> = Vec::new();
    /// assert!(!DataTypeParseResult::from(&data).is_advertising_interval_long());
    /// ```
    pub fn is_advertising_interval_long(&self) -> bool {
        matches!(self, DataTypeParseResult::AdvertisingIntervalLongResult(_))
    }

    /// Returns `true` if the result is [`DataTypeParseResult::AppearanceResult`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{appearance::Appearance, parser::DataTypeParseResult};
    ///
    /// let appearance: u16 = 0x1444;
    /// let data: Vec<u8> = Appearance::new(appearance).into();
    /// assert!(DataTypeParseResult::from(&data).is_appearance());
    ///
    /// let data: Vec<u8> = Vec::new();
    /// assert!(!DataTypeParseResult::from(&data).is_appearance());
    /// ```
    pub fn is_appearance(&self) -> bool {
        matches!(self, DataTypeParseResult::AppearanceResult(_))
    }

    /// Returns `true` if the result is [`DataTypeParseResult::BigInfoResult`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{big_info::BigInfo, parser::DataTypeParseResult};
    ///
    /// let big_offset: u16 = 1;
    /// let big_offset_units: bool = true;
    /// let iso_interval: u16 = 2;
    /// let num_bis: u8 = 3;
    /// let nse: u8 = 4;
    /// let bn: u8 = 5;
    /// let sub_interval: u32 = 6;
    /// let pto: u8 = 7;
    /// let bis_spacing: u32 = 8;
    /// let irc: u8 = 9;
    /// let max_pdu: u8 = 10;
    /// let rfu: u8 = 11;
    /// let seed_access_address: u32 = 12;
    /// let sdu_interval: u32 = 13;
    /// let max_sdu: u16 = 14;
    /// let base_crc_init: u16 = 15;
    /// let ch_m: u64 = 16;
    /// let phy: u8 = 17;
    /// let bis_payload_count: u64 = 18;
    /// let framing: bool = false;
    /// let giv: Option<[u8; 8]> = None;
    /// let gskd: Option<[u8; 16]> = None;
    /// let data: Vec<u8> = BigInfo::new(
    ///     big_offset,
    ///     big_offset_units,
    ///     iso_interval,
    ///     num_bis,
    ///     nse,
    ///     bn,
    ///     sub_interval,
    ///     pto,
    ///     bis_spacing,
    ///     irc,
    ///     max_pdu,
    ///     rfu,
    ///     seed_access_address,
    ///     sdu_interval,
    ///     max_sdu,
    ///     base_crc_init,
    ///     ch_m,
    ///     phy,
    ///     bis_payload_count,
    ///     framing,
    ///     giv,
    ///     gskd,
    /// )
    /// .into();
    /// assert!(DataTypeParseResult::from(&data).is_big_info());
    ///
    /// let data: Vec<u8> = Vec::new();
    /// assert!(!DataTypeParseResult::from(&data).is_big_info());
    /// ```
    pub fn is_big_info(&self) -> bool {
        matches!(self, DataTypeParseResult::BigInfoResult(_))
    }

    /// Returns `true` if the result is [`DataTypeParseResult::BroadcastCodeResult`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{broadcast_code::BroadcastCode, parser::DataTypeParseResult};
    ///
    /// let broadcast_code = [0x00u8; 4].to_vec();
    /// let data: Vec<u8> = BroadcastCode::new(&broadcast_code).into();
    /// assert!(DataTypeParseResult::from(&data).is_broadcast_code());
    ///
    /// let data: Vec<u8> = Vec::new();
    /// assert!(!DataTypeParseResult::from(&data).is_broadcast_code());
    /// ```
    pub fn is_broadcast_code(&self) -> bool {
        matches!(self, DataTypeParseResult::BroadcastCodeResult(_))
    }

    /// Returns `true` if the result is [`DataTypeParseResult::ChannelMapUpdateIndicationResult`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{channel_map_update_indication::ChannelMapUpdateIndication, parser::DataTypeParseResult};
    ///
    /// let mut ch_m = [false; 37].to_vec();
    /// for i in 0..37 {
    ///     ch_m[i] = true;
    ///     let data = ChannelMapUpdateIndication::new(&ch_m, i as u16).into();
    ///     assert!(DataTypeParseResult::from(&data).is_channel_map_update_indication());
    ///     ch_m[i] = false;
    /// }
    ///
    /// let data: Vec<u8> = Vec::new();
    /// assert!(!DataTypeParseResult::from(&data).is_channel_map_update_indication());
    /// ```
    pub fn is_channel_map_update_indication(&self) -> bool {
        matches!(
            self,
            DataTypeParseResult::ChannelMapUpdateIndicationResult(_)
        )
    }

    /// Returns `true` if the result is [`DataTypeParseResult::ClassOfDeviceResult`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{class_of_device::ClassOfDevice, parser::DataTypeParseResult};
    ///
    /// let major_service_classes = 0b10000000_00000000_00000000;
    /// let major_device_class = 0b00000000_00000001_00000000;
    /// let minor_device_class = 0b00000000_00000000_00000100;
    /// let class_of_device = major_service_classes | major_device_class | minor_device_class;
    /// let data = ClassOfDevice::new(class_of_device).into();
    /// assert!(DataTypeParseResult::from(&data).is_class_of_device());
    ///
    /// let data: Vec<u8> = Vec::new();
    /// assert!(!DataTypeParseResult::from(&data).is_class_of_device());
    /// ```
    pub fn is_class_of_device(&self) -> bool {
        matches!(self, DataTypeParseResult::ClassOfDeviceResult(_))
    }

    /// Returns `true` if the result is [`DataTypeParseResult::CompleteListOf128BitServiceUuidsResult`].
    ///
    /// # Examples
    ///
    /// ```
    /// use uuid::{uuid, Uuid};
    /// use ble_data_struct::data_types::{complete_list_of_128bit_service_uuids::CompleteListOf128BitServiceUuids, parser::DataTypeParseResult};
    ///
    /// let uuids: Vec<Uuid> = [
    ///     uuid!("00000001-0000-1000-8000-00805F9B34FB"),
    ///     uuid!("00000002-0000-1000-8000-00805F9B34FB"),
    /// ]
    /// .to_vec();
    /// let data = CompleteListOf128BitServiceUuids::new(&uuids).into();
    /// assert!(DataTypeParseResult::from(&data).is_complete_list_of_128bit_service_uuids());
    ///
    /// let data: Vec<u8> = Vec::new();
    /// assert!(!DataTypeParseResult::from(&data).is_complete_list_of_128bit_service_uuids());
    /// ```
    pub fn is_complete_list_of_128bit_service_uuids(&self) -> bool {
        matches!(
            self,
            DataTypeParseResult::CompleteListOf128BitServiceUuidsResult(_)
        )
    }

    /// Returns `true` if the result is [`DataTypeParseResult::CompleteListOf16BitServiceUuidsResult`].
    ///
    /// # Examples
    ///
    /// ```
    /// use uuid::{uuid, Uuid};
    /// use ble_data_struct::data_types::{complete_list_of_16bit_service_uuids::CompleteListOf16BitServiceUuids, parser::DataTypeParseResult};
    ///
    /// let uuids: Vec<Uuid> = [
    ///     uuid!("00000001-0000-1000-8000-00805F9B34FB"),
    ///     uuid!("00000002-0000-1000-8000-00805F9B34FB"),
    /// ]
    /// .to_vec();
    /// let data = CompleteListOf16BitServiceUuids::new(&uuids).into();
    /// assert!(DataTypeParseResult::from(&data).is_complete_list_of_16bit_service_uuids());
    ///
    /// let data: Vec<u8> = Vec::new();
    /// assert!(!DataTypeParseResult::from(&data).is_complete_list_of_16bit_service_uuids());
    /// ```
    pub fn is_complete_list_of_16bit_service_uuids(&self) -> bool {
        matches!(
            self,
            DataTypeParseResult::CompleteListOf16BitServiceUuidsResult(_)
        )
    }

    /// Returns `true` if the result is [`DataTypeParseResult::CompleteListOf32BitServiceUuidsResult`].
    ///
    /// # Examples
    ///
    /// ```
    /// use uuid::{uuid, Uuid};
    /// use ble_data_struct::data_types::{complete_list_of_32bit_service_uuids::CompleteListOf32BitServiceUuids, parser::DataTypeParseResult};
    ///
    /// let uuids: Vec<Uuid> = [
    ///     uuid!("00000001-0000-1000-8000-00805F9B34FB"),
    ///     uuid!("00000002-0000-1000-8000-00805F9B34FB"),
    /// ]
    /// .to_vec();
    /// let data = CompleteListOf32BitServiceUuids::new(&uuids).into();
    /// assert!(DataTypeParseResult::from(&data).is_complete_list_of_32bit_service_uuids());
    ///
    /// let data: Vec<u8> = Vec::new();
    /// assert!(!DataTypeParseResult::from(&data).is_complete_list_of_32bit_service_uuids());
    /// ```
    pub fn is_complete_list_of_32bit_service_uuids(&self) -> bool {
        matches!(
            self,
            DataTypeParseResult::CompleteListOf32BitServiceUuidsResult(_)
        )
    }

    /// Returns `true` if the result is [`DataTypeParseResult::CompleteLocalNameResult`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{complete_local_name::CompleteLocalName, parser::DataTypeParseResult};
    ///
    /// let name = "complete_local_name".to_string();
    /// let data = CompleteLocalName::new(&name).into();
    /// assert!(DataTypeParseResult::from(&data).is_complete_local_name());
    ///
    /// let data: Vec<u8> = Vec::new();
    /// assert!(!DataTypeParseResult::from(&data).is_complete_local_name());
    /// ```
    pub fn is_complete_local_name(&self) -> bool {
        matches!(self, DataTypeParseResult::CompleteLocalNameResult(_))
    }

    /// Returns `true` if the result is [`DataTypeParseResult::EncryptedDataResult`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{encrypted_data::EncryptedData, parser::DataTypeParseResult};
    ///
    /// let randomizer: [u8; 5] = [1, 2, 3, 4, 5];
    /// let payload = [6].to_vec();
    /// let mic: [u8; 4] = [7, 8, 9, 10];
    /// let data = EncryptedData::new(&randomizer, &payload, mic).into();
    /// assert!(DataTypeParseResult::from(&data).is_encrypted_data());
    ///
    /// let data: Vec<u8> = Vec::new();
    /// assert!(!DataTypeParseResult::from(&data).is_encrypted_data());
    /// ```
    pub fn is_encrypted_data(&self) -> bool {
        matches!(self, DataTypeParseResult::EncryptedDataResult(_))
    }

    /// Returns `true` if the result is [`DataTypeParseResult::FlagsResult`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{flags::Flags, parser::DataTypeParseResult};
    ///
    /// let flags = [true, false, false, false, false, false, false, false].to_vec();
    /// let data = Flags::new(&flags).into();
    /// assert!(DataTypeParseResult::from(&data).is_flags());
    ///
    /// let data: Vec<u8> = Vec::new();
    /// assert!(!DataTypeParseResult::from(&data).is_flags());
    /// ```
    pub fn is_flags(&self) -> bool {
        matches!(self, DataTypeParseResult::FlagsResult(_))
    }

    /// Returns `true` if the result is [`DataTypeParseResult::IncompleteListOf128BitServiceUuidsResult`].
    ///
    /// # Examples
    ///
    /// ```
    /// use uuid::{uuid, Uuid};
    /// use ble_data_struct::data_types::{incomplete_list_of_128bit_service_uuids::IncompleteListOf128BitServiceUuids, parser::DataTypeParseResult};
    ///
    /// let uuids: Vec<Uuid> = [
    ///     uuid!("00000001-0000-1000-8000-00805F9B34FB"),
    ///     uuid!("00000002-0000-1000-8000-00805F9B34FB"),
    /// ]
    /// .to_vec();
    /// let data = IncompleteListOf128BitServiceUuids::new(&uuids).into();
    /// assert!(DataTypeParseResult::from(&data).is_incomplete_list_of_128bit_service_uuids());
    ///
    /// let data: Vec<u8> = Vec::new();
    /// assert!(!DataTypeParseResult::from(&data).is_incomplete_list_of_128bit_service_uuids());
    /// ```
    pub fn is_incomplete_list_of_128bit_service_uuids(&self) -> bool {
        matches!(
            self,
            DataTypeParseResult::IncompleteListOf128BitServiceUuidsResult(_)
        )
    }

    /// Returns `true` if the result is [`DataTypeParseResult::IncompleteListOf16BitServiceUuidsResult`].
    ///
    /// # Examples
    ///
    /// ```
    /// use uuid::{uuid, Uuid};
    /// use ble_data_struct::data_types::{incomplete_list_of_16bit_service_uuids::IncompleteListOf16BitServiceUuids, parser::DataTypeParseResult};
    ///
    /// let uuids: Vec<Uuid> = [
    ///     uuid!("00000001-0000-1000-8000-00805F9B34FB"),
    ///     uuid!("00000002-0000-1000-8000-00805F9B34FB"),
    /// ]
    /// .to_vec();
    /// let data = IncompleteListOf16BitServiceUuids::new(&uuids).into();
    /// assert!(DataTypeParseResult::from(&data).is_incomplete_list_of_16bit_service_uuids());
    ///
    /// let data: Vec<u8> = Vec::new();
    /// assert!(!DataTypeParseResult::from(&data).is_incomplete_list_of_16bit_service_uuids());
    /// ```
    pub fn is_incomplete_list_of_16bit_service_uuids(&self) -> bool {
        matches!(
            self,
            DataTypeParseResult::IncompleteListOf16BitServiceUuidsResult(_)
        )
    }

    /// Returns `true` if the result is [`DataTypeParseResult::IncompleteListOf32BitServiceUuidsResult`].
    ///
    /// # Examples
    ///
    /// ```
    /// use uuid::{uuid, Uuid};
    /// use ble_data_struct::data_types::{incomplete_list_of_32bit_service_uuids::IncompleteListOf32BitServiceUuids, parser::DataTypeParseResult};
    ///
    /// let uuids: Vec<Uuid> = [
    ///     uuid!("00000001-0000-1000-8000-00805F9B34FB"),
    ///     uuid!("00000002-0000-1000-8000-00805F9B34FB"),
    /// ]
    /// .to_vec();
    /// let data = IncompleteListOf32BitServiceUuids::new(&uuids).into();
    /// assert!(DataTypeParseResult::from(&data).is_incomplete_list_of_32bit_service_uuids());
    ///
    /// let data: Vec<u8> = Vec::new();
    /// assert!(!DataTypeParseResult::from(&data).is_incomplete_list_of_32bit_service_uuids());
    /// ```
    pub fn is_incomplete_list_of_32bit_service_uuids(&self) -> bool {
        matches!(
            self,
            DataTypeParseResult::IncompleteListOf32BitServiceUuidsResult(_)
        )
    }

    /// Returns `true` if the result is [`DataTypeParseResult::LeBluetoothDeviceAddressResult`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{le_bluetooth_device_address::LeBluetoothDeviceAddress, parser::DataTypeParseResult};
    ///
    /// let le_bluetooth_device_address = 0x0000060504030201u64;
    /// let address_type = false;
    /// let data = LeBluetoothDeviceAddress::new(le_bluetooth_device_address, address_type).into();
    /// assert!(DataTypeParseResult::from(&data).is_le_bluetooth_device_address());
    ///
    /// let data: Vec<u8> = Vec::new();
    /// assert!(!DataTypeParseResult::from(&data).is_le_bluetooth_device_address());
    /// ```
    pub fn is_le_bluetooth_device_address(&self) -> bool {
        matches!(self, DataTypeParseResult::LeBluetoothDeviceAddressResult(_))
    }

    /// Returns `true` if the result is [`DataTypeParseResult::LeRoleResult`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{le_role::*, parser::DataTypeParseResult};
    ///
    /// let le_role = ONLY_PERIPHERAL_ROLE_SUPPORTED;
    /// let data = LeRole::new(le_role).into();
    /// assert!(DataTypeParseResult::from(&data).is_le_role());
    ///
    /// let data: Vec<u8> = Vec::new();
    /// assert!(!DataTypeParseResult::from(&data).is_le_role());
    /// ```
    pub fn is_le_role(&self) -> bool {
        matches!(self, DataTypeParseResult::LeRoleResult(_))
    }

    /// Returns `true` if the result is [`DataTypeParseResult::LeSecureConnectionsConfirmationValueResult`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{le_secure_connections_confirmation_value::LeSecureConnectionsConfirmationValue, parser::DataTypeParseResult};
    ///
    /// let le_secure_connections_confirmation_value = 0x0102030405060708090a0b0c0d0e0f10u128;
    /// let data =
    ///     LeSecureConnectionsConfirmationValue::new(le_secure_connections_confirmation_value)
    ///         .into();
    /// assert!(DataTypeParseResult::from(&data).is_le_secure_connections_confirmation_value());
    ///
    /// let data: Vec<u8> = Vec::new();
    /// assert!(!DataTypeParseResult::from(&data).is_le_secure_connections_confirmation_value());
    /// ```
    pub fn is_le_secure_connections_confirmation_value(&self) -> bool {
        matches!(
            self,
            DataTypeParseResult::LeSecureConnectionsConfirmationValueResult(_)
        )
    }

    /// Returns `true` if the result is [`DataTypeParseResult::LeSecureConnectionsRandomValueResult`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{le_secure_connections_random_value::LeSecureConnectionsRandomValue, parser::DataTypeParseResult};
    ///
    /// let le_secure_connections_random_value = 0x0102030405060708090a0b0c0d0e0f10u128;
    ///
    /// let data = LeSecureConnectionsRandomValue::new(le_secure_connections_random_value).into();
    /// assert!(DataTypeParseResult::from(&data).is_le_secure_connections_random_value());
    ///
    /// let data: Vec<u8> = Vec::new();
    /// assert!(!DataTypeParseResult::from(&data).is_le_secure_connections_random_value());
    /// ```
    pub fn is_le_secure_connections_random_value(&self) -> bool {
        matches!(
            self,
            DataTypeParseResult::LeSecureConnectionsRandomValueResult(_)
        )
    }

    /// Returns `true` if the result is [`DataTypeParseResult::LeSupportedFeaturesResult`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{le_supported_features::LeSupportedFeatures, parser::DataTypeParseResult};
    ///
    /// let mut le_supported_features = [false; 48].to_vec();
    /// for i in 0..44 {
    ///     le_supported_features[i] = true;
    ///     let data = LeSupportedFeatures::new(&le_supported_features).into();
    ///     assert!(DataTypeParseResult::from(&data).is_le_supported_features());
    ///     le_supported_features[i] = false;
    /// }
    ///
    /// let data: Vec<u8> = Vec::new();
    /// assert!(!DataTypeParseResult::from(&data).is_le_supported_features());
    /// ```
    pub fn is_le_supported_features(&self) -> bool {
        matches!(self, DataTypeParseResult::LeSupportedFeaturesResult(_))
    }

    /// Returns `true` if the result is [`DataTypeParseResult::ListOf128BitServiceSolicitationUUIDsResult`].
    ///
    /// # Examples
    ///
    /// ```
    /// use uuid::{uuid, Uuid};
    /// use ble_data_struct::data_types::{list_of_128bit_service_solicitation_uuids::ListOf128BitServiceSolicitationUUIDs, parser::DataTypeParseResult};
    ///
    /// let uuids: Vec<Uuid> = [
    ///     uuid!("00000001-0000-1000-8000-00805F9B34FB"),
    ///     uuid!("00000002-0000-1000-8000-00805F9B34FB"),
    /// ]
    /// .to_vec();
    /// let data = ListOf128BitServiceSolicitationUUIDs::new(&uuids).into();
    /// assert!(DataTypeParseResult::from(&data).is_list_of_128bit_service_solicitation_uuids());
    ///
    /// let data: Vec<u8> = Vec::new();
    /// assert!(!DataTypeParseResult::from(&data).is_list_of_128bit_service_solicitation_uuids());
    /// ```
    pub fn is_list_of_128bit_service_solicitation_uuids(&self) -> bool {
        matches!(
            self,
            DataTypeParseResult::ListOf128BitServiceSolicitationUUIDsResult(_)
        )
    }

    /// Returns `true` if the result is [`DataTypeParseResult::ListOf16BitServiceSolicitationUUIDsResult`].
    ///
    /// # Examples
    ///
    /// ```
    /// use uuid::{uuid, Uuid};
    /// use ble_data_struct::data_types::{list_of_16bit_service_solicitation_uuids::ListOf16BitServiceSolicitationUUIDs, parser::DataTypeParseResult};
    ///
    /// let uuids: Vec<Uuid> = [
    ///     uuid!("00000001-0000-1000-8000-00805F9B34FB"),
    ///     uuid!("00000002-0000-1000-8000-00805F9B34FB"),
    /// ]
    /// .to_vec();
    /// let data = ListOf16BitServiceSolicitationUUIDs::new(&uuids).into();
    /// assert!(DataTypeParseResult::from(&data).is_list_of_16bit_service_solicitation_uuids());
    ///
    /// let data: Vec<u8> = Vec::new();
    /// assert!(!DataTypeParseResult::from(&data).is_list_of_16bit_service_solicitation_uuids());
    /// ```
    pub fn is_list_of_16bit_service_solicitation_uuids(&self) -> bool {
        matches!(
            self,
            DataTypeParseResult::ListOf16BitServiceSolicitationUUIDsResult(_)
        )
    }

    /// Returns `true` if the result is [`DataTypeParseResult::ListOf32BitServiceSolicitationUUIDsResult`].
    ///
    /// # Examples
    ///
    /// ```
    /// use uuid::{uuid, Uuid};
    /// use ble_data_struct::data_types::{list_of_32bit_service_solicitation_uuids::ListOf32BitServiceSolicitationUUIDs, parser::DataTypeParseResult};
    ///
    /// let uuids: Vec<Uuid> = [
    ///     uuid!("00000001-0000-1000-8000-00805F9B34FB"),
    ///     uuid!("00000002-0000-1000-8000-00805F9B34FB"),
    /// ]
    /// .to_vec();
    /// let data = ListOf32BitServiceSolicitationUUIDs::new(&uuids).into();
    /// assert!(DataTypeParseResult::from(&data).is_list_of_32bit_service_solicitation_uuids());
    ///
    /// let data: Vec<u8> = Vec::new();
    /// assert!(!DataTypeParseResult::from(&data).is_list_of_32bit_service_solicitation_uuids());
    /// ```
    pub fn is_list_of_32bit_service_solicitation_uuids(&self) -> bool {
        matches!(
            self,
            DataTypeParseResult::ListOf32BitServiceSolicitationUUIDsResult(_)
        )
    }

    /// Returns `true` if the result is [`DataTypeParseResult::ManufacturerSpecificDataResult`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{manufacturer_specific_data::ManufacturerSpecificData, parser::DataTypeParseResult};
    ///
    /// let company_identifier = 0x0ca8u16;
    /// let manufacturer_specific_data = [0x03u8].to_vec();
    /// let data =
    ///     ManufacturerSpecificData::new(company_identifier, &manufacturer_specific_data).into();
    /// assert!(DataTypeParseResult::from(&data).is_manufacturer_specific_data());
    ///
    /// let data: Vec<u8> = Vec::new();
    /// assert!(!DataTypeParseResult::from(&data).is_manufacturer_specific_data());
    /// ```
    pub fn is_manufacturer_specific_data(&self) -> bool {
        matches!(self, DataTypeParseResult::ManufacturerSpecificDataResult(_))
    }

    /// Returns `true` if the result is [`DataTypeParseResult::PeriodicAdvertisingResponseTimingInformationResult`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{periodic_advertising_response_timing_information::PeriodicAdvertisingResponseTimingInformation, parser::DataTypeParseResult};
    ///
    /// let rsp_aa: [u8; 4] = [1, 2, 3, 4];
    /// let num_subevents = 6u8;
    /// let subevent_interval = 7u8;
    /// let response_slot_delay = 8u8;
    /// let response_slot_spacing = 9u8;
    /// let data = PeriodicAdvertisingResponseTimingInformation::new(
    ///     &rsp_aa,
    ///     num_subevents,
    ///     subevent_interval,
    ///     response_slot_delay,
    ///     response_slot_spacing,
    /// )
    /// .into();
    /// assert!(DataTypeParseResult::from(&data).is_periodic_advertising_response_timing_information());
    ///
    /// let data: Vec<u8> = Vec::new();
    /// assert!(!DataTypeParseResult::from(&data).is_periodic_advertising_response_timing_information());
    /// ```
    pub fn is_periodic_advertising_response_timing_information(&self) -> bool {
        matches!(
            self,
            DataTypeParseResult::PeriodicAdvertisingResponseTimingInformationResult(_)
        )
    }

    /// Returns `true` if the result is [`DataTypeParseResult::PeripheralConnectionIntervalRangeResult`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{peripheral_connection_interval_range::PeripheralConnectionIntervalRange, parser::DataTypeParseResult};
    ///
    /// let minimum_value = 0x0006u16;
    /// let maximum_value = 0x0C80u16;
    /// let data = PeripheralConnectionIntervalRange::new(minimum_value, maximum_value).into();
    /// assert!(DataTypeParseResult::from(&data).is_peripheral_connection_interval_range());
    ///
    /// let data: Vec<u8> = Vec::new();
    /// assert!(!DataTypeParseResult::from(&data).is_peripheral_connection_interval_range());
    /// ```
    pub fn is_peripheral_connection_interval_range(&self) -> bool {
        matches!(
            self,
            DataTypeParseResult::PeripheralConnectionIntervalRangeResult(_)
        )
    }

    /// Returns `true` if the result is [`DataTypeParseResult::PublicTargetAddressResult`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{public_target_address::PublicTargetAddress, parser::DataTypeParseResult};
    ///
    /// let public_target_address: Vec<u64> = [
    ///     u64::from_le_bytes([
    ///         0x01u8, 0x02u8, 0x03u8, 0x04u8, 0x05u8, 0x06u8, 0x00u8, 0x00u8,
    ///     ]),
    ///     u64::from_le_bytes([
    ///         0x07u8, 0x08u8, 0x09u8, 0x0au8, 0x0bu8, 0x0cu8, 0x00u8, 0x00u8,
    ///     ]),
    /// ]
    /// .to_vec();
    /// let data = PublicTargetAddress::new(&public_target_address).into();
    /// assert!(DataTypeParseResult::from(&data).is_public_target_address());
    ///
    /// let data: Vec<u8> = Vec::new();
    /// assert!(!DataTypeParseResult::from(&data).is_public_target_address());
    /// ```
    pub fn is_public_target_address(&self) -> bool {
        matches!(self, DataTypeParseResult::PublicTargetAddressResult(_))
    }

    /// Returns `true` if the result is [`DataTypeParseResult::RandomTargetAddressResult`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{random_target_address::RandomTargetAddress, parser::DataTypeParseResult};
    ///
    /// let random_target_address: Vec<u64> = [
    ///     u64::from_le_bytes([
    ///         0x01u8, 0x02u8, 0x03u8, 0x04u8, 0x05u8, 0x06u8, 0x00u8, 0x00u8,
    ///     ]),
    ///     u64::from_le_bytes([
    ///         0x07u8, 0x08u8, 0x09u8, 0x0au8, 0x0bu8, 0x0cu8, 0x00u8, 0x00u8,
    ///     ]),
    /// ]
    /// .to_vec();
    /// let data = RandomTargetAddress::new(&random_target_address).into();
    /// assert!(DataTypeParseResult::from(&data).is_random_target_address());
    ///
    /// let data: Vec<u8> = Vec::new();
    /// assert!(!DataTypeParseResult::from(&data).is_random_target_address());
    /// ```
    pub fn is_random_target_address(&self) -> bool {
        matches!(self, DataTypeParseResult::RandomTargetAddressResult(_))
    }

    /// Returns `true` if the result is [`DataTypeParseResult::SecureSimplePairingHashC192Result`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{secure_simple_pairing_hash_c192::SecureSimplePairingHashC192, parser::DataTypeParseResult};
    ///
    /// let secure_simple_pairing_hash_c192 = 0x0102030405060708090a0b0c0d0e0f10u128;
    /// let data = SecureSimplePairingHashC192::new(secure_simple_pairing_hash_c192).into();
    /// assert!(DataTypeParseResult::from(&data).is_secure_simple_pairing_hash_c192());
    ///
    /// let data: Vec<u8> = Vec::new();
    /// assert!(!DataTypeParseResult::from(&data).is_secure_simple_pairing_hash_c192());
    /// ```
    pub fn is_secure_simple_pairing_hash_c192(&self) -> bool {
        matches!(
            self,
            DataTypeParseResult::SecureSimplePairingHashC192Result(_)
        )
    }

    /// Returns `true` if the result is [`DataTypeParseResult::SecureSimplePairingHashC256Result`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{secure_simple_pairing_hash_c256::SecureSimplePairingHashC256, parser::DataTypeParseResult};
    ///
    /// let secure_simple_pairing_hash_c256 = 0x0102030405060708090a0b0c0d0e0f10u128;
    /// let data = SecureSimplePairingHashC256::new(secure_simple_pairing_hash_c256).into();
    /// assert!(DataTypeParseResult::from(&data).is_secure_simple_pairing_hash_c256());
    ///
    /// let data: Vec<u8> = Vec::new();
    /// assert!(!DataTypeParseResult::from(&data).is_secure_simple_pairing_hash_c256());
    /// ```
    pub fn is_secure_simple_pairing_hash_c256(&self) -> bool {
        matches!(
            self,
            DataTypeParseResult::SecureSimplePairingHashC256Result(_)
        )
    }

    /// Returns `true` if the result is [`DataTypeParseResult::SecureSimplePairingRandomizerR192Result`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{secure_simple_pairing_randomizer_r192::SecureSimplePairingRandomizerR192, parser::DataTypeParseResult};
    ///
    /// let secure_simple_pairing_randomizer_r192 = 0x0102030405060708090a0b0c0d0e0f10u128;
    /// let data =
    ///     SecureSimplePairingRandomizerR192::new(secure_simple_pairing_randomizer_r192).into();
    /// assert!(DataTypeParseResult::from(&data).is_secure_simple_pairing_randomizer_r192());
    ///
    /// let data: Vec<u8> = Vec::new();
    /// assert!(!DataTypeParseResult::from(&data).is_secure_simple_pairing_randomizer_r192());
    /// ```
    pub fn is_secure_simple_pairing_randomizer_r192(&self) -> bool {
        matches!(
            self,
            DataTypeParseResult::SecureSimplePairingRandomizerR192Result(_)
        )
    }

    /// Returns `true` if the result is [`DataTypeParseResult::SecureSimplePairingRandomizerR256Result`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{secure_simple_pairing_randomizer_r256::SecureSimplePairingRandomizerR256, parser::DataTypeParseResult};
    ///
    /// let secure_simple_pairing_randomizer_r256 = 0x0102030405060708090a0b0c0d0e0f10u128;
    /// let data =
    ///     SecureSimplePairingRandomizerR256::new(secure_simple_pairing_randomizer_r256).into();
    /// assert!(DataTypeParseResult::from(&data).is_secure_simple_pairing_randomizer_r256());
    ///
    /// let data: Vec<u8> = Vec::new();
    /// assert!(!DataTypeParseResult::from(&data).is_secure_simple_pairing_randomizer_r256());
    /// ```
    pub fn is_secure_simple_pairing_randomizer_r256(&self) -> bool {
        matches!(
            self,
            DataTypeParseResult::SecureSimplePairingRandomizerR256Result(_)
        )
    }

    /// Returns `true` if the result is [`DataTypeParseResult::SecurityManagerOutOfBandResult`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{security_manager_oob::SecurityManagerOutOfBand, parser::DataTypeParseResult};
    ///
    /// let security_manager_oob = [true, false, false, false, false, false, false, false];
    /// let data = SecurityManagerOutOfBand::new(&security_manager_oob).into();
    /// assert!(DataTypeParseResult::from(&data).is_security_manager_oob());
    ///
    /// let data: Vec<u8> = Vec::new();
    /// assert!(!DataTypeParseResult::from(&data).is_security_manager_oob());
    /// ```
    pub fn is_security_manager_oob(&self) -> bool {
        matches!(self, DataTypeParseResult::SecurityManagerOutOfBandResult(_))
    }

    /// Returns `true` if the result is [`DataTypeParseResult::SecurityManagerTkValueResult`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{security_manager_tk_value::SecurityManagerTkValue, parser::DataTypeParseResult};
    ///
    /// let security_manager_tk_value = 0x0102030405060708090a0b0c0d0e0f10u128;
    /// let data = SecurityManagerTkValue::new(security_manager_tk_value).into();
    /// assert!(DataTypeParseResult::from(&data).is_security_manager_tk_value());
    ///
    /// let data: Vec<u8> = Vec::new();
    /// assert!(!DataTypeParseResult::from(&data).is_security_manager_tk_value());
    /// ```
    pub fn is_security_manager_tk_value(&self) -> bool {
        matches!(self, DataTypeParseResult::SecurityManagerTkValueResult(_))
    }

    /// Returns `true` if the result is [`DataTypeParseResult::ServiceData128BitUUIDResult`].
    ///
    /// # Examples
    ///
    /// ```
    /// use uuid::{uuid, Uuid};
    /// use ble_data_struct::data_types::{service_data_128bit_uuid::ServiceData128BitUUID, parser::DataTypeParseResult};
    ///
    /// let uuid = uuid!("04030201-0000-1000-8000-00805F9B34FB");
    /// let additional_service_data = [0x05u8].to_vec();
    /// let data = ServiceData128BitUUID::new(&uuid, &additional_service_data).into();
    /// assert!(DataTypeParseResult::from(&data).is_service_data_128bit_uuid());
    ///
    /// let data: Vec<u8> = Vec::new();
    /// assert!(!DataTypeParseResult::from(&data).is_service_data_128bit_uuid());
    /// ```
    pub fn is_service_data_128bit_uuid(&self) -> bool {
        matches!(self, DataTypeParseResult::ServiceData128BitUUIDResult(_))
    }

    /// Returns `true` if the result is [`DataTypeParseResult::ServiceData16BitUUIDResult`].
    ///
    /// # Examples
    ///
    /// ```
    /// use uuid::{uuid, Uuid};
    /// use ble_data_struct::data_types::{service_data_16bit_uuid::ServiceData16BitUUID, parser::DataTypeParseResult};
    ///
    /// let uuid = uuid!("00000201-0000-1000-8000-00805F9B34FB");
    /// let additional_service_data = [0x03u8].to_vec();
    /// let data = ServiceData16BitUUID::new(&uuid, &additional_service_data).into();
    /// assert!(DataTypeParseResult::from(&data).is_service_data_16bit_uuid());
    ///
    /// let data: Vec<u8> = Vec::new();
    /// assert!(!DataTypeParseResult::from(&data).is_service_data_16bit_uuid());
    /// ```
    pub fn is_service_data_16bit_uuid(&self) -> bool {
        matches!(self, DataTypeParseResult::ServiceData16BitUUIDResult(_))
    }

    /// Returns `true` if the result is [`DataTypeParseResult::ServiceData32BitUUIDResult`].
    ///
    /// # Examples
    ///
    /// ```
    /// use uuid::{uuid, Uuid};
    /// use ble_data_struct::data_types::{service_data_32bit_uuid::ServiceData32BitUUID, parser::DataTypeParseResult};
    ///
    /// let uuid = uuid!("04030201-0000-1000-8000-00805F9B34FB");
    /// let additional_service_data = [0x05u8].to_vec();
    /// let data = ServiceData32BitUUID::new(&uuid, &additional_service_data).into();
    /// assert!(DataTypeParseResult::from(&data).is_service_data_32bit_uuid());
    ///
    /// let data: Vec<u8> = Vec::new();
    /// assert!(!DataTypeParseResult::from(&data).is_service_data_32bit_uuid());
    /// ```
    pub fn is_service_data_32bit_uuid(&self) -> bool {
        matches!(self, DataTypeParseResult::ServiceData32BitUUIDResult(_))
    }

    /// Returns `true` if the result is [`DataTypeParseResult::ShortenedLocalNameResult`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{shortened_local_name::ShortenedLocalName, parser::DataTypeParseResult};
    ///
    /// let name = "shortened_local_name".to_string();
    /// let data = ShortenedLocalName::new(&name).into();
    /// assert!(DataTypeParseResult::from(&data).is_shortened_local_name());
    ///
    /// let data: Vec<u8> = Vec::new();
    /// assert!(!DataTypeParseResult::from(&data).is_shortened_local_name());
    /// ```
    pub fn is_shortened_local_name(&self) -> bool {
        matches!(self, DataTypeParseResult::ShortenedLocalNameResult(_))
    }

    /// Returns `true` if the result is [`DataTypeParseResult::TxPowerLevelResult`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{tx_power_level::TxPowerLevel, parser::DataTypeParseResult};
    ///
    /// let tx_power_level = -127;
    /// let data = TxPowerLevel::new(tx_power_level).into();
    /// assert!(DataTypeParseResult::from(&data).is_tx_power_level());
    ///
    /// let data: Vec<u8> = Vec::new();
    /// assert!(!DataTypeParseResult::from(&data).is_tx_power_level());
    /// ```
    pub fn is_tx_power_level(&self) -> bool {
        matches!(self, DataTypeParseResult::TxPowerLevelResult(_))
    }
}

impl From<&Vec<u8>> for DataTypeParseResult {
    /// Create [`DataTypeParseResult`] from `Vec<u8>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{advertising_interval::AdvertisingInterval, parser::DataTypeParseResult};
    ///
    /// let advertising_interval = 0x01;
    /// let data: Vec<u8> = AdvertisingInterval::new(advertising_interval).into();
    /// assert!(matches!(
    ///     DataTypeParseResult::from(&data),
    ///     DataTypeParseResult::AdvertisingIntervalResult(_)
    /// ));
    /// let data: Vec<u8> = Vec::new();
    /// assert!(matches!(
    ///     DataTypeParseResult::from(&data),
    ///     DataTypeParseResult::DataTypeParseErr(_)
    /// ));
    fn from(value: &Vec<u8>) -> Self {
        if let Some(data_type) = value.get(1) {
            if is_advertising_interval(data_type.to_owned()) {
                DataTypeParseResult::AdvertisingIntervalResult(AdvertisingInterval::try_from(value))
            } else if is_advertising_interval_long(data_type.to_owned()) {
                DataTypeParseResult::AdvertisingIntervalLongResult(
                    AdvertisingIntervalLong::try_from(value),
                )
            } else if is_appearance(data_type.to_owned()) {
                DataTypeParseResult::AppearanceResult(Appearance::try_from(value))
            } else if is_big_info(data_type.to_owned()) {
                DataTypeParseResult::BigInfoResult(BigInfo::try_from(value))
            } else if is_broadcast_code(data_type.to_owned()) {
                DataTypeParseResult::BroadcastCodeResult(BroadcastCode::try_from(value))
            } else if is_channel_map_update_indication(data_type.to_owned()) {
                DataTypeParseResult::ChannelMapUpdateIndicationResult(
                    ChannelMapUpdateIndication::try_from(value),
                )
            } else if is_class_of_device(data_type.to_owned()) {
                DataTypeParseResult::ClassOfDeviceResult(ClassOfDevice::try_from(value))
            } else if is_complete_list_of_128bit_service_uuids(data_type.to_owned()) {
                DataTypeParseResult::CompleteListOf128BitServiceUuidsResult(
                    CompleteListOf128BitServiceUuids::try_from(value),
                )
            } else if is_complete_list_of_16bit_service_uuids(data_type.to_owned()) {
                DataTypeParseResult::CompleteListOf16BitServiceUuidsResult(
                    CompleteListOf16BitServiceUuids::try_from(value),
                )
            } else if is_complete_list_of_32bit_service_uuids(data_type.to_owned()) {
                DataTypeParseResult::CompleteListOf32BitServiceUuidsResult(
                    CompleteListOf32BitServiceUuids::try_from(value),
                )
            } else if is_complete_local_name(data_type.to_owned()) {
                DataTypeParseResult::CompleteLocalNameResult(CompleteLocalName::try_from(value))
            } else if is_encrypted_data(data_type.to_owned()) {
                DataTypeParseResult::EncryptedDataResult(EncryptedData::try_from(value))
            } else if is_flags(data_type.to_owned()) {
                DataTypeParseResult::FlagsResult(Flags::try_from(value))
            } else if is_incomplete_list_of_128bit_service_uuids(data_type.to_owned()) {
                DataTypeParseResult::IncompleteListOf128BitServiceUuidsResult(
                    IncompleteListOf128BitServiceUuids::try_from(value),
                )
            } else if is_incomplete_list_of_16bit_service_uuids(data_type.to_owned()) {
                DataTypeParseResult::IncompleteListOf16BitServiceUuidsResult(
                    IncompleteListOf16BitServiceUuids::try_from(value),
                )
            } else if is_incomplete_list_of_32bit_service_uuids(data_type.to_owned()) {
                DataTypeParseResult::IncompleteListOf32BitServiceUuidsResult(
                    IncompleteListOf32BitServiceUuids::try_from(value),
                )
            } else if is_le_bluetooth_device_address(data_type.to_owned()) {
                DataTypeParseResult::LeBluetoothDeviceAddressResult(
                    LeBluetoothDeviceAddress::try_from(value),
                )
            } else if is_le_role(data_type.to_owned()) {
                DataTypeParseResult::LeRoleResult(LeRole::try_from(value))
            } else if is_le_secure_connections_confirmation_value(data_type.to_owned()) {
                DataTypeParseResult::LeSecureConnectionsConfirmationValueResult(
                    LeSecureConnectionsConfirmationValue::try_from(value),
                )
            } else if is_le_secure_connections_random_value(data_type.to_owned()) {
                DataTypeParseResult::LeSecureConnectionsRandomValueResult(
                    LeSecureConnectionsRandomValue::try_from(value),
                )
            } else if is_le_supported_features(data_type.to_owned()) {
                DataTypeParseResult::LeSupportedFeaturesResult(LeSupportedFeatures::try_from(value))
            } else if is_list_of_128bit_service_solicitation_uuids(data_type.to_owned()) {
                DataTypeParseResult::ListOf128BitServiceSolicitationUUIDsResult(
                    ListOf128BitServiceSolicitationUUIDs::try_from(value),
                )
            } else if is_list_of_16bit_service_solicitation_uuids(data_type.to_owned()) {
                DataTypeParseResult::ListOf16BitServiceSolicitationUUIDsResult(
                    ListOf16BitServiceSolicitationUUIDs::try_from(value),
                )
            } else if is_list_of_32bit_service_solicitation_uuids(data_type.to_owned()) {
                DataTypeParseResult::ListOf32BitServiceSolicitationUUIDsResult(
                    ListOf32BitServiceSolicitationUUIDs::try_from(value),
                )
            } else if is_manufacturer_specific_data(data_type.to_owned()) {
                DataTypeParseResult::ManufacturerSpecificDataResult(
                    ManufacturerSpecificData::try_from(value),
                )
            } else if is_periodic_advertising_response_timing_information(data_type.to_owned()) {
                DataTypeParseResult::PeriodicAdvertisingResponseTimingInformationResult(
                    PeriodicAdvertisingResponseTimingInformation::try_from(value),
                )
            } else if is_peripheral_connection_interval_range(data_type.to_owned()) {
                DataTypeParseResult::PeripheralConnectionIntervalRangeResult(
                    PeripheralConnectionIntervalRange::try_from(value),
                )
            } else if is_public_target_address(data_type.to_owned()) {
                DataTypeParseResult::PublicTargetAddressResult(PublicTargetAddress::try_from(value))
            } else if is_random_target_address(data_type.to_owned()) {
                DataTypeParseResult::RandomTargetAddressResult(RandomTargetAddress::try_from(value))
            } else if is_secure_simple_pairing_hash_c192(data_type.to_owned()) {
                DataTypeParseResult::SecureSimplePairingHashC192Result(
                    SecureSimplePairingHashC192::try_from(value),
                )
            } else if is_secure_simple_pairing_hash_c256(data_type.to_owned()) {
                DataTypeParseResult::SecureSimplePairingHashC256Result(
                    SecureSimplePairingHashC256::try_from(value),
                )
            } else if is_secure_simple_pairing_randomizer_r192(data_type.to_owned()) {
                DataTypeParseResult::SecureSimplePairingRandomizerR192Result(
                    SecureSimplePairingRandomizerR192::try_from(value),
                )
            } else if is_secure_simple_pairing_randomizer_r256(data_type.to_owned()) {
                DataTypeParseResult::SecureSimplePairingRandomizerR256Result(
                    SecureSimplePairingRandomizerR256::try_from(value),
                )
            } else if is_security_manager_oob(data_type.to_owned()) {
                DataTypeParseResult::SecurityManagerOutOfBandResult(
                    SecurityManagerOutOfBand::try_from(value),
                )
            } else if is_security_manager_tk_value(data_type.to_owned()) {
                DataTypeParseResult::SecurityManagerTkValueResult(SecurityManagerTkValue::try_from(
                    value,
                ))
            } else if is_service_data_128bit_uuid(data_type.to_owned()) {
                DataTypeParseResult::ServiceData128BitUUIDResult(ServiceData128BitUUID::try_from(
                    value,
                ))
            } else if is_service_data_16bit_uuid(data_type.to_owned()) {
                DataTypeParseResult::ServiceData16BitUUIDResult(ServiceData16BitUUID::try_from(
                    value,
                ))
            } else if is_service_data_32bit_uuid(data_type.to_owned()) {
                DataTypeParseResult::ServiceData32BitUUIDResult(ServiceData32BitUUID::try_from(
                    value,
                ))
            } else if is_shortened_local_name(data_type.to_owned()) {
                DataTypeParseResult::ShortenedLocalNameResult(ShortenedLocalName::try_from(value))
            } else if is_tx_power_level(data_type.to_owned()) {
                DataTypeParseResult::TxPowerLevelResult(TxPowerLevel::try_from(value))
            } else {
                DataTypeParseResult::DataTypeParseErr(
                    format!("Unknown data type :{}", data_type).to_string(),
                )
            }
        } else {
            DataTypeParseResult::DataTypeParseErr("Invalid data size".to_string())
        }
    }
}

/// Data types parse results.
pub struct DataTypeParseResults {
    /// Parse results.
    pub results: Vec<DataTypeParseResult>,
}

impl From<&Vec<Vec<u8>>> for DataTypeParseResults {
    /// Create [`DataTypeParseResults`] from `Vec<Vec<u8>>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{advertising_interval::AdvertisingInterval, parser::{DataTypeParseResult, DataTypeParseResults}};
    ///
    /// let mut vec: Vec<Vec<u8>> = Vec::new();
    /// let advertising_interval = 0x01;
    /// let data: Vec<u8> = AdvertisingInterval::new(advertising_interval).into();
    /// vec.push(data);
    /// vec.push(vec![]);
    ///
    /// let results = DataTypeParseResults::from(&vec);
    /// assert!(matches!(
    ///     results.results.get(0),
    ///     Some(DataTypeParseResult::AdvertisingIntervalResult(_))
    /// ));
    /// assert!(matches!(
    ///     results.results.get(1),
    ///     Some(DataTypeParseResult::DataTypeParseErr(_))
    /// ));
    /// assert!(matches!(results.results.get(2), None));
    /// ```
    fn from(value: &Vec<Vec<u8>>) -> Self {
        Self {
            results: value
                .iter()
                .map(|f| DataTypeParseResult::from(f))
                .collect::<Vec<DataTypeParseResult>>(),
        }
    }
}

impl From<&Vec<u8>> for DataTypeParseResults {
    /// Create [`DataTypeParseResults`] from `Vec<u8>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{advertising_interval::AdvertisingInterval, parser::{DataTypeParseResult, DataTypeParseResults}};
    ///
    /// let mut vec: Vec<Vec<u8>> = Vec::new();
    /// let advertising_interval = 0x01;
    /// vec.push(AdvertisingInterval::new(advertising_interval).into());
    /// vec.push(vec![100]);
    ///
    /// let results = DataTypeParseResults::from(&vec);
    /// assert!(matches!(
    ///     results.results.get(0),
    ///     Some(DataTypeParseResult::AdvertisingIntervalResult(_))
    /// ));
    /// assert!(matches!(
    ///     results.results.get(1),
    ///     Some(DataTypeParseResult::DataTypeParseErr(_))
    /// ));
    /// assert!(matches!(results.results.get(2), None));
    /// ```
    fn from(value: &Vec<u8>) -> Self {
        let mut vec = Vec::new();
        let mut index = 0;
        let len = value.len();
        while index < len {
            let mut inner: Vec<u8> = Vec::new();
            let size = value[index];
            inner.append(&mut value[index..index + 1 + size as usize].to_vec());
            vec.push(inner);

            index += 1;
            index += size as usize;
        }
        Self::from(&vec)
    }
}

#[cfg(test)]
mod tests {
    use uuid::{uuid, Uuid};

    use crate::data_types::{
        advertising_interval::AdvertisingInterval,
        advertising_interval_long::AdvertisingIntervalLong,
        appearance::Appearance,
        big_info::BigInfo,
        broadcast_code::BroadcastCode,
        channel_map_update_indication::ChannelMapUpdateIndication,
        class_of_device::ClassOfDevice,
        complete_list_of_128bit_service_uuids::CompleteListOf128BitServiceUuids,
        complete_list_of_16bit_service_uuids::CompleteListOf16BitServiceUuids,
        complete_list_of_32bit_service_uuids::CompleteListOf32BitServiceUuids,
        complete_local_name::CompleteLocalName,
        encrypted_data::EncryptedData,
        flags::Flags,
        incomplete_list_of_128bit_service_uuids::IncompleteListOf128BitServiceUuids,
        incomplete_list_of_16bit_service_uuids::IncompleteListOf16BitServiceUuids,
        incomplete_list_of_32bit_service_uuids::IncompleteListOf32BitServiceUuids,
        le_bluetooth_device_address::LeBluetoothDeviceAddress,
        le_role::{LeRole, ONLY_PERIPHERAL_ROLE_SUPPORTED},
        le_secure_connections_confirmation_value::LeSecureConnectionsConfirmationValue,
        le_secure_connections_random_value::LeSecureConnectionsRandomValue,
        le_supported_features::LeSupportedFeatures,
        list_of_128bit_service_solicitation_uuids::ListOf128BitServiceSolicitationUUIDs,
        list_of_16bit_service_solicitation_uuids::ListOf16BitServiceSolicitationUUIDs,
        list_of_32bit_service_solicitation_uuids::ListOf32BitServiceSolicitationUUIDs,
        manufacturer_specific_data::ManufacturerSpecificData,
        parser::DataTypeParseResult,
        periodic_advertising_response_timing_information::PeriodicAdvertisingResponseTimingInformation,
        peripheral_connection_interval_range::PeripheralConnectionIntervalRange,
        public_target_address::PublicTargetAddress,
        random_target_address::RandomTargetAddress,
        secure_simple_pairing_hash_c192::SecureSimplePairingHashC192,
        secure_simple_pairing_hash_c256::SecureSimplePairingHashC256,
        secure_simple_pairing_randomizer_r192::SecureSimplePairingRandomizerR192,
        secure_simple_pairing_randomizer_r256::SecureSimplePairingRandomizerR256,
        security_manager_oob::SecurityManagerOutOfBand,
        security_manager_tk_value::SecurityManagerTkValue,
        service_data_128bit_uuid::ServiceData128BitUUID,
        service_data_16bit_uuid::ServiceData16BitUUID,
        service_data_32bit_uuid::ServiceData32BitUUID,
        shortened_local_name::ShortenedLocalName,
        tx_power_level::TxPowerLevel,
    };

    use super::DataTypeParseResults;

    #[test]
    fn test_is_advertising_interval() {
        let advertising_interval = 0x01;
        let data: Vec<u8> = AdvertisingInterval::new(advertising_interval).into();
        assert!(DataTypeParseResult::from(&data).is_advertising_interval());

        let data: Vec<u8> = Vec::new();
        assert!(!DataTypeParseResult::from(&data).is_advertising_interval());
    }

    #[test]
    fn test_is_advertising_interval_long() {
        let advertising_interval_long: u32 = 0x01020304u32;
        let data: Vec<u8> = AdvertisingIntervalLong::new(true, advertising_interval_long).into();
        assert!(DataTypeParseResult::from(&data).is_advertising_interval_long());

        let data: Vec<u8> = Vec::new();
        assert!(!DataTypeParseResult::from(&data).is_advertising_interval_long());
    }

    #[test]
    fn test_is_appearance() {
        let appearance: u16 = 0x1444;
        let data: Vec<u8> = Appearance::new(appearance).into();
        assert!(DataTypeParseResult::from(&data).is_appearance());

        let data: Vec<u8> = Vec::new();
        assert!(!DataTypeParseResult::from(&data).is_appearance());
    }

    #[test]
    fn test_is_big_info() {
        let big_offset: u16 = 1;
        let big_offset_units: bool = true;
        let iso_interval: u16 = 2;
        let num_bis: u8 = 3;
        let nse: u8 = 4;
        let bn: u8 = 5;
        let sub_interval: u32 = 6;
        let pto: u8 = 7;
        let bis_spacing: u32 = 8;
        let irc: u8 = 9;
        let max_pdu: u8 = 10;
        let rfu: u8 = 11;
        let seed_access_address: u32 = 12;
        let sdu_interval: u32 = 13;
        let max_sdu: u16 = 14;
        let base_crc_init: u16 = 15;
        let ch_m: u64 = 16;
        let phy: u8 = 17;
        let bis_payload_count: u64 = 18;
        let framing: bool = false;
        let giv: Option<[u8; 8]> = None;
        let gskd: Option<[u8; 16]> = None;
        let data: Vec<u8> = BigInfo::new(
            big_offset,
            big_offset_units,
            iso_interval,
            num_bis,
            nse,
            bn,
            sub_interval,
            pto,
            bis_spacing,
            irc,
            max_pdu,
            rfu,
            seed_access_address,
            sdu_interval,
            max_sdu,
            base_crc_init,
            ch_m,
            phy,
            bis_payload_count,
            framing,
            giv,
            gskd,
        )
        .into();
        assert!(DataTypeParseResult::from(&data).is_big_info());

        let data: Vec<u8> = Vec::new();
        assert!(!DataTypeParseResult::from(&data).is_big_info());
    }

    #[test]
    fn test_is_broadcast_code() {
        let broadcast_code = [0x00u8; 4].to_vec();
        let data: Vec<u8> = BroadcastCode::new(&broadcast_code).into();
        assert!(DataTypeParseResult::from(&data).is_broadcast_code());

        let data: Vec<u8> = Vec::new();
        assert!(!DataTypeParseResult::from(&data).is_broadcast_code());
    }

    #[test]
    fn test_is_channel_map_update_indication() {
        let mut ch_m = [false; 37].to_vec();
        for i in 0..37 {
            ch_m[i] = true;
            let data = ChannelMapUpdateIndication::new(&ch_m, i as u16).into();
            assert!(DataTypeParseResult::from(&data).is_channel_map_update_indication());
            ch_m[i] = false;
        }

        let data: Vec<u8> = Vec::new();
        assert!(!DataTypeParseResult::from(&data).is_channel_map_update_indication());
    }

    #[test]
    fn test_is_class_of_device() {
        let major_service_classes = 0b10000000_00000000_00000000;
        let major_device_class = 0b00000000_00000001_00000000;
        let minor_device_class = 0b00000000_00000000_00000100;
        let class_of_device = major_service_classes | major_device_class | minor_device_class;
        let data = ClassOfDevice::new(class_of_device).into();
        assert!(DataTypeParseResult::from(&data).is_class_of_device());

        let data: Vec<u8> = Vec::new();
        assert!(!DataTypeParseResult::from(&data).is_class_of_device());
    }

    #[test]
    fn test_is_complete_list_of_128bit_service_uuids() {
        let uuids: Vec<Uuid> = [
            uuid!("00000001-0000-1000-8000-00805F9B34FB"),
            uuid!("00000002-0000-1000-8000-00805F9B34FB"),
        ]
        .to_vec();
        let data = CompleteListOf128BitServiceUuids::new(&uuids).into();
        assert!(DataTypeParseResult::from(&data).is_complete_list_of_128bit_service_uuids());

        let data: Vec<u8> = Vec::new();
        assert!(!DataTypeParseResult::from(&data).is_complete_list_of_128bit_service_uuids());
    }

    #[test]
    fn test_is_complete_list_of_16bit_service_uuids() {
        let uuids: Vec<Uuid> = [
            uuid!("00000001-0000-1000-8000-00805F9B34FB"),
            uuid!("00000002-0000-1000-8000-00805F9B34FB"),
        ]
        .to_vec();
        let data = CompleteListOf16BitServiceUuids::new(&uuids).into();
        assert!(DataTypeParseResult::from(&data).is_complete_list_of_16bit_service_uuids());

        let data: Vec<u8> = Vec::new();
        assert!(!DataTypeParseResult::from(&data).is_complete_list_of_16bit_service_uuids());
    }

    #[test]
    fn test_is_complete_list_of_32bit_service_uuids() {
        let uuids: Vec<Uuid> = [
            uuid!("00000001-0000-1000-8000-00805F9B34FB"),
            uuid!("00000002-0000-1000-8000-00805F9B34FB"),
        ]
        .to_vec();
        let data = CompleteListOf32BitServiceUuids::new(&uuids).into();
        assert!(DataTypeParseResult::from(&data).is_complete_list_of_32bit_service_uuids());

        let data: Vec<u8> = Vec::new();
        assert!(!DataTypeParseResult::from(&data).is_complete_list_of_32bit_service_uuids());
    }

    #[test]
    fn test_is_complete_local_name() {
        let name = "complete_local_name".to_string();
        let data = CompleteLocalName::new(&name).into();
        assert!(DataTypeParseResult::from(&data).is_complete_local_name());

        let data: Vec<u8> = Vec::new();
        assert!(!DataTypeParseResult::from(&data).is_complete_local_name());
    }

    #[test]
    fn test_is_encrypted_data() {
        let randomizer: [u8; 5] = [1, 2, 3, 4, 5];
        let payload = [6].to_vec();
        let mic: [u8; 4] = [7, 8, 9, 10];
        let data = EncryptedData::new(&randomizer, &payload, mic).into();
        assert!(DataTypeParseResult::from(&data).is_encrypted_data());

        let data: Vec<u8> = Vec::new();
        assert!(!DataTypeParseResult::from(&data).is_encrypted_data());
    }

    #[test]
    fn test_is_flags() {
        let flags = [true, false, false, false, false, false, false, false].to_vec();
        let data = Flags::new(&flags).into();
        assert!(DataTypeParseResult::from(&data).is_flags());

        let data: Vec<u8> = Vec::new();
        assert!(!DataTypeParseResult::from(&data).is_flags());
    }

    #[test]
    fn test_is_incomplete_list_of_128bit_service_uuids() {
        let uuids: Vec<Uuid> = [
            uuid!("00000001-0000-1000-8000-00805F9B34FB"),
            uuid!("00000002-0000-1000-8000-00805F9B34FB"),
        ]
        .to_vec();
        let data = IncompleteListOf128BitServiceUuids::new(&uuids).into();
        assert!(DataTypeParseResult::from(&data).is_incomplete_list_of_128bit_service_uuids());

        let data: Vec<u8> = Vec::new();
        assert!(!DataTypeParseResult::from(&data).is_incomplete_list_of_128bit_service_uuids());
    }

    #[test]
    fn test_is_incomplete_list_of_16bit_service_uuids() {
        let uuids: Vec<Uuid> = [
            uuid!("00000001-0000-1000-8000-00805F9B34FB"),
            uuid!("00000002-0000-1000-8000-00805F9B34FB"),
        ]
        .to_vec();
        let data = IncompleteListOf16BitServiceUuids::new(&uuids).into();
        assert!(DataTypeParseResult::from(&data).is_incomplete_list_of_16bit_service_uuids());

        let data: Vec<u8> = Vec::new();
        assert!(!DataTypeParseResult::from(&data).is_incomplete_list_of_16bit_service_uuids());
    }

    #[test]
    fn test_is_incomplete_list_of_32bit_service_uuids() {
        let uuids: Vec<Uuid> = [
            uuid!("00000001-0000-1000-8000-00805F9B34FB"),
            uuid!("00000002-0000-1000-8000-00805F9B34FB"),
        ]
        .to_vec();
        let data = IncompleteListOf32BitServiceUuids::new(&uuids).into();
        assert!(DataTypeParseResult::from(&data).is_incomplete_list_of_32bit_service_uuids());

        let data: Vec<u8> = Vec::new();
        assert!(!DataTypeParseResult::from(&data).is_incomplete_list_of_32bit_service_uuids());
    }

    #[test]
    fn test_is_le_bluetooth_device_address() {
        let le_bluetooth_device_address = 0x0000060504030201u64;
        let address_type = false;
        let data = LeBluetoothDeviceAddress::new(le_bluetooth_device_address, address_type).into();
        assert!(DataTypeParseResult::from(&data).is_le_bluetooth_device_address());

        let data: Vec<u8> = Vec::new();
        assert!(!DataTypeParseResult::from(&data).is_le_bluetooth_device_address());
    }

    #[test]
    fn test_is_le_role() {
        let le_role = ONLY_PERIPHERAL_ROLE_SUPPORTED;
        let data = LeRole::new(le_role).into();
        assert!(DataTypeParseResult::from(&data).is_le_role());

        let data: Vec<u8> = Vec::new();
        assert!(!DataTypeParseResult::from(&data).is_le_role());
    }

    #[test]
    fn test_is_le_secure_connections_confirmation_value() {
        let le_secure_connections_confirmation_value = 0x0102030405060708090a0b0c0d0e0f10u128;
        let data =
            LeSecureConnectionsConfirmationValue::new(le_secure_connections_confirmation_value)
                .into();
        assert!(DataTypeParseResult::from(&data).is_le_secure_connections_confirmation_value());

        let data: Vec<u8> = Vec::new();
        assert!(!DataTypeParseResult::from(&data).is_le_secure_connections_confirmation_value());
    }

    #[test]
    fn test_is_le_secure_connections_random_value() {
        let le_secure_connections_random_value = 0x0102030405060708090a0b0c0d0e0f10u128;

        let data = LeSecureConnectionsRandomValue::new(le_secure_connections_random_value).into();
        assert!(DataTypeParseResult::from(&data).is_le_secure_connections_random_value());

        let data: Vec<u8> = Vec::new();
        assert!(!DataTypeParseResult::from(&data).is_le_secure_connections_random_value());
    }

    #[test]
    fn test_is_le_supported_features() {
        let mut le_supported_features = [false; 48].to_vec();
        for i in 0..44 {
            le_supported_features[i] = true;
            let data = LeSupportedFeatures::new(&le_supported_features).into();
            assert!(DataTypeParseResult::from(&data).is_le_supported_features());
            le_supported_features[i] = false;
        }

        let data: Vec<u8> = Vec::new();
        assert!(!DataTypeParseResult::from(&data).is_le_supported_features());
    }

    #[test]
    fn test_is_list_of_128bit_service_solicitation_uuids() {
        let uuids: Vec<Uuid> = [
            uuid!("00000001-0000-1000-8000-00805F9B34FB"),
            uuid!("00000002-0000-1000-8000-00805F9B34FB"),
        ]
        .to_vec();
        let data = ListOf128BitServiceSolicitationUUIDs::new(&uuids).into();
        assert!(DataTypeParseResult::from(&data).is_list_of_128bit_service_solicitation_uuids());

        let data: Vec<u8> = Vec::new();
        assert!(!DataTypeParseResult::from(&data).is_list_of_128bit_service_solicitation_uuids());
    }

    #[test]
    fn test_is_list_of_16bit_service_solicitation_uuids() {
        let uuids: Vec<Uuid> = [
            uuid!("00000001-0000-1000-8000-00805F9B34FB"),
            uuid!("00000002-0000-1000-8000-00805F9B34FB"),
        ]
        .to_vec();
        let data = ListOf16BitServiceSolicitationUUIDs::new(&uuids).into();
        assert!(DataTypeParseResult::from(&data).is_list_of_16bit_service_solicitation_uuids());

        let data: Vec<u8> = Vec::new();
        assert!(!DataTypeParseResult::from(&data).is_list_of_16bit_service_solicitation_uuids());
    }

    #[test]
    fn test_is_list_of_32bit_service_solicitation_uuids() {
        let uuids: Vec<Uuid> = [
            uuid!("00000001-0000-1000-8000-00805F9B34FB"),
            uuid!("00000002-0000-1000-8000-00805F9B34FB"),
        ]
        .to_vec();
        let data = ListOf32BitServiceSolicitationUUIDs::new(&uuids).into();
        assert!(DataTypeParseResult::from(&data).is_list_of_32bit_service_solicitation_uuids());

        let data: Vec<u8> = Vec::new();
        assert!(!DataTypeParseResult::from(&data).is_list_of_32bit_service_solicitation_uuids());
    }

    #[test]
    fn test_is_manufacturer_specific_data() {
        let company_identifier = 0x0ca8u16;
        let manufacturer_specific_data = [0x03u8].to_vec();
        let data =
            ManufacturerSpecificData::new(company_identifier, &manufacturer_specific_data).into();
        assert!(DataTypeParseResult::from(&data).is_manufacturer_specific_data());

        let data: Vec<u8> = Vec::new();
        assert!(!DataTypeParseResult::from(&data).is_manufacturer_specific_data());
    }

    #[test]
    fn test_is_periodic_advertising_response_timing_information() {
        let rsp_aa: [u8; 4] = [1, 2, 3, 4];
        let num_subevents = 6u8;
        let subevent_interval = 7u8;
        let response_slot_delay = 8u8;
        let response_slot_spacing = 9u8;
        let data = PeriodicAdvertisingResponseTimingInformation::new(
            &rsp_aa,
            num_subevents,
            subevent_interval,
            response_slot_delay,
            response_slot_spacing,
        )
        .into();
        assert!(
            DataTypeParseResult::from(&data).is_periodic_advertising_response_timing_information()
        );

        let data: Vec<u8> = Vec::new();
        assert!(
            !DataTypeParseResult::from(&data).is_periodic_advertising_response_timing_information()
        );
    }

    #[test]
    fn test_is_peripheral_connection_interval_range() {
        let minimum_value = 0x0006u16;
        let maximum_value = 0x0C80u16;
        let data = PeripheralConnectionIntervalRange::new(minimum_value, maximum_value).into();
        assert!(DataTypeParseResult::from(&data).is_peripheral_connection_interval_range());

        let data: Vec<u8> = Vec::new();
        assert!(!DataTypeParseResult::from(&data).is_peripheral_connection_interval_range());
    }

    #[test]
    fn test_is_public_target_address() {
        let public_target_address: Vec<u64> = [
            u64::from_le_bytes([
                0x01u8, 0x02u8, 0x03u8, 0x04u8, 0x05u8, 0x06u8, 0x00u8, 0x00u8,
            ]),
            u64::from_le_bytes([
                0x07u8, 0x08u8, 0x09u8, 0x0au8, 0x0bu8, 0x0cu8, 0x00u8, 0x00u8,
            ]),
        ]
        .to_vec();
        let data = PublicTargetAddress::new(&public_target_address).into();
        assert!(DataTypeParseResult::from(&data).is_public_target_address());

        let data: Vec<u8> = Vec::new();
        assert!(!DataTypeParseResult::from(&data).is_public_target_address());
    }

    #[test]
    fn test_is_random_target_address() {
        let random_target_address: Vec<u64> = [
            u64::from_le_bytes([
                0x01u8, 0x02u8, 0x03u8, 0x04u8, 0x05u8, 0x06u8, 0x00u8, 0x00u8,
            ]),
            u64::from_le_bytes([
                0x07u8, 0x08u8, 0x09u8, 0x0au8, 0x0bu8, 0x0cu8, 0x00u8, 0x00u8,
            ]),
        ]
        .to_vec();
        let data = RandomTargetAddress::new(&random_target_address).into();
        assert!(DataTypeParseResult::from(&data).is_random_target_address());

        let data: Vec<u8> = Vec::new();
        assert!(!DataTypeParseResult::from(&data).is_random_target_address());
    }

    #[test]
    fn test_is_secure_simple_pairing_hash_c192() {
        let secure_simple_pairing_hash_c192 = 0x0102030405060708090a0b0c0d0e0f10u128;
        let data = SecureSimplePairingHashC192::new(secure_simple_pairing_hash_c192).into();
        assert!(DataTypeParseResult::from(&data).is_secure_simple_pairing_hash_c192());

        let data: Vec<u8> = Vec::new();
        assert!(!DataTypeParseResult::from(&data).is_secure_simple_pairing_hash_c192());
    }

    #[test]
    fn test_is_secure_simple_pairing_hash_c256() {
        let secure_simple_pairing_hash_c256 = 0x0102030405060708090a0b0c0d0e0f10u128;
        let data = SecureSimplePairingHashC256::new(secure_simple_pairing_hash_c256).into();
        assert!(DataTypeParseResult::from(&data).is_secure_simple_pairing_hash_c256());

        let data: Vec<u8> = Vec::new();
        assert!(!DataTypeParseResult::from(&data).is_secure_simple_pairing_hash_c256());
    }

    #[test]
    fn test_is_secure_simple_pairing_randomizer_r192() {
        let secure_simple_pairing_randomizer_r192 = 0x0102030405060708090a0b0c0d0e0f10u128;
        let data =
            SecureSimplePairingRandomizerR192::new(secure_simple_pairing_randomizer_r192).into();
        assert!(DataTypeParseResult::from(&data).is_secure_simple_pairing_randomizer_r192());

        let data: Vec<u8> = Vec::new();
        assert!(!DataTypeParseResult::from(&data).is_secure_simple_pairing_randomizer_r192());
    }

    #[test]
    fn test_is_secure_simple_pairing_randomizer_r256() {
        let secure_simple_pairing_randomizer_r256 = 0x0102030405060708090a0b0c0d0e0f10u128;
        let data =
            SecureSimplePairingRandomizerR256::new(secure_simple_pairing_randomizer_r256).into();
        assert!(DataTypeParseResult::from(&data).is_secure_simple_pairing_randomizer_r256());

        let data: Vec<u8> = Vec::new();
        assert!(!DataTypeParseResult::from(&data).is_secure_simple_pairing_randomizer_r256());
    }

    #[test]
    fn test_is_security_manager_oob() {
        let security_manager_oob = [true, false, false, false, false, false, false, false];
        let data = SecurityManagerOutOfBand::new(&security_manager_oob).into();
        assert!(DataTypeParseResult::from(&data).is_security_manager_oob());

        let data: Vec<u8> = Vec::new();
        assert!(!DataTypeParseResult::from(&data).is_security_manager_oob());
    }

    #[test]
    fn test_is_security_manager_tk_value() {
        let security_manager_tk_value = 0x0102030405060708090a0b0c0d0e0f10u128;
        let data = SecurityManagerTkValue::new(security_manager_tk_value).into();
        assert!(DataTypeParseResult::from(&data).is_security_manager_tk_value());

        let data: Vec<u8> = Vec::new();
        assert!(!DataTypeParseResult::from(&data).is_security_manager_tk_value());
    }

    #[test]
    fn test_is_service_data_128bit_uuid() {
        let uuid = uuid!("04030201-0000-1000-8000-00805F9B34FB");
        let additional_service_data = [0x05u8].to_vec();
        let data = ServiceData128BitUUID::new(&uuid, &additional_service_data).into();
        assert!(DataTypeParseResult::from(&data).is_service_data_128bit_uuid());

        let data: Vec<u8> = Vec::new();
        assert!(!DataTypeParseResult::from(&data).is_service_data_128bit_uuid());
    }

    #[test]
    fn test_is_service_data_16bit_uuid() {
        let uuid = uuid!("00000201-0000-1000-8000-00805F9B34FB");
        let additional_service_data = [0x03u8].to_vec();
        let data = ServiceData16BitUUID::new(&uuid, &additional_service_data).into();
        assert!(DataTypeParseResult::from(&data).is_service_data_16bit_uuid());

        let data: Vec<u8> = Vec::new();
        assert!(!DataTypeParseResult::from(&data).is_service_data_16bit_uuid());
    }

    #[test]
    fn test_is_service_data_32bit_uuid() {
        let uuid = uuid!("04030201-0000-1000-8000-00805F9B34FB");
        let additional_service_data = [0x05u8].to_vec();
        let data = ServiceData32BitUUID::new(&uuid, &additional_service_data).into();
        assert!(DataTypeParseResult::from(&data).is_service_data_32bit_uuid());

        let data: Vec<u8> = Vec::new();
        assert!(!DataTypeParseResult::from(&data).is_service_data_32bit_uuid());
    }

    #[test]
    fn test_is_shortened_local_name() {
        let name = "shortened_local_name".to_string();
        let data = ShortenedLocalName::new(&name).into();
        assert!(DataTypeParseResult::from(&data).is_shortened_local_name());

        let data: Vec<u8> = Vec::new();
        assert!(!DataTypeParseResult::from(&data).is_shortened_local_name());
    }

    #[test]
    fn test_is_tx_power_level() {
        let tx_power_level = -127;
        let data = TxPowerLevel::new(tx_power_level).into();
        assert!(DataTypeParseResult::from(&data).is_tx_power_level());

        let data: Vec<u8> = Vec::new();
        assert!(!DataTypeParseResult::from(&data).is_tx_power_level());
    }

    #[test]
    fn test_result_from_vec() {
        let advertising_interval = 0x01;
        let data: Vec<u8> = AdvertisingInterval::new(advertising_interval).into();
        assert!(matches!(
            DataTypeParseResult::from(&data),
            DataTypeParseResult::AdvertisingIntervalResult(_)
        ));

        let data: Vec<u8> = Vec::new();
        assert!(matches!(
            DataTypeParseResult::from(&data),
            DataTypeParseResult::DataTypeParseErr(_)
        ));
    }

    #[test]
    fn test_results_from_vec_vec() {
        let mut vec: Vec<Vec<u8>> = Vec::new();
        let advertising_interval = 0x01;
        let data: Vec<u8> = AdvertisingInterval::new(advertising_interval).into();
        vec.push(data);
        vec.push(vec![]);

        let results = DataTypeParseResults::from(&vec);
        assert!(matches!(
            results.results.get(0),
            Some(DataTypeParseResult::AdvertisingIntervalResult(_))
        ));
        assert!(matches!(
            results.results.get(1),
            Some(DataTypeParseResult::DataTypeParseErr(_))
        ));
        assert!(matches!(results.results.get(2), None));
    }

    #[test]
    fn test_results_from_vec() {
        let mut vec: Vec<Vec<u8>> = Vec::new();
        let advertising_interval = 0x01;
        vec.push(AdvertisingInterval::new(advertising_interval).into());
        vec.push(vec![100]);

        let results = DataTypeParseResults::from(&vec);
        assert!(matches!(
            results.results.get(0),
            Some(DataTypeParseResult::AdvertisingIntervalResult(_))
        ));
        assert!(matches!(
            results.results.get(1),
            Some(DataTypeParseResult::DataTypeParseErr(_))
        ));
        assert!(matches!(results.results.get(2), None));
    }
}
