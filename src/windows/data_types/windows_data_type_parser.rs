//! Data type parser module for windows.
#[cfg(target_os = "windows")]
use crate::{
    data_types::data_type_parser::{DataTypeParseResult, DataTypeParseResults},
    windows::buffer::i_buffer_to_vec,
};
#[cfg(target_os = "windows")]
use windows::{
    core::Error,
    Devices::Bluetooth::Advertisement::{
        BluetoothLEAdvertisement, BluetoothLEAdvertisementDataSection,
    },
};

#[cfg(target_os = "windows")]
impl From<BluetoothLEAdvertisementDataSection> for DataTypeParseResult {
    /// Create [`DataTypeParseResult`] from [`BluetoothLEAdvertisementDataSection`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::data_type::DataType;
    /// use ble_data_struct::data_types::data_type_parser::DataTypeParseResult;
    /// use ble_data_struct::data_types::advertising_interval::AdvertisingInterval;
    /// use windows::Storage::Streams::DataWriter;
    /// use windows::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisementDataSection;
    ///
    /// let advertising_interval = AdvertisingInterval::new(1);
    /// let data_section = BluetoothLEAdvertisementDataSection::new().unwrap();
    /// data_section
    ///     .SetDataType(AdvertisingInterval::data_type())
    ///     .unwrap();
    /// let data_writer = DataWriter::new().unwrap();
    /// let ble_packet: Vec<u8> = advertising_interval.into();
    /// let buffer_data = ble_packet[2..].to_vec();
    /// data_writer.WriteBytes(&buffer_data).unwrap();
    /// let buffer = data_writer.DetachBuffer().unwrap();
    /// data_section.SetData(&buffer).unwrap();
    ///
    /// let data_type_parse_result = DataTypeParseResult::from(data_section);
    /// assert!(matches!(
    ///     data_type_parse_result,
    ///     DataTypeParseResult::AdvertisingIntervalResult(_)
    /// ));
    /// if let DataTypeParseResult::AdvertisingIntervalResult(x) = data_type_parse_result {
    ///     assert!(x.is_ok());
    ///     let converted_data: Vec<u8> = x.unwrap().into();
    ///     assert_eq!(ble_packet, converted_data);
    /// } else {
    ///     panic!();
    /// }
    /// ```
    fn from(data_section: BluetoothLEAdvertisementDataSection) -> Self {
        let data_type = match data_section.DataType() {
            Ok(data_type) => data_type,
            Err(error) => return create_error_result(error),
        };

        let i_buffer = match data_section.Data() {
            Ok(buffer) => buffer,
            Err(error) => return create_error_result(error),
        };
        match i_buffer_to_vec(i_buffer) {
            Ok(mut vec) => {
                let mut data: Vec<u8> = Vec::new();
                data.push(vec.len() as u8 + 1);
                data.push(data_type);

                data.append(&mut vec);
                DataTypeParseResult::from(&data)
            }
            Err(error) => create_error_result(error),
        }
    }
}

#[cfg(target_os = "windows")]
fn create_error_result(error: Error) -> DataTypeParseResult {
    DataTypeParseResult::DataTypeParseError(error.message())
}

#[cfg(target_os = "windows")]
impl TryFrom<BluetoothLEAdvertisement> for DataTypeParseResults {
    type Error = String;

    /// Create [`DataTypeParseResults`] from [`BluetoothLEAdvertisement`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::data_type_parser::DataTypeParseResults;
    /// use windows::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisement;
    ///
    /// let bluetooth_le_advertisiement = BluetoothLEAdvertisement::new().unwrap();
    /// let data_type_parse_results = DataTypeParseResults::try_from(bluetooth_le_advertisiement);
    ///
    /// assert!(matches!(data_type_parse_results, Ok(_)));
    /// ```
    fn try_from(value: BluetoothLEAdvertisement) -> Result<Self, Self::Error> {
        let data_sections = match value.DataSections() {
            Ok(data_sections) => data_sections,
            Err(error) => return Err(error.to_string()),
        };

        let vec = data_sections
            .into_iter()
            .map(|f| DataTypeParseResult::from(f))
            .collect();

        Ok(DataTypeParseResults::new(vec))
    }
}

#[cfg(target_os = "windows")]
#[cfg(test)]
mod tests {
    use windows::{
        Devices::Bluetooth::Advertisement::{
            BluetoothLEAdvertisement, BluetoothLEAdvertisementDataSection,
        },
        Storage::Streams::DataWriter,
    };

    use crate::data_types::{
        advertising_interval::AdvertisingInterval,
        data_type::DataType,
        data_type_parser::{DataTypeParseResult, DataTypeParseResults},
    };

    #[test]
    fn test_from() {
        let advertising_interval = AdvertisingInterval::new(1);
        let data_section = BluetoothLEAdvertisementDataSection::new().unwrap();
        data_section
            .SetDataType(AdvertisingInterval::data_type())
            .unwrap();
        let data_writer = DataWriter::new().unwrap();
        let ble_packet: Vec<u8> = advertising_interval.into();
        let buffer_data = ble_packet[2..].to_vec();
        data_writer.WriteBytes(&buffer_data).unwrap();
        let buffer = data_writer.DetachBuffer().unwrap();
        data_section.SetData(&buffer).unwrap();

        let data_type_parse_result = DataTypeParseResult::from(data_section);
        assert!(matches!(
            data_type_parse_result,
            DataTypeParseResult::AdvertisingIntervalResult(_)
        ));
        if let DataTypeParseResult::AdvertisingIntervalResult(x) = data_type_parse_result {
            assert!(x.is_ok());
            let converted_data: Vec<u8> = x.unwrap().into();
            assert_eq!(ble_packet, converted_data);
        } else {
            panic!();
        }
    }

    #[test]
    fn test_try_from() {
        let bluetooth_le_advertisiement = BluetoothLEAdvertisement::new().unwrap();
        let data_type_parse_results = DataTypeParseResults::try_from(bluetooth_le_advertisiement);

        assert!(matches!(data_type_parse_results, Ok(_)));
    }
}
