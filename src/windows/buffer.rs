#[cfg(target_os = "windows")]
use windows::{
    core::Error,
    Storage::Streams::{DataReader, IBuffer},
};

/// Convert [`IBuffer`] to [`Vec<u8>`].
///
/// # Examples
///
/// ```
/// use windows::Storage::Streams::DataWriter;
/// use ble_data_struct::windows::buffer::i_buffer_to_vec;
///
/// let data = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9].to_vec();
/// let data_writer = DataWriter::new().unwrap();
/// data_writer.WriteBytes(&data).unwrap();
/// let buffer = data_writer.DetachBuffer().unwrap();
///
/// let result = i_buffer_to_vec(buffer);
/// assert!(result.is_ok());
/// assert_eq!(data.to_vec(), result.unwrap());
/// ```
#[cfg(target_os = "windows")]
pub fn i_buffer_to_vec(i_buffer: IBuffer) -> Result<Vec<u8>, Error> {
    let reader = match DataReader::FromBuffer(&i_buffer) {
        Ok(reader) => reader,
        Err(error) => return Err(error),
    };

    let length = match reader.UnconsumedBufferLength() {
        Ok(length) => length as u8,
        Err(error) => return Err(error),
    };

    let mut read_buffer = vec![0u8; length as usize];
    match reader.ReadBytes(read_buffer.as_mut()) {
        Ok(_) => Ok(read_buffer),
        Err(error) => Err(error),
    }
}

#[cfg(target_os = "windows")]
#[cfg(test)]
mod tests {
    use windows::Storage::Streams::DataWriter;

    use crate::windows::buffer::i_buffer_to_vec;

    #[test]
    fn test_i_buffer_to_vec() {
        let data = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9].to_vec();
        let data_writer = DataWriter::new().unwrap();
        data_writer.WriteBytes(&data).unwrap();
        let buffer = data_writer.DetachBuffer().unwrap();

        let result = i_buffer_to_vec(buffer);
        assert!(result.is_ok());
        assert_eq!(data.to_vec(), result.unwrap());
    }
}
