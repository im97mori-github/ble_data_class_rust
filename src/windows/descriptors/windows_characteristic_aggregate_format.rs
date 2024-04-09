//! Characteristic Aggregate Format (Attribute Type: 0x2905) module for windows.

#[cfg(target_os = "windows")]
use crate::descriptors::characteristic_aggregate_format::CharacteristicAggregateFormat;
#[cfg(target_os = "windows")]
use crate::windows::buffer::{i_buffer_to_vec, vec_to_i_buffer};

#[cfg(target_os = "windows")]
use windows::Storage::Streams::IBuffer;

#[cfg(target_os = "windows")]
impl TryFrom<IBuffer> for CharacteristicAggregateFormat {
    type Error = String;
    /// Create [`CharacteristicAggregateFormat`] from [`IBuffer`].
    ///
    /// # Examples
    ///
    /// ```
    /// use windows::Storage::Streams::{DataWriter, IBuffer};
    ///
    /// use ble_data_struct::descriptors::characteristic_aggregate_format::CharacteristicAggregateFormat;
    ///
    /// let list_of_attribute_handles: Vec<u16> = [0x0201, 0x0403].to_vec();
    /// let chracteristic_aggregate_format =
    ///     CharacteristicAggregateFormat::new(&list_of_attribute_handles.clone());
    ///
    /// let data_writer = DataWriter::new().unwrap();
    /// let ble_packet: Vec<u8> = chracteristic_aggregate_format.into();
    /// data_writer.WriteBytes(&ble_packet).unwrap();
    /// let buffer = data_writer.DetachBuffer().unwrap();
    ///
    /// let result = CharacteristicAggregateFormat::try_from(buffer);
    /// assert!(result.is_ok());
    /// let value: Vec<u8> = result.unwrap().into();
    /// assert_eq!(ble_packet, value);
    /// ```
    fn try_from(value: IBuffer) -> Result<Self, String> {
        let vec = i_buffer_to_vec(value).unwrap();
        Self::try_from(&vec)
    }
}

#[cfg(target_os = "windows")]
impl Into<IBuffer> for CharacteristicAggregateFormat {
    /// Create [`IBuffer`] from [`CharacteristicAggregateFormat`].
    ///
    /// # Examples
    ///
    /// ```
    /// use windows::Storage::Streams::{DataWriter, IBuffer};
    ///
    /// use ble_data_struct::{
    ///     descriptors::characteristic_aggregate_format::CharacteristicAggregateFormat,
    ///     windows::buffer::i_buffer_to_vec,
    /// };
    ///
    /// let list_of_attribute_handles: Vec<u16> = [0x0201, 0x0403].to_vec();
    /// let value = CharacteristicAggregateFormat::new(&list_of_attribute_handles.clone());
    /// let buffer: IBuffer = value.clone().into();
    /// let vec: Vec<u8> = value.into();
    /// assert_eq!(vec, i_buffer_to_vec(buffer).unwrap());
    /// ```
    fn into(self) -> IBuffer {
        let vec: Vec<u8> = self.into();
        vec_to_i_buffer(&vec).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::descriptors::characteristic_aggregate_format::CharacteristicAggregateFormat;
    use windows::Storage::Streams::{DataWriter, IBuffer};

    use crate::windows::buffer::i_buffer_to_vec;

    #[test]
    fn test_try_from_i_buffer() {
        let list_of_attribute_handles: Vec<u16> = [0x0201, 0x0403].to_vec();
        let chracteristic_aggregate_format =
            CharacteristicAggregateFormat::new(&list_of_attribute_handles.clone());

        let data_writer = DataWriter::new().unwrap();
        let ble_packet: Vec<u8> = chracteristic_aggregate_format.into();
        data_writer.WriteBytes(&ble_packet).unwrap();
        let buffer = data_writer.DetachBuffer().unwrap();

        let result = CharacteristicAggregateFormat::try_from(buffer);
        assert!(result.is_ok());
        let value: Vec<u8> = result.unwrap().into();
        assert_eq!(ble_packet, value);
    }

    #[test]
    fn test_into_i_buffer() {
        let list_of_attribute_handles: Vec<u16> = [0x0201, 0x0403].to_vec();
        let value = CharacteristicAggregateFormat::new(&list_of_attribute_handles.clone());
        let buffer: IBuffer = value.clone().into();
        let vec: Vec<u8> = value.into();
        assert_eq!(vec, i_buffer_to_vec(buffer).unwrap());
    }
}
