//! LE Supported Features (Data Type Value: 0xff) module.

use crate::data_types::data_type::DataType;

/// LE Supported Features.
pub struct LeSupportedFeatures {
    /// data length
    pub length: u8,

    /// LE Supported Features
    pub le_supported_features: Vec<bool>,
}

impl LeSupportedFeatures {
    /// Create [LeSupportedFeatures] from `LE Supported Features`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::le_supported_features::LeSupportedFeatures;
    ///
    /// let mut le_supported_features = [false; 48].to_vec();
    /// for i in 0..44 {
    ///     le_supported_features[i] = true;
    ///     let result = LeSupportedFeatures::new(&le_supported_features);
    ///     assert_eq!(7, result.length);
    ///     assert_eq!(le_supported_features, result.le_supported_features);
    ///     le_supported_features[i] = false;
    /// }
    /// ```
    pub fn new(le_supported_features: &Vec<bool>) -> Self {
        Self {
            length: 1 + le_supported_features.len() as u8 / 8,
            le_supported_features: le_supported_features.clone(),
        }
    }

    /// Create [LeSupportedFeatures] from `Vec<u8>` with offset.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::{BASE_UUID, data_types::{le_supported_features::LeSupportedFeatures, data_type::DataType}};
    ///
    /// let mut le_supported_features = [0u8; 6].to_vec();
    ///
    /// for i in 0..44 {
    ///     le_supported_features[i / 8] = 0b1 << (i % 8);
    ///
    ///     let length = le_supported_features.len() as u8 + 1;
    ///     let mut data: Vec<u8> = Vec::new();
    ///     data.push(length);
    ///     data.push(LeSupportedFeatures::data_type());
    ///     data.append(&mut le_supported_features.clone());
    ///
    ///     let result = LeSupportedFeatures::from_with_offset(&data, 0);
    ///     assert_eq!(length, result.length);
    ///     let bool_vec: Vec<bool> = le_supported_features
    ///         .clone()
    ///         .iter()
    ///         .flat_map(|x| {
    ///             let mut data: Vec<bool> = Vec::new();
    ///             data.push((x & 0b0000_0001) != 0);
    ///             data.push((x & 0b0000_0010) != 0);
    ///             data.push((x & 0b0000_0100) != 0);
    ///             data.push((x & 0b0000_1000) != 0);
    ///             data.push((x & 0b0001_0000) != 0);
    ///             data.push((x & 0b0010_0000) != 0);
    ///             data.push((x & 0b0100_0000) != 0);
    ///             data.push((x & 0b1000_0000) != 0);
    ///             data
    ///         })
    ///         .collect();
    ///
    ///     assert_eq!(bool_vec, result.le_supported_features);
    ///
    ///     le_supported_features[i / 8] = 0u8;
    /// }
    ///
    /// let mut le_supported_features = [0u8; 6].to_vec();
    ///
    /// for i in 0..44 {
    ///     le_supported_features[i / 8] = 0b1 << (i % 8);
    ///
    ///     let length = le_supported_features.len() as u8 + 1;
    ///     let mut data: Vec<u8> = Vec::new();
    ///     data.push(0);
    ///     data.push(length);
    ///     data.push(LeSupportedFeatures::data_type());
    ///     data.append(&mut le_supported_features.clone());
    ///
    ///     let result = LeSupportedFeatures::from_with_offset(&data, 1);
    ///     assert_eq!(length, result.length);
    ///     let bool_vec: Vec<bool> = le_supported_features
    ///         .clone()
    ///         .iter()
    ///         .flat_map(|x| {
    ///             let mut data: Vec<bool> = Vec::new();
    ///             data.push((x & 0b0000_0001) != 0);
    ///             data.push((x & 0b0000_0010) != 0);
    ///             data.push((x & 0b0000_0100) != 0);
    ///             data.push((x & 0b0000_1000) != 0);
    ///             data.push((x & 0b0001_0000) != 0);
    ///             data.push((x & 0b0010_0000) != 0);
    ///             data.push((x & 0b0100_0000) != 0);
    ///             data.push((x & 0b1000_0000) != 0);
    ///             data
    ///         })
    ///         .collect();
    ///
    ///     assert_eq!(bool_vec, result.le_supported_features);
    ///
    ///     le_supported_features[i / 8] = 0u8;
    /// }
    /// ```
    pub fn from_with_offset(data: &Vec<u8>, offset: usize) -> Self {
        let data = data[offset..].to_vec();
        let length = data[0];
        let le_supported_features: Vec<bool> = data[2..]
            .iter()
            .flat_map(|x| {
                let mut data: Vec<bool> = Vec::new();
                data.push(x & 0b0000_0001 != 0);
                data.push(x & 0b0000_0010 != 0);
                data.push(x & 0b0000_0100 != 0);
                data.push(x & 0b0000_1000 != 0);
                data.push(x & 0b0001_0000 != 0);
                data.push(x & 0b0010_0000 != 0);
                data.push(x & 0b0100_0000 != 0);
                data.push(x & 0b1000_0000 != 0);
                data
            })
            .collect();
        Self {
            length,
            le_supported_features: le_supported_features.to_vec(),
        }
    }

    /// check LE Encryption Feature.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::le_supported_features::LeSupportedFeatures;
    ///
    /// let mut le_supported_features = [false; 48].to_vec();
    /// le_supported_features[0] = true;
    /// let result = LeSupportedFeatures::new(&le_supported_features);
    /// assert!(result.is_le_encryption_supported());
    /// ```
    pub fn is_le_encryption_supported(&self) -> bool {
        *self.le_supported_features.get(0).unwrap_or(&false)
    }

    /// check Connection Parameters Request Procedure Feature.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::le_supported_features::LeSupportedFeatures;
    ///
    /// let mut le_supported_features = [false; 48].to_vec();
    /// le_supported_features[1] = true;
    /// let result = LeSupportedFeatures::new(&le_supported_features);
    /// assert!(result.is_connection_parameters_request_procedure_supported());
    /// ```
    pub fn is_connection_parameters_request_procedure_supported(&self) -> bool {
        *self.le_supported_features.get(1).unwrap_or(&false)
    }

    /// check Extended Reject Indication Feature.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::le_supported_features::LeSupportedFeatures;
    ///
    /// let mut le_supported_features = [false; 48].to_vec();
    /// le_supported_features[2] = true;
    /// let result = LeSupportedFeatures::new(&le_supported_features);
    /// assert!(result.is_extended_reject_indication_supported());
    /// ```
    pub fn is_extended_reject_indication_supported(&self) -> bool {
        *self.le_supported_features.get(2).unwrap_or(&false)
    }

    /// check Peripheral-initiated Features Exchange Feature.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::le_supported_features::LeSupportedFeatures;
    ///
    /// let mut le_supported_features = [false; 48].to_vec();
    /// le_supported_features[3] = true;
    /// let result = LeSupportedFeatures::new(&le_supported_features);
    /// assert!(result.is_peripheral_initiated_features_exchange_supported());
    /// ```
    pub fn is_peripheral_initiated_features_exchange_supported(&self) -> bool {
        *self.le_supported_features.get(3).unwrap_or(&false)
    }

    /// check LE Ping Feature.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::le_supported_features::LeSupportedFeatures;
    ///
    /// let mut le_supported_features = [false; 48].to_vec();
    /// le_supported_features[4] = true;
    /// let result = LeSupportedFeatures::new(&le_supported_features);
    /// assert!(result.is_le_ping_supported());
    /// ```
    pub fn is_le_ping_supported(&self) -> bool {
        *self.le_supported_features.get(4).unwrap_or(&false)
    }

    /// check LE Data Packet Length Extension Feature.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::le_supported_features::LeSupportedFeatures;
    ///
    /// let mut le_supported_features = [false; 48].to_vec();
    /// le_supported_features[5] = true;
    /// let result = LeSupportedFeatures::new(&le_supported_features);
    /// assert!(result.is_le_data_packet_length_extension_supported());
    /// ```
    pub fn is_le_data_packet_length_extension_supported(&self) -> bool {
        *self.le_supported_features.get(5).unwrap_or(&false)
    }

    /// check LL Privacy Feature.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::le_supported_features::LeSupportedFeatures;
    ///
    /// let mut le_supported_features = [false; 48].to_vec();
    /// le_supported_features[6] = true;
    /// let result = LeSupportedFeatures::new(&le_supported_features);
    /// assert!(result.is_ll_privacy_supported());
    /// ```
    pub fn is_ll_privacy_supported(&self) -> bool {
        *self.le_supported_features.get(6).unwrap_or(&false)
    }

    /// check Extended Scanning Filter Policies Feature.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::le_supported_features::LeSupportedFeatures;
    ///
    /// let mut le_supported_features = [false; 48].to_vec();
    /// le_supported_features[7] = true;
    /// let result = LeSupportedFeatures::new(&le_supported_features);
    /// assert!(result.is_extended_scanning_filter_policies_supported());
    /// ```
    pub fn is_extended_scanning_filter_policies_supported(&self) -> bool {
        *self.le_supported_features.get(7).unwrap_or(&false)
    }

    /// check LE 2M PHY Feature.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::le_supported_features::LeSupportedFeatures;
    ///
    /// let mut le_supported_features = [false; 48].to_vec();
    /// le_supported_features[8] = true;
    /// let result = LeSupportedFeatures::new(&le_supported_features);
    /// assert!(result.is_le_2m_phy_supported());
    /// ```
    pub fn is_le_2m_phy_supported(&self) -> bool {
        *self.le_supported_features.get(8).unwrap_or(&false)
    }

    /// check Stable Modulation Index - Transmitter Feature.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::le_supported_features::LeSupportedFeatures;
    ///
    /// let mut le_supported_features = [false; 48].to_vec();
    /// le_supported_features[9] = true;
    /// let result = LeSupportedFeatures::new(&le_supported_features);
    /// assert!(result.is_stable_modulation_index_transmitter_supported());
    /// ```
    pub fn is_stable_modulation_index_transmitter_supported(&self) -> bool {
        *self.le_supported_features.get(9).unwrap_or(&false)
    }

    /// check Stable Modulation Index - Receiver Feature.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::le_supported_features::LeSupportedFeatures;
    ///
    /// let mut le_supported_features = [false; 48].to_vec();
    /// le_supported_features[10] = true;
    /// let result = LeSupportedFeatures::new(&le_supported_features);
    /// assert!(result.is_stable_modulation_index_receiver_supported());
    /// ```
    pub fn is_stable_modulation_index_receiver_supported(&self) -> bool {
        *self.le_supported_features.get(10).unwrap_or(&false)
    }

    /// check LE Coded PHY Feature.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::le_supported_features::LeSupportedFeatures;
    ///
    /// let mut le_supported_features = [false; 48].to_vec();
    /// le_supported_features[11] = true;
    /// let result = LeSupportedFeatures::new(&le_supported_features);
    /// assert!(result.is_le_coded_phy_supported());
    /// ```
    pub fn is_le_coded_phy_supported(&self) -> bool {
        *self.le_supported_features.get(11).unwrap_or(&false)
    }

    /// check LE Extended Advertising Feature.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::le_supported_features::LeSupportedFeatures;
    ///
    /// let mut le_supported_features = [false; 48].to_vec();
    /// le_supported_features[12] = true;
    /// let result = LeSupportedFeatures::new(&le_supported_features);
    /// assert!(result.is_le_extended_advertising_supported());
    /// ```
    pub fn is_le_extended_advertising_supported(&self) -> bool {
        *self.le_supported_features.get(12).unwrap_or(&false)
    }

    /// check LE Periodic Advertising Feature.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::le_supported_features::LeSupportedFeatures;
    ///
    /// let mut le_supported_features = [false; 48].to_vec();
    /// le_supported_features[13] = true;
    /// let result = LeSupportedFeatures::new(&le_supported_features);
    /// assert!(result.is_le_periodic_advertising_supported());
    /// ```
    pub fn is_le_periodic_advertising_supported(&self) -> bool {
        *self.le_supported_features.get(13).unwrap_or(&false)
    }

    /// check Channel Selection Algorithm #2 Feature.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::le_supported_features::LeSupportedFeatures;
    ///
    /// let mut le_supported_features = [false; 48].to_vec();
    /// le_supported_features[14] = true;
    /// let result = LeSupportedFeatures::new(&le_supported_features);
    /// assert!(result.is_channel_selection_algorithm2_supported());
    /// ```
    pub fn is_channel_selection_algorithm2_supported(&self) -> bool {
        *self.le_supported_features.get(14).unwrap_or(&false)
    }

    /// check LE Power Class 1 Feature.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::le_supported_features::LeSupportedFeatures;
    ///
    /// let mut le_supported_features = [false; 48].to_vec();
    /// le_supported_features[15] = true;
    /// let result = LeSupportedFeatures::new(&le_supported_features);
    /// assert!(result.is_le_power_class1_supported());
    /// ```
    pub fn is_le_power_class1_supported(&self) -> bool {
        *self.le_supported_features.get(15).unwrap_or(&false)
    }

    /// check Minimum Number of Used Channels procedure Feature.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::le_supported_features::LeSupportedFeatures;
    ///
    /// let mut le_supported_features = [false; 48].to_vec();
    /// le_supported_features[16] = true;
    /// let result = LeSupportedFeatures::new(&le_supported_features);
    /// assert!(result.is_minimum_number_of_used_channels_procedure_supported());
    /// ```
    pub fn is_minimum_number_of_used_channels_procedure_supported(&self) -> bool {
        *self.le_supported_features.get(16).unwrap_or(&false)
    }

    /// check Connection CTE Request Feature.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::le_supported_features::LeSupportedFeatures;
    ///
    /// let mut le_supported_features = [false; 48].to_vec();
    /// le_supported_features[17] = true;
    /// let result = LeSupportedFeatures::new(&le_supported_features);
    /// assert!(result.is_connection_cte_request_supported());
    /// ```
    pub fn is_connection_cte_request_supported(&self) -> bool {
        *self.le_supported_features.get(17).unwrap_or(&false)
    }

    /// check Connection CTE Response Feature.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::le_supported_features::LeSupportedFeatures;
    ///
    /// let mut le_supported_features = [false; 48].to_vec();
    /// le_supported_features[18] = true;
    /// let result = LeSupportedFeatures::new(&le_supported_features);
    /// assert!(result.is_connection_cte_response_supported());
    /// ```
    pub fn is_connection_cte_response_supported(&self) -> bool {
        *self.le_supported_features.get(18).unwrap_or(&false)
    }

    /// check Connectionless CTE Transmitter Feature.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::le_supported_features::LeSupportedFeatures;
    ///
    /// let mut le_supported_features = [false; 48].to_vec();
    /// le_supported_features[19] = true;
    /// let result = LeSupportedFeatures::new(&le_supported_features);
    /// assert!(result.is_connectionless_cte_transmitter_supported());
    /// ```
    pub fn is_connectionless_cte_transmitter_supported(&self) -> bool {
        *self.le_supported_features.get(19).unwrap_or(&false)
    }

    /// check Connectionless CTE Receiver Feature.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::le_supported_features::LeSupportedFeatures;
    ///
    /// let mut le_supported_features = [false; 48].to_vec();
    /// le_supported_features[20] = true;
    /// let result = LeSupportedFeatures::new(&le_supported_features);
    /// assert!(result.is_connectionless_cte_receiver_supported());
    /// ```
    pub fn is_connectionless_cte_receiver_supported(&self) -> bool {
        *self.le_supported_features.get(20).unwrap_or(&false)
    }

    /// check Antenna Switching During CTE Transmission (AoD) Feature.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::le_supported_features::LeSupportedFeatures;
    ///
    /// let mut le_supported_features = [false; 48].to_vec();
    /// le_supported_features[21] = true;
    /// let result = LeSupportedFeatures::new(&le_supported_features);
    /// assert!(result.is_antenna_switching_during_cte_transmission_aod_supported());
    /// ```
    pub fn is_antenna_switching_during_cte_transmission_aod_supported(&self) -> bool {
        *self.le_supported_features.get(21).unwrap_or(&false)
    }

    /// check Antenna Switching During CTE Reception (AoA) Feature.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::le_supported_features::LeSupportedFeatures;
    ///
    /// let mut le_supported_features = [false; 48].to_vec();
    /// le_supported_features[22] = true;
    /// let result = LeSupportedFeatures::new(&le_supported_features);
    /// assert!(result.is_antenna_switching_during_cte_reception_aoa_supported());
    /// ```
    pub fn is_antenna_switching_during_cte_reception_aoa_supported(&self) -> bool {
        *self.le_supported_features.get(22).unwrap_or(&false)
    }

    /// check Receiving Constant Tone Extensions Feature.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::le_supported_features::LeSupportedFeatures;
    ///
    /// let mut le_supported_features = [false; 48].to_vec();
    /// le_supported_features[23] = true;
    /// let result = LeSupportedFeatures::new(&le_supported_features);
    /// assert!(result.is_receiving_constant_tone_extensions_supported());
    /// ```
    pub fn is_receiving_constant_tone_extensions_supported(&self) -> bool {
        *self.le_supported_features.get(23).unwrap_or(&false)
    }

    /// check Periodic Advertising Sync Transfer - Sender Feature.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::le_supported_features::LeSupportedFeatures;
    ///
    /// let mut le_supported_features = [false; 48].to_vec();
    /// le_supported_features[24] = true;
    /// let result = LeSupportedFeatures::new(&le_supported_features);
    /// assert!(result.is_periodic_advertising_sync_transfer_sender_supported());
    /// ```
    pub fn is_periodic_advertising_sync_transfer_sender_supported(&self) -> bool {
        *self.le_supported_features.get(24).unwrap_or(&false)
    }

    /// check Periodic Advertising Sync Transfer - Recipient Feature.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::le_supported_features::LeSupportedFeatures;
    ///
    /// let mut le_supported_features = [false; 48].to_vec();
    /// le_supported_features[25] = true;
    /// let result = LeSupportedFeatures::new(&le_supported_features);
    /// assert!(result.is_periodic_advertising_sync_transfer_recipient_supported());
    /// ```
    pub fn is_periodic_advertising_sync_transfer_recipient_supported(&self) -> bool {
        *self.le_supported_features.get(25).unwrap_or(&false)
    }

    /// check Sleep Clock Accuracy Updates Feature.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::le_supported_features::LeSupportedFeatures;
    ///
    /// let mut le_supported_features = [false; 48].to_vec();
    /// le_supported_features[26] = true;
    /// let result = LeSupportedFeatures::new(&le_supported_features);
    /// assert!(result.is_sleep_clock_accuracy_updates_supported());
    /// ```
    pub fn is_sleep_clock_accuracy_updates_supported(&self) -> bool {
        *self.le_supported_features.get(26).unwrap_or(&false)
    }

    /// check Remote Public Key Validation Feature.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::le_supported_features::LeSupportedFeatures;
    ///
    /// let mut le_supported_features = [false; 48].to_vec();
    /// le_supported_features[27] = true;
    /// let result = LeSupportedFeatures::new(&le_supported_features);
    /// assert!(result.is_remote_public_key_validation_supported());
    /// ```
    pub fn is_remote_public_key_validation_supported(&self) -> bool {
        *self.le_supported_features.get(27).unwrap_or(&false)
    }

    /// check Connected Isochronous Stream – Central Feature.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::le_supported_features::LeSupportedFeatures;
    ///
    /// let mut le_supported_features = [false; 48].to_vec();
    /// le_supported_features[28] = true;
    /// let result = LeSupportedFeatures::new(&le_supported_features);
    /// assert!(result.is_connected_isochronous_stream_central_supported());
    /// ```
    pub fn is_connected_isochronous_stream_central_supported(&self) -> bool {
        *self.le_supported_features.get(28).unwrap_or(&false)
    }

    /// check Connected Isochronous Stream – Peripheral Feature.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::le_supported_features::LeSupportedFeatures;
    ///
    /// let mut le_supported_features = [false; 48].to_vec();
    /// le_supported_features[29] = true;
    /// let result = LeSupportedFeatures::new(&le_supported_features);
    /// assert!(result.is_connected_isochronous_stream_peripheral_supported());
    /// ```
    pub fn is_connected_isochronous_stream_peripheral_supported(&self) -> bool {
        *self.le_supported_features.get(29).unwrap_or(&false)
    }

    /// check Isochronous Broadcaster Feature.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::le_supported_features::LeSupportedFeatures;
    ///
    /// let mut le_supported_features = [false; 48].to_vec();
    /// le_supported_features[30] = true;
    /// let result = LeSupportedFeatures::new(&le_supported_features);
    /// assert!(result.is_isochronous_broadcaster_supported());
    /// ```
    pub fn is_isochronous_broadcaster_supported(&self) -> bool {
        *self.le_supported_features.get(30).unwrap_or(&false)
    }

    /// check Synchronized Receiver Feature.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::le_supported_features::LeSupportedFeatures;
    ///
    /// let mut le_supported_features = [false; 48].to_vec();
    /// le_supported_features[31] = true;
    /// let result = LeSupportedFeatures::new(&le_supported_features);
    /// assert!(result.is_synchronized_receiver_supported());
    /// ```
    pub fn is_synchronized_receiver_supported(&self) -> bool {
        *self.le_supported_features.get(31).unwrap_or(&false)
    }

    /// check Connected Isochronous Stream (Host Support) Feature.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::le_supported_features::LeSupportedFeatures;
    ///
    /// let mut le_supported_features = [false; 48].to_vec();
    /// le_supported_features[32] = true;
    /// let result = LeSupportedFeatures::new(&le_supported_features);
    /// assert!(result.is_connected_isochronous_stream_host_support_supported());
    /// ```
    pub fn is_connected_isochronous_stream_host_support_supported(&self) -> bool {
        *self.le_supported_features.get(32).unwrap_or(&false)
    }

    /// check LE Power Control Request Feature.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::le_supported_features::LeSupportedFeatures;
    ///
    /// let mut le_supported_features = [false; 48].to_vec();
    /// le_supported_features[33] = true;
    /// let result = LeSupportedFeatures::new(&le_supported_features);
    /// assert!(result.is_le_power_control_request_supported());
    ///
    /// let mut le_supported_features = [false; 48].to_vec();
    /// le_supported_features[34] = true;
    /// let result = LeSupportedFeatures::new(&le_supported_features);
    /// assert!(result.is_le_power_control_request_supported());
    /// ```
    pub fn is_le_power_control_request_supported(&self) -> bool {
        *self.le_supported_features.get(33).unwrap_or(&false)
            | *self.le_supported_features.get(34).unwrap_or(&false)
    }

    /// check LE Path Loss Monitoring Feature.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::le_supported_features::LeSupportedFeatures;
    ///
    /// let mut le_supported_features = [false; 48].to_vec();
    /// le_supported_features[35] = true;
    /// let result = LeSupportedFeatures::new(&le_supported_features);
    /// assert!(result.is_le_path_loss_monitoring_supported());
    /// ```
    pub fn is_le_path_loss_monitoring_supported(&self) -> bool {
        *self.le_supported_features.get(35).unwrap_or(&false)
    }

    /// check Periodic Advertising ADI support Feature.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::le_supported_features::LeSupportedFeatures;
    ///
    /// let mut le_supported_features = [false; 48].to_vec();
    /// le_supported_features[36] = true;
    /// let result = LeSupportedFeatures::new(&le_supported_features);
    /// assert!(result.is_periodic_advertising_adi_support_supported());
    /// ```
    pub fn is_periodic_advertising_adi_support_supported(&self) -> bool {
        *self.le_supported_features.get(36).unwrap_or(&false)
    }

    /// check Connection Subrating support Feature.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::le_supported_features::LeSupportedFeatures;
    ///
    /// let mut le_supported_features = [false; 48].to_vec();
    /// le_supported_features[37] = true;
    /// let result = LeSupportedFeatures::new(&le_supported_features);
    /// assert!(result.is_connection_subrating_supported());
    /// ```
    pub fn is_connection_subrating_supported(&self) -> bool {
        *self.le_supported_features.get(37).unwrap_or(&false)
    }

    /// check Connection Subrating (Host Support) Feature.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::le_supported_features::LeSupportedFeatures;
    ///
    /// let mut le_supported_features = [false; 48].to_vec();
    /// le_supported_features[38] = true;
    /// let result = LeSupportedFeatures::new(&le_supported_features);
    /// assert!(result.is_connection_subrating_host_support_supported());
    /// ```
    pub fn is_connection_subrating_host_support_supported(&self) -> bool {
        *self.le_supported_features.get(38).unwrap_or(&false)
    }

    /// check Channel Classification Feature.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::le_supported_features::LeSupportedFeatures;
    ///
    /// let mut le_supported_features = [false; 48].to_vec();
    /// le_supported_features[39] = true;
    /// let result = LeSupportedFeatures::new(&le_supported_features);
    /// assert!(result.is_channel_classification_supported());
    /// ```
    pub fn is_channel_classification_supported(&self) -> bool {
        *self.le_supported_features.get(39).unwrap_or(&false)
    }

    /// check Advertising Coding Selection Feature.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::le_supported_features::LeSupportedFeatures;
    ///
    /// let mut le_supported_features = [false; 48].to_vec();
    /// le_supported_features[40] = true;
    /// let result = LeSupportedFeatures::new(&le_supported_features);
    /// assert!(result.is_advertising_coding_selection_supported());
    /// ```
    pub fn is_advertising_coding_selection_supported(&self) -> bool {
        *self.le_supported_features.get(40).unwrap_or(&false)
    }

    /// check Advertising Coding Selection (Host Support) Feature.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::le_supported_features::LeSupportedFeatures;
    ///
    /// let mut le_supported_features = [false; 48].to_vec();
    /// le_supported_features[41] = true;
    /// let result = LeSupportedFeatures::new(&le_supported_features);
    /// assert!(result.is_advertising_coding_selection_host_support_supported());
    /// ```
    pub fn is_advertising_coding_selection_host_support_supported(&self) -> bool {
        *self.le_supported_features.get(41).unwrap_or(&false)
    }

    /// check Periodic Advertising with Responses - Advertiser Feature.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::le_supported_features::LeSupportedFeatures;
    ///
    /// let mut le_supported_features = [false; 48].to_vec();
    /// le_supported_features[43] = true;
    /// let result = LeSupportedFeatures::new(&le_supported_features);
    /// assert!(result.is_periodic_advertising_with_responses_advertiser_supported());
    /// ```
    pub fn is_periodic_advertising_with_responses_advertiser_supported(&self) -> bool {
        *self.le_supported_features.get(43).unwrap_or(&false)
    }

    /// check Periodic Advertising with Responses - Scanner Feature.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::le_supported_features::LeSupportedFeatures;
    ///
    /// let mut le_supported_features = [false; 48].to_vec();
    /// le_supported_features[44] = true;
    /// let result = LeSupportedFeatures::new(&le_supported_features);
    /// assert!(result.is_periodic_advertising_with_responses_scanner_supported());
    /// ```
    pub fn is_periodic_advertising_with_responses_scanner_supported(&self) -> bool {
        *self.le_supported_features.get(44).unwrap_or(&false)
    }
}

impl From<&Vec<u8>> for LeSupportedFeatures {
    /// Create [LeSupportedFeatures] from `Vec<u8>`.
    ///
    /// [`LeSupportedFeatures::from_with_offset`]
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{le_supported_features::LeSupportedFeatures, data_type::DataType};
    ///
    /// let mut le_supported_features = [0u8; 6].to_vec();
    ///
    /// for i in 0..44 {
    ///     le_supported_features[i / 8] = 0b1 << (i % 8);
    ///
    ///     let length = le_supported_features.len() as u8 + 1;
    ///     let mut data: Vec<u8> = Vec::new();
    ///     data.push(length);
    ///     data.push(LeSupportedFeatures::data_type());
    ///     data.append(&mut le_supported_features.clone());
    ///
    ///     let result = LeSupportedFeatures::from(&data);
    ///     assert_eq!(length, result.length);
    ///     let bool_vec: Vec<bool> = le_supported_features
    ///         .clone()
    ///         .iter()
    ///         .flat_map(|x| {
    ///             let mut data: Vec<bool> = Vec::new();
    ///             data.push((x & 0b0000_0001) != 0);
    ///             data.push((x & 0b0000_0010) != 0);
    ///             data.push((x & 0b0000_0100) != 0);
    ///             data.push((x & 0b0000_1000) != 0);
    ///             data.push((x & 0b0001_0000) != 0);
    ///             data.push((x & 0b0010_0000) != 0);
    ///             data.push((x & 0b0100_0000) != 0);
    ///             data.push((x & 0b1000_0000) != 0);
    ///             data
    ///         })
    ///         .collect();
    ///
    ///     assert_eq!(bool_vec, result.le_supported_features);
    ///
    ///     le_supported_features[i / 8] = 0u8;
    /// }
    /// ```
    fn from(data: &Vec<u8>) -> Self {
        Self::from_with_offset(data, 0)
    }
}

impl Into<Vec<u8>> for LeSupportedFeatures {
    /// Create `Vec<u8>` from [LeSupportedFeatures].
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{le_supported_features::LeSupportedFeatures, data_type::DataType};
    ///
    /// let mut le_supported_features = [false; 48].to_vec();
    /// for i in 0..44 {
    ///     le_supported_features[i] = true;
    ///     let result1 = LeSupportedFeatures::new(&le_supported_features);

    ///     let mut data: Vec<u8> = Vec::new();
    ///     data.push(le_supported_features.len() as u8 / 8 + 1);
    ///     data.push(LeSupportedFeatures::data_type());
    ///     let mut u8_vec = [0u8; 6];
    ///     for (i, element) in le_supported_features.iter().enumerate() {
    ///         if *element {
    ///             u8_vec[i / 8] = u8_vec[i / 8] | 1 << i % 8
    ///         }
    ///     }
    ///     data.append(&mut u8_vec.clone().to_vec());

    ///     let into_data: Vec<u8> = result1.into();
    ///     assert_eq!(data, into_data);

    ///     let result2 = LeSupportedFeatures::from(&data);
    ///     let into_data: Vec<u8> = result2.into();
    ///     assert_eq!(data, into_data);

    ///     le_supported_features[i] = false;
    /// }
    /// ```
    fn into(self) -> Vec<u8> {
        let mut data: Vec<u8> = Vec::new();
        data.push(self.length);
        data.push(Self::data_type());
        let mut le_supported_features: Vec<u8> = self
            .le_supported_features
            .windows(8)
            .step_by(8)
            .map(|f| {
                let mut byte = 0u8;
                byte |= if f[0] { 0b0000_0001 } else { 0 };
                byte |= if f[1] { 0b0000_0001 << 1 } else { 0 };
                byte |= if f[2] { 0b0000_0001 << 2 } else { 0 };
                byte |= if f[3] { 0b0000_0001 << 3 } else { 0 };
                byte |= if f[4] { 0b0000_0001 << 4 } else { 0 };
                byte |= if f[5] { 0b0000_0001 << 5 } else { 0 };
                byte |= if f[6] { 0b0000_0001 << 6 } else { 0 };
                byte |= if f[7] { 0b0000_0001 << 7 } else { 0 };
                byte
            })
            .collect();
        data.append(&mut le_supported_features);
        return data;
    }
}

impl DataType for LeSupportedFeatures {
    /// return `0x27`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{le_supported_features::LeSupportedFeatures, data_type::DataType};
    ///
    /// assert_eq!(0x27, LeSupportedFeatures::data_type());
    /// ```
    fn data_type() -> u8 {
        0x27
    }
}

/// check `LE Supported Features.` data type.
///
/// # Examples
///
/// ```
/// use ble_data_struct::data_types::le_supported_features::*;
/// use ble_data_struct::data_types::data_type::DataType;
///
/// assert!(is_le_supported_features(0x27));
/// assert!(!is_le_supported_features(0x00));
/// ```
pub fn is_le_supported_features(data_type: u8) -> bool {
    LeSupportedFeatures::data_type() == data_type
}

#[cfg(test)]
mod tests {

    use crate::data_types::{data_type::DataType, le_supported_features::*};

    #[test]
    fn test_new() {
        let mut le_supported_features = [false; 48].to_vec();
        for i in 0..44 {
            le_supported_features[i] = true;
            let result = LeSupportedFeatures::new(&le_supported_features);
            assert_eq!(7, result.length);
            assert_eq!(le_supported_features, result.le_supported_features);
            le_supported_features[i] = false;
        }
    }

    #[test]
    fn test_from_with_offset() {
        let mut le_supported_features = [0u8; 6].to_vec();

        for i in 0..44 {
            le_supported_features[i / 8] = 0b1 << (i % 8);

            let length = le_supported_features.len() as u8 + 1;
            let mut data: Vec<u8> = Vec::new();
            data.push(length);
            data.push(LeSupportedFeatures::data_type());
            data.append(&mut le_supported_features.clone());

            let result = LeSupportedFeatures::from_with_offset(&data, 0);
            assert_eq!(length, result.length);
            let bool_vec: Vec<bool> = le_supported_features
                .clone()
                .iter()
                .flat_map(|x| {
                    let mut data: Vec<bool> = Vec::new();
                    data.push((x & 0b0000_0001) != 0);
                    data.push((x & 0b0000_0010) != 0);
                    data.push((x & 0b0000_0100) != 0);
                    data.push((x & 0b0000_1000) != 0);
                    data.push((x & 0b0001_0000) != 0);
                    data.push((x & 0b0010_0000) != 0);
                    data.push((x & 0b0100_0000) != 0);
                    data.push((x & 0b1000_0000) != 0);
                    data
                })
                .collect();

            assert_eq!(bool_vec, result.le_supported_features);

            le_supported_features[i / 8] = 0u8;
        }

        let mut le_supported_features = [0u8; 6].to_vec();

        for i in 0..44 {
            le_supported_features[i / 8] = 0b1 << (i % 8);

            let length = le_supported_features.len() as u8 + 1;
            let mut data: Vec<u8> = Vec::new();
            data.push(0);
            data.push(length);
            data.push(LeSupportedFeatures::data_type());
            data.append(&mut le_supported_features.clone());

            let result = LeSupportedFeatures::from_with_offset(&data, 1);
            assert_eq!(length, result.length);
            let bool_vec: Vec<bool> = le_supported_features
                .clone()
                .iter()
                .flat_map(|x| {
                    let mut data: Vec<bool> = Vec::new();
                    data.push((x & 0b0000_0001) != 0);
                    data.push((x & 0b0000_0010) != 0);
                    data.push((x & 0b0000_0100) != 0);
                    data.push((x & 0b0000_1000) != 0);
                    data.push((x & 0b0001_0000) != 0);
                    data.push((x & 0b0010_0000) != 0);
                    data.push((x & 0b0100_0000) != 0);
                    data.push((x & 0b1000_0000) != 0);
                    data
                })
                .collect();

            assert_eq!(bool_vec, result.le_supported_features);

            le_supported_features[i / 8] = 0u8;
        }
    }

    #[test]
    fn test_is_le_encryption_supported() {
        let mut le_supported_features = [false; 48].to_vec();
        le_supported_features[0] = true;
        let result = LeSupportedFeatures::new(&le_supported_features);
        assert!(result.is_le_encryption_supported());
    }

    #[test]
    fn test_is_connection_parameters_request_procedure_supported() {
        let mut le_supported_features = [false; 48].to_vec();
        le_supported_features[1] = true;
        let result = LeSupportedFeatures::new(&le_supported_features);
        assert!(result.is_connection_parameters_request_procedure_supported());
    }

    #[test]
    fn test_is_extended_reject_indication_supported() {
        let mut le_supported_features = [false; 48].to_vec();
        le_supported_features[2] = true;
        let result = LeSupportedFeatures::new(&le_supported_features);
        assert!(result.is_extended_reject_indication_supported());
    }

    #[test]
    fn test_is_peripheral_initiated_features_exchange_supported() {
        let mut le_supported_features = [false; 48].to_vec();
        le_supported_features[3] = true;
        let result = LeSupportedFeatures::new(&le_supported_features);
        assert!(result.is_peripheral_initiated_features_exchange_supported());
    }

    #[test]
    fn test_is_le_ping_supported() {
        let mut le_supported_features = [false; 48].to_vec();
        le_supported_features[4] = true;
        let result = LeSupportedFeatures::new(&le_supported_features);
        assert!(result.is_le_ping_supported());
    }

    #[test]
    fn test_is_le_data_packet_length_extension_supported() {
        let mut le_supported_features = [false; 48].to_vec();
        le_supported_features[5] = true;
        let result = LeSupportedFeatures::new(&le_supported_features);
        assert!(result.is_le_data_packet_length_extension_supported());
    }

    #[test]
    fn test_is_ll_privacy_supported() {
        let mut le_supported_features = [false; 48].to_vec();
        le_supported_features[6] = true;
        let result = LeSupportedFeatures::new(&le_supported_features);
        assert!(result.is_ll_privacy_supported());
    }

    #[test]
    fn test_is_extended_scanning_filter_policies_supported() {
        let mut le_supported_features = [false; 48].to_vec();
        le_supported_features[7] = true;
        let result = LeSupportedFeatures::new(&le_supported_features);
        assert!(result.is_extended_scanning_filter_policies_supported());
    }

    #[test]
    fn test_is_le_2m_phy_supported() {
        let mut le_supported_features = [false; 48].to_vec();
        le_supported_features[8] = true;
        let result = LeSupportedFeatures::new(&le_supported_features);
        assert!(result.is_le_2m_phy_supported());
    }

    #[test]
    fn test_is_stable_modulation_index_transmitter_supported() {
        let mut le_supported_features = [false; 48].to_vec();
        le_supported_features[9] = true;
        let result = LeSupportedFeatures::new(&le_supported_features);
        assert!(result.is_stable_modulation_index_transmitter_supported());
    }

    #[test]
    fn test_is_stable_modulation_index_receiver_supported() {
        let mut le_supported_features = [false; 48].to_vec();
        le_supported_features[10] = true;
        let result = LeSupportedFeatures::new(&le_supported_features);
        assert!(result.is_stable_modulation_index_receiver_supported());
    }

    #[test]
    fn test_is_le_coded_phy_supported() {
        let mut le_supported_features = [false; 48].to_vec();
        le_supported_features[11] = true;
        let result = LeSupportedFeatures::new(&le_supported_features);
        assert!(result.is_le_coded_phy_supported());
    }

    #[test]
    fn test_is_le_extended_advertising_supported() {
        let mut le_supported_features = [false; 48].to_vec();
        le_supported_features[12] = true;
        let result = LeSupportedFeatures::new(&le_supported_features);
        assert!(result.is_le_extended_advertising_supported());
    }

    #[test]
    fn test_is_le_periodic_advertising_supported() {
        let mut le_supported_features = [false; 48].to_vec();
        le_supported_features[13] = true;
        let result = LeSupportedFeatures::new(&le_supported_features);
        assert!(result.is_le_periodic_advertising_supported());
    }

    #[test]
    fn test_is_channel_selection_algorithm2_supported() {
        let mut le_supported_features = [false; 48].to_vec();
        le_supported_features[14] = true;
        let result = LeSupportedFeatures::new(&le_supported_features);
        assert!(result.is_channel_selection_algorithm2_supported());
    }

    #[test]
    fn test_is_le_power_class1_supported() {
        let mut le_supported_features = [false; 48].to_vec();
        le_supported_features[15] = true;
        let result = LeSupportedFeatures::new(&le_supported_features);
        assert!(result.is_le_power_class1_supported());
    }

    #[test]
    fn test_is_minimum_number_of_used_channels_procedure_supported() {
        let mut le_supported_features = [false; 48].to_vec();
        le_supported_features[16] = true;
        let result = LeSupportedFeatures::new(&le_supported_features);
        assert!(result.is_minimum_number_of_used_channels_procedure_supported());
    }

    #[test]
    fn test_is_connection_cte_request_supported() {
        let mut le_supported_features = [false; 48].to_vec();
        le_supported_features[17] = true;
        let result = LeSupportedFeatures::new(&le_supported_features);
        assert!(result.is_connection_cte_request_supported());
    }

    #[test]
    fn test_is_connection_cte_response_supported() {
        let mut le_supported_features = [false; 48].to_vec();
        le_supported_features[18] = true;
        let result = LeSupportedFeatures::new(&le_supported_features);
        assert!(result.is_connection_cte_response_supported());
    }

    #[test]
    fn test_is_connectionless_cte_transmitter_supported() {
        let mut le_supported_features = [false; 48].to_vec();
        le_supported_features[19] = true;
        let result = LeSupportedFeatures::new(&le_supported_features);
        assert!(result.is_connectionless_cte_transmitter_supported());
    }

    #[test]
    fn test_is_connectionless_cte_receiver_supported() {
        let mut le_supported_features = [false; 48].to_vec();
        le_supported_features[20] = true;
        let result = LeSupportedFeatures::new(&le_supported_features);
        assert!(result.is_connectionless_cte_receiver_supported());
    }

    #[test]
    fn test_is_antenna_switching_during_cte_transmission_aod_supported() {
        let mut le_supported_features = [false; 48].to_vec();
        le_supported_features[21] = true;
        let result = LeSupportedFeatures::new(&le_supported_features);
        assert!(result.is_antenna_switching_during_cte_transmission_aod_supported());
    }

    #[test]
    fn test_is_antenna_switching_during_cte_reception_aoa_supported() {
        let mut le_supported_features = [false; 48].to_vec();
        le_supported_features[22] = true;
        let result = LeSupportedFeatures::new(&le_supported_features);
        assert!(result.is_antenna_switching_during_cte_reception_aoa_supported());
    }

    #[test]
    fn test_is_receiving_constant_tone_extensions_supported() {
        let mut le_supported_features = [false; 48].to_vec();
        le_supported_features[23] = true;
        let result = LeSupportedFeatures::new(&le_supported_features);
        assert!(result.is_receiving_constant_tone_extensions_supported());
    }

    #[test]
    fn test_is_periodic_advertising_sync_transfer_sender_supported() {
        let mut le_supported_features = [false; 48].to_vec();
        le_supported_features[24] = true;
        let result = LeSupportedFeatures::new(&le_supported_features);
        assert!(result.is_periodic_advertising_sync_transfer_sender_supported());
    }

    #[test]
    fn test_is_periodic_advertising_sync_transfer_recipient_supported() {
        let mut le_supported_features = [false; 48].to_vec();
        le_supported_features[25] = true;
        let result = LeSupportedFeatures::new(&le_supported_features);
        assert!(result.is_periodic_advertising_sync_transfer_recipient_supported());
    }

    #[test]
    fn test_is_sleep_clock_accuracy_updates_supported() {
        let mut le_supported_features = [false; 48].to_vec();
        le_supported_features[26] = true;
        let result = LeSupportedFeatures::new(&le_supported_features);
        assert!(result.is_sleep_clock_accuracy_updates_supported());
    }

    #[test]
    fn test_is_remote_public_key_validation_supported() {
        let mut le_supported_features = [false; 48].to_vec();
        le_supported_features[27] = true;
        let result = LeSupportedFeatures::new(&le_supported_features);
        assert!(result.is_remote_public_key_validation_supported());
    }

    #[test]
    fn test_is_connected_isochronous_stream_central_supported() {
        let mut le_supported_features = [false; 48].to_vec();
        le_supported_features[28] = true;
        let result = LeSupportedFeatures::new(&le_supported_features);
        assert!(result.is_connected_isochronous_stream_central_supported());
    }

    #[test]
    fn test_is_connected_isochronous_stream_peripheral_supported() {
        let mut le_supported_features = [false; 48].to_vec();
        le_supported_features[29] = true;
        let result = LeSupportedFeatures::new(&le_supported_features);
        assert!(result.is_connected_isochronous_stream_peripheral_supported());
    }

    #[test]
    fn test_is_isochronous_broadcaster_supported() {
        let mut le_supported_features = [false; 48].to_vec();
        le_supported_features[30] = true;
        let result = LeSupportedFeatures::new(&le_supported_features);
        assert!(result.is_isochronous_broadcaster_supported());
    }

    #[test]
    fn test_is_synchronized_receiver_supported() {
        let mut le_supported_features = [false; 48].to_vec();
        le_supported_features[31] = true;
        let result = LeSupportedFeatures::new(&le_supported_features);
        assert!(result.is_synchronized_receiver_supported());
    }

    #[test]
    fn test_is_connected_isochronous_stream_host_support_supported() {
        let mut le_supported_features = [false; 48].to_vec();
        le_supported_features[32] = true;
        let result = LeSupportedFeatures::new(&le_supported_features);
        assert!(result.is_connected_isochronous_stream_host_support_supported());
    }

    #[test]
    fn test_is_le_power_control_request_supported() {
        let mut le_supported_features = [false; 48].to_vec();
        le_supported_features[33] = true;
        let result = LeSupportedFeatures::new(&le_supported_features);
        assert!(result.is_le_power_control_request_supported());

        let mut le_supported_features = [false; 48].to_vec();
        le_supported_features[34] = true;
        let result = LeSupportedFeatures::new(&le_supported_features);
        assert!(result.is_le_power_control_request_supported());
    }

    #[test]
    fn test_is_le_path_loss_monitoring_supported() {
        let mut le_supported_features = [false; 48].to_vec();
        le_supported_features[35] = true;
        let result = LeSupportedFeatures::new(&le_supported_features);
        assert!(result.is_le_path_loss_monitoring_supported());
    }

    #[test]
    fn test_is_periodic_advertising_adi_support_supported() {
        let mut le_supported_features = [false; 48].to_vec();
        le_supported_features[36] = true;
        let result = LeSupportedFeatures::new(&le_supported_features);
        assert!(result.is_periodic_advertising_adi_support_supported());
    }

    #[test]
    fn test_is_connection_subrating_supported() {
        let mut le_supported_features = [false; 48].to_vec();
        le_supported_features[37] = true;
        let result = LeSupportedFeatures::new(&le_supported_features);
        assert!(result.is_connection_subrating_supported());
    }

    #[test]
    fn test_is_connection_subrating_host_support_supported() {
        let mut le_supported_features = [false; 48].to_vec();
        le_supported_features[38] = true;
        let result = LeSupportedFeatures::new(&le_supported_features);
        assert!(result.is_connection_subrating_host_support_supported());
    }

    #[test]
    fn test_is_channel_classification_supported() {
        let mut le_supported_features = [false; 48].to_vec();
        le_supported_features[39] = true;
        let result = LeSupportedFeatures::new(&le_supported_features);
        assert!(result.is_channel_classification_supported());
    }

    #[test]
    fn test_is_advertising_coding_selection_supported() {
        let mut le_supported_features = [false; 48].to_vec();
        le_supported_features[40] = true;
        let result = LeSupportedFeatures::new(&le_supported_features);
        assert!(result.is_advertising_coding_selection_supported());
    }

    #[test]
    fn test_is_advertising_coding_selection_host_support_supported() {
        let mut le_supported_features = [false; 48].to_vec();
        le_supported_features[41] = true;
        let result = LeSupportedFeatures::new(&le_supported_features);
        assert!(result.is_advertising_coding_selection_host_support_supported());
    }

    #[test]
    fn test_is_periodic_advertising_with_responses_advertiser_supported() {
        let mut le_supported_features = [false; 48].to_vec();
        le_supported_features[43] = true;
        let result = LeSupportedFeatures::new(&le_supported_features);
        assert!(result.is_periodic_advertising_with_responses_advertiser_supported());
    }

    #[test]
    fn test_is_periodic_advertising_with_responses_scanner_supported() {
        let mut le_supported_features = [false; 48].to_vec();
        le_supported_features[44] = true;
        let result = LeSupportedFeatures::new(&le_supported_features);
        assert!(result.is_periodic_advertising_with_responses_scanner_supported());
    }

    #[test]
    fn test_from() {
        let mut le_supported_features = [0u8; 6].to_vec();

        for i in 0..44 {
            le_supported_features[i / 8] = 0b1 << (i % 8);

            let length = le_supported_features.len() as u8 + 1;
            let mut data: Vec<u8> = Vec::new();
            data.push(length);
            data.push(LeSupportedFeatures::data_type());
            data.append(&mut le_supported_features.clone());

            let result = LeSupportedFeatures::from(&data);
            assert_eq!(length, result.length);
            let bool_vec: Vec<bool> = le_supported_features
                .clone()
                .iter()
                .flat_map(|x| {
                    let mut data: Vec<bool> = Vec::new();
                    data.push((x & 0b0000_0001) != 0);
                    data.push((x & 0b0000_0010) != 0);
                    data.push((x & 0b0000_0100) != 0);
                    data.push((x & 0b0000_1000) != 0);
                    data.push((x & 0b0001_0000) != 0);
                    data.push((x & 0b0010_0000) != 0);
                    data.push((x & 0b0100_0000) != 0);
                    data.push((x & 0b1000_0000) != 0);
                    data
                })
                .collect();

            assert_eq!(bool_vec, result.le_supported_features);

            le_supported_features[i / 8] = 0u8;
        }
    }

    #[test]
    fn test_into() {
        let mut le_supported_features = [false; 48].to_vec();
        for i in 0..44 {
            le_supported_features[i] = true;
            let result1 = LeSupportedFeatures::new(&le_supported_features);

            let mut data: Vec<u8> = Vec::new();
            data.push(le_supported_features.len() as u8 / 8 + 1);
            data.push(LeSupportedFeatures::data_type());
            let mut u8_vec = [0u8; 6];
            for (i, element) in le_supported_features.iter().enumerate() {
                if *element {
                    u8_vec[i / 8] = u8_vec[i / 8] | 1 << i % 8
                }
            }
            data.append(&mut u8_vec.clone().to_vec());

            let into_data: Vec<u8> = result1.into();
            assert_eq!(data, into_data);

            let result2 = LeSupportedFeatures::from(&data);
            let into_data: Vec<u8> = result2.into();
            assert_eq!(data, into_data);

            le_supported_features[i] = false;
        }
    }

    #[test]
    fn test_data_type() {
        assert_eq!(0x27, LeSupportedFeatures::data_type());
    }

    #[test]
    fn test_is_le_supported_features() {
        assert!(is_le_supported_features(0x27));
        assert!(!is_le_supported_features(0x00));
    }
}
