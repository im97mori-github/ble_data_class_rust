//! LE Bluetooth Device Address (Data Type Value:0x1b) module.

use crate::data_types::data_type::DataType;

/// LE Bluetooth Device Address.

#[derive(Debug)]
pub struct LeBluetoothDeviceAddress {
    /// data length
    pub length: u8,

    /// LE Bluetooth Device Address
    pub le_bluetooth_device_address: u64,

    /// Address type
    pub address_type: bool,
}

impl LeBluetoothDeviceAddress {
    /// Create [`LeBluetoothDeviceAddress`] from Parameters.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::le_bluetooth_device_address::LeBluetoothDeviceAddress;
    ///
    /// let le_bluetooth_device_address = 0x0000060504030201u64;
    /// let address_type = false;
    /// let result = LeBluetoothDeviceAddress::new(le_bluetooth_device_address, address_type);
    /// assert_eq!(8, result.length);
    /// assert_eq!(
    ///     le_bluetooth_device_address,
    ///     result.le_bluetooth_device_address
    /// );
    /// assert_eq!(address_type, result.address_type);
    ///
    /// let le_bluetooth_device_address = 0x0000060504030201u64;
    /// let address_type = true;
    /// let result = LeBluetoothDeviceAddress::new(le_bluetooth_device_address, address_type);
    /// assert_eq!(8, result.length);
    /// assert_eq!(
    ///     le_bluetooth_device_address,
    ///     result.le_bluetooth_device_address
    /// );
    /// assert_eq!(address_type, result.address_type);
    /// ```
    pub fn new(le_bluetooth_device_address: u64, address_type: bool) -> Self {
        Self {
            length: 8,
            le_bluetooth_device_address,
            address_type,
        }
    }

    /// check Address type.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::le_bluetooth_device_address::LeBluetoothDeviceAddress;
    ///
    /// let le_bluetooth_device_address = 0x0000060504030201u64;
    /// let address_type = false;
    /// let result = LeBluetoothDeviceAddress::new(le_bluetooth_device_address, address_type);
    /// assert_eq!(address_type, result.is_random_address());
    ///
    /// let le_bluetooth_device_address = 0x0000060504030201u64;
    /// let address_type = true;
    /// let result = LeBluetoothDeviceAddress::new(le_bluetooth_device_address, address_type);
    /// assert_eq!(address_type, result.is_random_address());
    /// ```
    pub const fn is_random_address(&self) -> bool {
        self.address_type
    }
}

impl TryFrom<&Vec<u8>> for LeBluetoothDeviceAddress {
    type Error = String;
    /// Create [`LeBluetoothDeviceAddress`] from `Vec<u8>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{le_bluetooth_device_address::LeBluetoothDeviceAddress, data_type::DataType};
    ///
    /// let le_bluetooth_device_address = 0x0000060504030201u64;
    /// let address_type = false;
    /// let length = 8;
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(length);
    /// data.push(LeBluetoothDeviceAddress::data_type());
    /// data.append(&mut le_bluetooth_device_address.clone().to_le_bytes()[..6].to_vec());
    /// data.push(u8::from(address_type));
    ///
    /// let result = LeBluetoothDeviceAddress::try_from(&data);
    /// assert!(result.is_ok());
    /// let data_type = result.unwrap();
    /// assert_eq!(length, data_type.length);
    /// assert_eq!(
    ///     le_bluetooth_device_address,
    ///     data_type.le_bluetooth_device_address
    /// );
    /// assert_eq!(address_type, data_type.address_type);
    ///
    /// let le_bluetooth_device_address = 0x0000060504030201u64;
    /// let address_type = true;
    /// let length = 8;
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(length);
    /// data.push(LeBluetoothDeviceAddress::data_type());
    /// data.append(&mut le_bluetooth_device_address.clone().to_le_bytes()[..6].to_vec());
    /// data.push(u8::from(address_type));
    ///
    /// let result = LeBluetoothDeviceAddress::try_from(&data);
    /// assert!(result.is_ok());
    /// let data_type = result.unwrap();
    /// assert_eq!(length, data_type.length);
    /// assert_eq!(
    ///     le_bluetooth_device_address,
    ///     data_type.le_bluetooth_device_address
    /// );
    /// assert_eq!(address_type, data_type.address_type);
    ///
    /// let data: Vec<u8> = Vec::new();
    /// let result = LeBluetoothDeviceAddress::try_from(&data);
    /// assert!(result.is_err());
    /// assert_eq!(
    ///     format!("Invalid data size :{}", data.len()),
    ///     result.unwrap_err()
    /// );
    /// ```
    fn try_from(value: &Vec<u8>) -> Result<Self, String> {
        let len = value.len();
        if len < 8 {
            return Err(format!("Invalid data size :{}", len).to_string());
        }
        let length = value[0];
        let mut bytes = [0x00u8; 8];
        bytes[0] = value[2];
        bytes[1] = value[3];
        bytes[2] = value[4];
        bytes[3] = value[5];
        bytes[4] = value[6];
        bytes[5] = value[7];
        Ok(Self {
            length,
            le_bluetooth_device_address: u64::from_le_bytes(bytes),
            address_type: value[8] & ADDRESS_TYPE != 0,
        })
    }
}

/// Address type
/// (0 = Public Address, 1 = Random Address)
///
pub const ADDRESS_TYPE: u8 = 0b00000001;

impl Into<Vec<u8>> for LeBluetoothDeviceAddress {
    /// Create `Vec<u8>` from [`LeBluetoothDeviceAddress`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{le_bluetooth_device_address::LeBluetoothDeviceAddress, data_type::DataType};
    ///
    /// let le_bluetooth_device_address = 0x0000060504030201u64;
    /// let address_type = false;
    /// let result1 = LeBluetoothDeviceAddress::new(le_bluetooth_device_address, address_type);
    ///
    /// let length = 8;
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(length);
    /// data.push(LeBluetoothDeviceAddress::data_type());
    /// data.append(&mut le_bluetooth_device_address.clone().to_le_bytes()[..6].to_vec());
    /// data.push(u8::from(address_type));
    ///
    /// let into_data: Vec<u8> = result1.into();
    /// assert_eq!(data, into_data);
    ///
    /// let result2 = LeBluetoothDeviceAddress::try_from(&data);
    /// assert!(result2.is_ok());
    /// let data_type = result2.unwrap();
    /// let into_data: Vec<u8> = data_type.into();
    /// assert_eq!(data, into_data);
    /// ```
    fn into(self) -> Vec<u8> {
        let mut data: Vec<u8> = Vec::new();
        data.push(self.length);
        data.push(Self::data_type());
        data.append(&mut self.le_bluetooth_device_address.clone().to_le_bytes()[..6].to_vec());
        data.push(u8::from(self.address_type));
        return data;
    }
}

impl DataType for LeBluetoothDeviceAddress {
    /// return `0x1b`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{le_bluetooth_device_address::LeBluetoothDeviceAddress, data_type::DataType};
    ///
    /// assert_eq!(0x1b, LeBluetoothDeviceAddress::data_type());
    /// ```
    fn data_type() -> u8 {
        0x1b
    }
}

/// check `LE Bluetooth Device Address` data type.
///
/// # Examples
///
/// ```
/// use ble_data_struct::data_types::le_bluetooth_device_address::*;
/// use ble_data_struct::data_types::data_type::DataType;
///
/// assert!(is_le_bluetooth_device_address(0x1b));
/// assert!(!is_le_bluetooth_device_address(0x00));
/// ```
pub fn is_le_bluetooth_device_address(data_type: u8) -> bool {
    LeBluetoothDeviceAddress::data_type() == data_type
}

#[cfg(test)]
mod tests {
    use crate::data_types::{data_type::DataType, le_bluetooth_device_address::*};

    #[test]
    fn test_new() {
        let le_bluetooth_device_address = 0x0000060504030201u64;
        let address_type = false;
        let result = LeBluetoothDeviceAddress::new(le_bluetooth_device_address, address_type);
        assert_eq!(8, result.length);
        assert_eq!(
            le_bluetooth_device_address,
            result.le_bluetooth_device_address
        );
        assert_eq!(address_type, result.address_type);

        let le_bluetooth_device_address = 0x0000060504030201u64;
        let address_type = true;
        let result = LeBluetoothDeviceAddress::new(le_bluetooth_device_address, address_type);
        assert_eq!(8, result.length);
        assert_eq!(
            le_bluetooth_device_address,
            result.le_bluetooth_device_address
        );
        assert_eq!(address_type, result.address_type);
    }

    #[test]
    fn test_is_random_address() {
        let le_bluetooth_device_address = 0x0000060504030201u64;
        let address_type = false;
        let result = LeBluetoothDeviceAddress::new(le_bluetooth_device_address, address_type);
        assert_eq!(address_type, result.is_random_address());

        let le_bluetooth_device_address = 0x0000060504030201u64;
        let address_type = true;
        let result = LeBluetoothDeviceAddress::new(le_bluetooth_device_address, address_type);
        assert_eq!(address_type, result.is_random_address());
    }

    #[test]
    fn test_try_from() {
        let le_bluetooth_device_address = 0x0000060504030201u64;
        let address_type = false;
        let length = 8;
        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(LeBluetoothDeviceAddress::data_type());
        data.append(&mut le_bluetooth_device_address.clone().to_le_bytes()[..6].to_vec());
        data.push(u8::from(address_type));

        let result = LeBluetoothDeviceAddress::try_from(&data);
        assert!(result.is_ok());
        let data_type = result.unwrap();
        assert_eq!(length, data_type.length);
        assert_eq!(
            le_bluetooth_device_address,
            data_type.le_bluetooth_device_address
        );
        assert_eq!(address_type, data_type.address_type);

        let le_bluetooth_device_address = 0x0000060504030201u64;
        let address_type = true;
        let length = 8;
        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(LeBluetoothDeviceAddress::data_type());
        data.append(&mut le_bluetooth_device_address.clone().to_le_bytes()[..6].to_vec());
        data.push(u8::from(address_type));

        let result = LeBluetoothDeviceAddress::try_from(&data);
        assert!(result.is_ok());
        let data_type = result.unwrap();
        assert_eq!(length, data_type.length);
        assert_eq!(
            le_bluetooth_device_address,
            data_type.le_bluetooth_device_address
        );
        assert_eq!(address_type, data_type.address_type);

        let data: Vec<u8> = Vec::new();
        let result = LeBluetoothDeviceAddress::try_from(&data);
        assert!(result.is_err());
        assert_eq!(
            format!("Invalid data size :{}", data.len()),
            result.unwrap_err()
        );
    }

    #[test]
    fn test_into() {
        let le_bluetooth_device_address = 0x0000060504030201u64;
        let address_type = false;
        let result1 = LeBluetoothDeviceAddress::new(le_bluetooth_device_address, address_type);

        let length = 8;
        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(LeBluetoothDeviceAddress::data_type());
        data.append(&mut le_bluetooth_device_address.clone().to_le_bytes()[..6].to_vec());
        data.push(u8::from(address_type));

        let into_data: Vec<u8> = result1.into();
        assert_eq!(data, into_data);

        let result2 = LeBluetoothDeviceAddress::try_from(&data);
        assert!(result2.is_ok());
        let data_type = result2.unwrap();
        let into_data: Vec<u8> = data_type.into();
        assert_eq!(data, into_data);
    }

    #[test]
    fn test_data_type() {
        assert_eq!(0x1b, LeBluetoothDeviceAddress::data_type());
    }

    #[test]
    fn test_is_le_bluetooth_device_address() {
        assert!(is_le_bluetooth_device_address(0x1b));
        assert!(!is_le_bluetooth_device_address(0x00));
    }
}
