//! Incomplete List of 128-bit Service Class UUIDs (Data Type Value: 0x06) module.

use uuid::Uuid;

use crate::data_types::data_type::DataType;

/// Incomplete List of 128-bit Service Class UUIDs.
#[derive(Debug)]
pub struct IncompleteListOf128BitServiceUuids {
    /// data length
    pub length: u8,

    /// UUIDs
    pub uuids: Vec<Uuid>,
}

impl IncompleteListOf128BitServiceUuids {
    /// Create [`IncompleteListOf128BitServiceUuids`] from [`Vec<Uuid>`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::incomplete_list_of_128bit_service_uuids::IncompleteListOf128BitServiceUuids;
    /// use uuid::{uuid, Uuid};
    ///
    /// let uuids: Vec<Uuid> = [
    ///     uuid!("00000001-0000-1000-8000-00805F9B34FB"),
    ///     uuid!("00000002-0000-1000-8000-00805F9B34FB"),
    /// ]
    /// .to_vec();
    /// let result = IncompleteListOf128BitServiceUuids::new(&uuids);
    /// assert_eq!(uuids.len() as u8 * 16 + 1, result.length);
    /// assert_eq!(uuids, result.uuids);
    /// ```
    pub fn new(uuids: &Vec<Uuid>) -> Self {
        Self {
            length: (uuids.len() * 16 + 1) as u8,
            uuids: uuids.clone(),
        }
    }
}

impl TryFrom<&Vec<u8>> for IncompleteListOf128BitServiceUuids {
    type Error = String;
    /// Create [`IncompleteListOf128BitServiceUuids`] from `Vec<u8>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::{BASE_UUID, data_types::{incomplete_list_of_128bit_service_uuids::IncompleteListOf128BitServiceUuids, data_type::DataType}};
    /// use uuid::{uuid, Uuid};
    ///
    /// let uuid_bytes: Vec<u8> = [
    ///     0x01u8, 0x02u8, 0x03u8, 0x04u8, 0x05u8, 0x06u8, 0x07u8, 0x08u8, 0x09u8, 0x0au8, 0x0bu8,
    ///     0x0cu8, 0x0du8, 0x0eu8, 0x0fu8, 0x10u8, 0x11u8, 0x12u8, 0x13u8, 0x14u8, 0x15u8, 0x16u8,
    ///     0x17u8, 0x18u8, 0x19u8, 0x1au8, 0x1bu8, 0x1cu8, 0x1du8, 0x1eu8, 0x1fu8, 0x20u8,
    /// ]
    /// .to_vec();
    /// let uuids: Vec<Uuid> = uuid_bytes
    ///     .windows(16)
    ///     .step_by(16)
    ///     .map(|f| Uuid::from_bytes_le(f.try_into().unwrap()))
    ///     .collect();
    /// let length = uuid_bytes.len() as u8 + 1;
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(length);
    /// data.push(IncompleteListOf128BitServiceUuids::data_type());
    /// data.append(&mut uuid_bytes.clone());
    ///
    /// let result = IncompleteListOf128BitServiceUuids::try_from(&data);
    /// assert!(result.is_ok());
    /// let data_type = result.unwrap();
    /// assert_eq!(length, data_type.length);
    /// assert_eq!(uuids, data_type.uuids);
    ///
    /// let data: Vec<u8> = Vec::new();
    /// let result = IncompleteListOf128BitServiceUuids::try_from(&data);
    /// assert!(result.is_err());
    /// assert_eq!(
    ///     format!("Invalid data size :{}", data.len()),
    ///     result.unwrap_err()
    /// );
    /// ```
    fn try_from(value: &Vec<u8>) -> Result<Self, String> {
        let len = value.len();
        if len < 17 {
            return Err(format!("Invalid data size :{}", len).to_string());
        }
        let length = value[0];
        Ok(Self {
            length,
            uuids: value[2..2 + length as usize - 1]
                .windows(16)
                .step_by(16)
                .map(|w| Uuid::from_bytes_le(w.try_into().unwrap()))
                .collect(),
        })
    }
}

impl Into<Vec<u8>> for IncompleteListOf128BitServiceUuids {
    /// Create `Vec<u8>` from [`IncompleteListOf128BitServiceUuids`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::{BASE_UUID, data_types::{incomplete_list_of_128bit_service_uuids::IncompleteListOf128BitServiceUuids, data_type::DataType}};
    /// use uuid::{uuid, Uuid};
    ///
    /// let uuid_bytes: Vec<u8> = [
    ///     0x01u8, 0x02u8, 0x03u8, 0x04u8, 0x05u8, 0x06u8, 0x07u8, 0x08u8, 0x09u8, 0x0au8, 0x0bu8,
    ///     0x0cu8, 0x0du8, 0x0eu8, 0x0fu8, 0x10u8, 0x11u8, 0x12u8, 0x13u8, 0x14u8, 0x15u8, 0x16u8,
    ///     0x17u8, 0x18u8, 0x19u8, 0x1au8, 0x1bu8, 0x1cu8, 0x1du8, 0x1eu8, 0x1fu8, 0x20u8,
    /// ]
    /// .to_vec();
    /// let uuids: Vec<Uuid> = uuid_bytes
    ///     .windows(16)
    ///     .step_by(16)
    ///     .map(|f| Uuid::from_bytes_le(f.try_into().unwrap()))
    ///     .collect();
    /// let result1 = IncompleteListOf128BitServiceUuids::new(&uuids);
    ///
    /// let length = uuid_bytes.len() as u8 + 1;
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(length);
    /// data.push(IncompleteListOf128BitServiceUuids::data_type());
    /// data.append(&mut uuid_bytes.clone());
    ///
    /// let into_data: Vec<u8> = result1.into();
    /// assert_eq!(data, into_data);
    ///
    /// let result2 = IncompleteListOf128BitServiceUuids::try_from(&data);
    /// assert!(result2.is_ok());
    /// let data_type = result2.unwrap();
    /// let into_data: Vec<u8> = data_type.into();
    /// assert_eq!(data, into_data);
    /// ```
    fn into(self) -> Vec<u8> {
        let mut data: Vec<u8> = Vec::new();
        data.push(self.length);
        data.push(Self::data_type());
        data.append(
            &mut self
                .uuids
                .clone()
                .iter()
                .flat_map(|f| f.to_bytes_le().to_vec())
                .collect(),
        );
        return data;
    }
}

impl DataType for IncompleteListOf128BitServiceUuids {
    /// return `0x07`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{incomplete_list_of_128bit_service_uuids::IncompleteListOf128BitServiceUuids, data_type::DataType};
    ///
    /// assert_eq!(0x06, IncompleteListOf128BitServiceUuids::data_type());
    /// ```
    fn data_type() -> u8 {
        0x06
    }
}

/// check `Incomplete List of 128-bit Service Class UUIDs.` data type.
///
/// # Examples
///
/// ```
/// use ble_data_struct::data_types::incomplete_list_of_128bit_service_uuids::*;
/// use ble_data_struct::data_types::data_type::DataType;
///
/// assert!(is_incomplete_list_of_128bit_service_uuids(0x06));
/// assert!(!is_incomplete_list_of_128bit_service_uuids(0x00));
/// ```
pub fn is_incomplete_list_of_128bit_service_uuids(data_type: u8) -> bool {
    IncompleteListOf128BitServiceUuids::data_type() == data_type
}

#[cfg(test)]
mod tests {
    use uuid::{uuid, Uuid};

    use crate::data_types::{data_type::DataType, incomplete_list_of_128bit_service_uuids::*};

    #[test]
    fn test_new() {
        let uuids: Vec<Uuid> = [
            uuid!("00000001-0000-1000-8000-00805F9B34FB"),
            uuid!("00000002-0000-1000-8000-00805F9B34FB"),
        ]
        .to_vec();
        let result = IncompleteListOf128BitServiceUuids::new(&uuids);
        assert_eq!(uuids.len() as u8 * 16 + 1, result.length);
        assert_eq!(uuids, result.uuids);
    }

    #[test]
    fn test_try_from() {
        let uuid_bytes: Vec<u8> = [
            0x01u8, 0x02u8, 0x03u8, 0x04u8, 0x05u8, 0x06u8, 0x07u8, 0x08u8, 0x09u8, 0x0au8, 0x0bu8,
            0x0cu8, 0x0du8, 0x0eu8, 0x0fu8, 0x10u8, 0x11u8, 0x12u8, 0x13u8, 0x14u8, 0x15u8, 0x16u8,
            0x17u8, 0x18u8, 0x19u8, 0x1au8, 0x1bu8, 0x1cu8, 0x1du8, 0x1eu8, 0x1fu8, 0x20u8,
        ]
        .to_vec();
        let uuids: Vec<Uuid> = uuid_bytes
            .windows(16)
            .step_by(16)
            .map(|f| Uuid::from_bytes_le(f.try_into().unwrap()))
            .collect();
        let length = uuid_bytes.len() as u8 + 1;
        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(IncompleteListOf128BitServiceUuids::data_type());
        data.append(&mut uuid_bytes.clone());

        let result = IncompleteListOf128BitServiceUuids::try_from(&data);
        assert!(result.is_ok());
        let data_type = result.unwrap();
        assert_eq!(length, data_type.length);
        assert_eq!(uuids, data_type.uuids);

        let data: Vec<u8> = Vec::new();
        let result = IncompleteListOf128BitServiceUuids::try_from(&data);
        assert!(result.is_err());
        assert_eq!(
            format!("Invalid data size :{}", data.len()),
            result.unwrap_err()
        );
    }

    #[test]
    fn test_into() {
        let uuid_bytes: Vec<u8> = [
            0x01u8, 0x02u8, 0x03u8, 0x04u8, 0x05u8, 0x06u8, 0x07u8, 0x08u8, 0x09u8, 0x0au8, 0x0bu8,
            0x0cu8, 0x0du8, 0x0eu8, 0x0fu8, 0x10u8, 0x11u8, 0x12u8, 0x13u8, 0x14u8, 0x15u8, 0x16u8,
            0x17u8, 0x18u8, 0x19u8, 0x1au8, 0x1bu8, 0x1cu8, 0x1du8, 0x1eu8, 0x1fu8, 0x20u8,
        ]
        .to_vec();
        let uuids: Vec<Uuid> = uuid_bytes
            .windows(16)
            .step_by(16)
            .map(|f| Uuid::from_bytes_le(f.try_into().unwrap()))
            .collect();
        let result1 = IncompleteListOf128BitServiceUuids::new(&uuids);

        let length = uuid_bytes.len() as u8 + 1;
        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(IncompleteListOf128BitServiceUuids::data_type());
        data.append(&mut uuid_bytes.clone());

        let into_data: Vec<u8> = result1.into();
        assert_eq!(data, into_data);

        let result2 = IncompleteListOf128BitServiceUuids::try_from(&data);
        assert!(result2.is_ok());
        let data_type = result2.unwrap();
        let into_data: Vec<u8> = data_type.into();
        assert_eq!(data, into_data);
    }

    #[test]
    fn test_data_type() {
        assert_eq!(0x06, IncompleteListOf128BitServiceUuids::data_type());
    }

    #[test]
    fn test_is_incomplete_list_of_128bit_service_uuids() {
        assert!(is_incomplete_list_of_128bit_service_uuids(0x06));
        assert!(!is_incomplete_list_of_128bit_service_uuids(0x00));
    }
}
