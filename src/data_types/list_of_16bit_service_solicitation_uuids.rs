//! List of 16-bit Service Solicitation UUIDs (Data Type Value: 0x03) module.

use uuid::Uuid;

use crate::{data_types::data_type::DataType, BASE_UUID};

/// List of 16-bit Service Solicitation UUIDs.
#[derive(Debug)]
pub struct ListOf16BitServiceSolicitationUUIDs {
    /// data length
    pub length: u8,

    /// UUIDs
    pub uuids: Vec<Uuid>,
}

impl ListOf16BitServiceSolicitationUUIDs {
    /// Create [ListOf16BitServiceSolicitationUUIDs] from [`Vec<Uuid>`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::list_of_16bit_service_solicitation_uuids::ListOf16BitServiceSolicitationUUIDs;
    /// use uuid::{uuid, Uuid};
    ///
    /// let uuids: Vec<Uuid> = [
    ///     uuid!("00000001-0000-1000-8000-00805F9B34FB"),
    ///     uuid!("00000002-0000-1000-8000-00805F9B34FB"),
    /// ]
    /// .to_vec();
    /// let result = ListOf16BitServiceSolicitationUUIDs::new(&uuids);
    /// assert_eq!(uuids.len() as u8 * 2 + 1, result.length);
    /// assert_eq!(uuids, result.uuids);
    /// ```
    pub fn new(uuids: &Vec<Uuid>) -> Self {
        Self {
            length: (uuids.len() * 2 + 1) as u8,
            uuids: uuids.clone(),
        }
    }
}

impl TryFrom<&Vec<u8>> for ListOf16BitServiceSolicitationUUIDs {
    type Error = String;
    /// Create [ListOf16BitServiceSolicitationUUIDs] from `Vec<u8>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::{BASE_UUID, data_types::{list_of_16bit_service_solicitation_uuids::ListOf16BitServiceSolicitationUUIDs, data_type::DataType}};
    /// use uuid::{uuid, Uuid};
    ///
    /// let uuid_bytes: Vec<u8> = [0x01u8, 0x02u8, 0x03u8, 0x04u8].to_vec();
    /// let uuids: Vec<Uuid> = uuid_bytes
    ///     .windows(2)
    ///     .step_by(2)
    ///     .map(|f| {
    ///         let (d1, d2, d3, d4) = BASE_UUID.as_fields();
    ///         Uuid::from_fields(d1 | ((f[0] as u32) << 0) | ((f[1] as u32) << 8), d2, d3, d4)
    ///     })
    ///     .collect();
    /// let length = uuid_bytes.len() as u8 + 1;
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(length);
    /// data.push(ListOf16BitServiceSolicitationUUIDs::data_type());
    /// data.append(&mut uuid_bytes.clone());
    ///
    /// let result = ListOf16BitServiceSolicitationUUIDs::try_from(&data);
    /// assert!(result.is_ok());
    /// let data_type: ListOf16BitServiceSolicitationUUIDs = result.unwrap();
    /// assert_eq!(length, data_type.length);
    /// assert_eq!(uuids, data_type.uuids);
    ///
    /// let data: Vec<u8> = Vec::new();
    /// let result = ListOf16BitServiceSolicitationUUIDs::try_from(&data);
    /// assert!(result.is_err());
    /// assert_eq!(
    ///     format!("Invalid data size :{}", data.len()),
    ///     result.unwrap_err()
    /// );
    /// ```
    fn try_from(value: &Vec<u8>) -> Result<Self, String> {
        let len = value.len();
        if len < 4 {
            return Err(format!("Invalid data size :{}", len).to_string());
        }
        let length = value[0];
        Ok(Self {
            length,
            uuids: value[2..2 + length as usize - 1]
                .windows(2)
                .step_by(2)
                .map(|w| {
                    let mut bytes = BASE_UUID.to_bytes_le();
                    bytes[0] = w[0];
                    bytes[1] = w[1];
                    Uuid::from_bytes_le(bytes)
                })
                .collect(),
        })
    }
}

impl Into<Vec<u8>> for ListOf16BitServiceSolicitationUUIDs {
    /// Create `Vec<u8>` from [ListOf16BitServiceSolicitationUUIDs].
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::{BASE_UUID, data_types::{list_of_16bit_service_solicitation_uuids::ListOf16BitServiceSolicitationUUIDs, data_type::DataType}};
    /// use uuid::{uuid, Uuid};
    ///
    /// let uuid_bytes: Vec<u8> = [0x01u8, 0x02u8, 0x03u8, 0x04u8].to_vec();
    /// let uuids: Vec<Uuid> = uuid_bytes
    ///     .windows(2)
    ///     .step_by(2)
    ///     .map(|f| {
    ///         let (d1, d2, d3, d4) = BASE_UUID.as_fields();
    ///         Uuid::from_fields(d1 | ((f[0] as u32) << 0) | ((f[1] as u32) << 8), d2, d3, d4)
    ///     })
    ///     .collect();
    /// let result1 = ListOf16BitServiceSolicitationUUIDs::new(&uuids);

    /// let length = uuid_bytes.len() as u8 + 1;
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(length);
    /// data.push(ListOf16BitServiceSolicitationUUIDs::data_type());
    /// data.append(&mut uuid_bytes.clone());

    /// let into_data: Vec<u8> = result1.into();
    /// assert_eq!(data, into_data);

    /// let result2 = ListOf16BitServiceSolicitationUUIDs::try_from(&data);
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
                .flat_map(|f| f.to_bytes_le()[..2].to_vec())
                .collect(),
        );
        return data;
    }
}

impl DataType for ListOf16BitServiceSolicitationUUIDs {
    /// return `0x14`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{list_of_16bit_service_solicitation_uuids::ListOf16BitServiceSolicitationUUIDs, data_type::DataType};
    ///
    /// assert_eq!(0x14, ListOf16BitServiceSolicitationUUIDs::data_type());
    /// ```
    fn data_type() -> u8 {
        0x14
    }
}

/// check `List of 16-bit Service Solicitation UUIDs.` data type.
///
/// # Examples
///
/// ```
/// use ble_data_struct::data_types::list_of_16bit_service_solicitation_uuids::*;
/// use ble_data_struct::data_types::data_type::DataType;
///
/// assert!(is_list_of_16bit_service_solicitation_uuids(0x14));
/// assert!(!is_list_of_16bit_service_solicitation_uuids(0x00));
/// ```
pub fn is_list_of_16bit_service_solicitation_uuids(data_type: u8) -> bool {
    ListOf16BitServiceSolicitationUUIDs::data_type() == data_type
}

#[cfg(test)]
mod tests {
    use uuid::{uuid, Uuid};

    use crate::{
        data_types::{data_type::DataType, list_of_16bit_service_solicitation_uuids::*},
        BASE_UUID,
    };

    #[test]
    fn test_new() {
        let uuids: Vec<Uuid> = [
            uuid!("00000001-0000-1000-8000-00805F9B34FB"),
            uuid!("00000002-0000-1000-8000-00805F9B34FB"),
        ]
        .to_vec();
        let result = ListOf16BitServiceSolicitationUUIDs::new(&uuids);
        assert_eq!(uuids.len() as u8 * 2 + 1, result.length);
        assert_eq!(uuids, result.uuids);
    }

    #[test]
    fn test_try_from() {
        let uuid_bytes: Vec<u8> = [0x01u8, 0x02u8, 0x03u8, 0x04u8].to_vec();
        let uuids: Vec<Uuid> = uuid_bytes
            .windows(2)
            .step_by(2)
            .map(|f| {
                let (d1, d2, d3, d4) = BASE_UUID.as_fields();
                Uuid::from_fields(d1 | ((f[0] as u32) << 0) | ((f[1] as u32) << 8), d2, d3, d4)
            })
            .collect();
        let length = uuid_bytes.len() as u8 + 1;
        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(ListOf16BitServiceSolicitationUUIDs::data_type());
        data.append(&mut uuid_bytes.clone());

        let result = ListOf16BitServiceSolicitationUUIDs::try_from(&data);
        assert!(result.is_ok());
        let data_type: ListOf16BitServiceSolicitationUUIDs = result.unwrap();
        assert_eq!(length, data_type.length);
        assert_eq!(uuids, data_type.uuids);

        let mut data: Vec<u8> = vec![0u8; 3];
        data[0] = data.len() as u8 - 1;
        let result = ListOf16BitServiceSolicitationUUIDs::try_from(&data);
        assert!(result.is_err());
        assert_eq!(
            format!("Invalid data size :{}", data.len()),
            result.unwrap_err()
        );
    }

    #[test]
    fn test_into() {
        let uuid_bytes: Vec<u8> = [0x01u8, 0x02u8, 0x03u8, 0x04u8].to_vec();
        let uuids: Vec<Uuid> = uuid_bytes
            .windows(2)
            .step_by(2)
            .map(|f| {
                let (d1, d2, d3, d4) = BASE_UUID.as_fields();
                Uuid::from_fields(d1 | ((f[0] as u32) << 0) | ((f[1] as u32) << 8), d2, d3, d4)
            })
            .collect();
        let result1 = ListOf16BitServiceSolicitationUUIDs::new(&uuids);

        let length = uuid_bytes.len() as u8 + 1;
        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(ListOf16BitServiceSolicitationUUIDs::data_type());
        data.append(&mut uuid_bytes.clone());

        let into_data: Vec<u8> = result1.into();
        assert_eq!(data, into_data);

        let result2 = ListOf16BitServiceSolicitationUUIDs::try_from(&data);
        assert!(result2.is_ok());
        let data_type = result2.unwrap();
        let into_data: Vec<u8> = data_type.into();
        assert_eq!(data, into_data);
    }

    #[test]
    fn test_data_type() {
        assert_eq!(0x14, ListOf16BitServiceSolicitationUUIDs::data_type());
    }

    #[test]
    fn test_is_list_of_16bit_service_solicitation_uuids() {
        assert!(is_list_of_16bit_service_solicitation_uuids(0x14));
        assert!(!is_list_of_16bit_service_solicitation_uuids(0x00));
    }
}
