#[cfg(target_os = "windows")]
use windows::{
    core::Error,
    Storage::Streams::{DataReader, DataWriter, IBuffer},
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

/// Convert [`Vec<u8>`] to [`IBuffer`].
///
/// # Examples
///
/// ```
/// use windows::Storage::Streams::{DataReader, DataWriter};
/// use ble_data_struct::windows::buffer::vec_to_i_buffer;
///
/// let data = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9].to_vec();
/// 
/// let result = vec_to_i_buffer(&data);
/// assert!(result.is_ok());
/// 
/// let i_buffer = result.unwrap();
/// let reader = DataReader::FromBuffer(&i_buffer).unwrap();
/// let length = reader.UnconsumedBufferLength().unwrap();
/// let mut read_buffer = vec![0u8; length as usize];
/// reader.ReadBytes(read_buffer.as_mut()).unwrap();
/// assert_eq!(data.to_vec(), read_buffer);
/// ```
#[cfg(target_os = "windows")]
pub fn vec_to_i_buffer(vec: &Vec<u8>) -> Result<IBuffer, Error> {
    let data_writer = match DataWriter::new() {
        Ok(writer) => writer,
        Err(error) => return Err(error),
    };

    match data_writer.WriteBytes(&vec) {
        Err(error) => return Err(error),
        _ => {}
    }

    match data_writer.DetachBuffer() {
        Ok(buffer) => Ok(buffer),
        Err(error) => Err(error),
    }
}

#[cfg(target_os = "windows")]
#[cfg(test)]
mod tests {
    use windows::Storage::Streams::{DataReader, DataWriter};

    use crate::windows::buffer::{i_buffer_to_vec, vec_to_i_buffer};

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

    #[test]
    fn test_vec_to_i_buffer() {
        let data = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9].to_vec();

        let result = vec_to_i_buffer(&data);
        assert!(result.is_ok());

        let i_buffer = result.unwrap();
        let reader = DataReader::FromBuffer(&i_buffer).unwrap();
        let length = reader.UnconsumedBufferLength().unwrap();
        let mut read_buffer = vec![0u8; length as usize];
        reader.ReadBytes(read_buffer.as_mut()).unwrap();
        assert_eq!(data.to_vec(), read_buffer);
    }
}
