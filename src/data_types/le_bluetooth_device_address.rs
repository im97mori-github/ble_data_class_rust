//! LE Bluetooth Device Address (Data Type Value:0x17) module.

use crate::data_types::data_type::DataType;

/// LE Bluetooth Device Address.
pub struct LeBluetoothDeviceAddress {
    /// data length
    pub length: u8,

    /// LE Bluetooth Device Address
    pub le_bluetooth_device_address: u64,

    /// Address type
    pub address_type: bool,
}

impl LeBluetoothDeviceAddress {
    /// Create [LeBluetoothDeviceAddress] from Parameters.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_class::data_types::le_bluetooth_device_address::LeBluetoothDeviceAddress;
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

    /// Create [LeBluetoothDeviceAddress] from `Vec<u8>` with offset.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_class::data_types::{le_bluetooth_device_address::LeBluetoothDeviceAddress, data_type::DataType};
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
    /// let result = LeBluetoothDeviceAddress::from_with_offset(&data, 0);
    /// assert_eq!(length, result.length);
    /// assert_eq!(
    ///     le_bluetooth_device_address,
    ///     result.le_bluetooth_device_address
    /// );
    /// assert_eq!(address_type, result.address_type);
    /// 
    /// data = Vec::new();
    /// data.push(0);
    /// data.push(length);
    /// data.push(LeBluetoothDeviceAddress::data_type());
    /// data.append(&mut le_bluetooth_device_address.clone().to_le_bytes()[..6].to_vec());
    /// data.push(u8::from(address_type));
    /// let result = LeBluetoothDeviceAddress::from_with_offset(&data, 1);
    /// assert_eq!(length, result.length);
    /// assert_eq!(
    ///     le_bluetooth_device_address,
    ///     result.le_bluetooth_device_address
    /// );
    /// assert_eq!(address_type, result.address_type);
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
    /// let result = LeBluetoothDeviceAddress::from_with_offset(&data, 0);
    /// assert_eq!(length, result.length);
    /// assert_eq!(
    ///     le_bluetooth_device_address,
    ///     result.le_bluetooth_device_address
    /// );
    /// assert_eq!(address_type, result.address_type);
    /// 
    /// data = Vec::new();
    /// data.push(0);
    /// data.push(length);
    /// data.push(LeBluetoothDeviceAddress::data_type());
    /// data.append(&mut le_bluetooth_device_address.clone().to_le_bytes()[..6].to_vec());
    /// data.push(u8::from(address_type));
    /// let result = LeBluetoothDeviceAddress::from_with_offset(&data, 1);
    /// assert_eq!(length, result.length);
    /// assert_eq!(
    ///     le_bluetooth_device_address,
    ///     result.le_bluetooth_device_address
    /// );
    /// assert_eq!(address_type, result.address_type);
    /// ```
    pub fn from_with_offset(data: &Vec<u8>, offset: usize) -> Self {
        let data = data[offset..].to_vec();
        let length = data[0];
        let mut bytes = [0x00u8; 8];
        bytes[0] = data[2];
        bytes[1] = data[3];
        bytes[2] = data[4];
        bytes[3] = data[5];
        bytes[4] = data[6];
        bytes[5] = data[7];
        Self {
            length,
            le_bluetooth_device_address: u64::from_le_bytes(bytes),
            address_type: data[8] & ADDRESS_TYPE != 0,
        }
    }

    /// check Address type.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_class::data_types::le_bluetooth_device_address::LeBluetoothDeviceAddress;
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
    pub fn is_random_address(self) -> bool {
        self.address_type
    }
}

impl From<&Vec<u8>> for LeBluetoothDeviceAddress {
    /// Create [LeBluetoothDeviceAddress] from `Vec<u8>`.
    ///
    /// [`LeBluetoothDeviceAddress::from_with_offset`]
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_class::data_types::{le_bluetooth_device_address::LeBluetoothDeviceAddress, data_type::DataType};
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
    /// let result = LeBluetoothDeviceAddress::from(&data);
    /// assert_eq!(length, result.length);
    /// assert_eq!(
    ///     le_bluetooth_device_address,
    ///     result.le_bluetooth_device_address
    /// );
    /// assert_eq!(address_type, result.address_type);
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
    /// let result = LeBluetoothDeviceAddress::from(&data);
    /// assert_eq!(length, result.length);
    /// assert_eq!(
    ///     le_bluetooth_device_address,
    ///     result.le_bluetooth_device_address
    /// );
    /// assert_eq!(address_type, result.address_type);
    /// ```
    fn from(data: &Vec<u8>) -> Self {
        Self::from_with_offset(data, 0)
    }
}

/// Address type
/// (0 = Public Address, 1 = Random Address)
///
pub const ADDRESS_TYPE: u8 = 0b00000001;

impl Into<Vec<u8>> for LeBluetoothDeviceAddress {
    /// Create `Vec<u8>` from [LeBluetoothDeviceAddress].
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_class::data_types::{le_bluetooth_device_address::LeBluetoothDeviceAddress, data_type::DataType};
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
    /// let result2 = LeBluetoothDeviceAddress::from(&data);
    /// let into_data: Vec<u8> = result2.into();
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
    /// return `0x17`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_class::data_types::{le_bluetooth_device_address::LeBluetoothDeviceAddress, data_type::DataType};
    ///
    /// assert_eq!(0x17, LeBluetoothDeviceAddress::data_type());
    /// ```
    fn data_type() -> u8 {
        0x17
    }
}

/// check `LE Bluetooth Device Address` data type.
///
/// # Examples
///
/// ```
/// use ble_data_class::data_types::le_bluetooth_device_address::*;
/// use ble_data_class::data_types::data_type::DataType;
///
/// assert!(is_le_bluetooth_device_address(0x17));
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
    fn test_from_with_offset() {
        let le_bluetooth_device_address = 0x0000060504030201u64;
        let address_type = false;
        let length = 8;
        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(LeBluetoothDeviceAddress::data_type());
        data.append(&mut le_bluetooth_device_address.clone().to_le_bytes()[..6].to_vec());
        data.push(u8::from(address_type));

        let result = LeBluetoothDeviceAddress::from_with_offset(&data, 0);
        assert_eq!(length, result.length);
        assert_eq!(
            le_bluetooth_device_address,
            result.le_bluetooth_device_address
        );
        assert_eq!(address_type, result.address_type);

        data = Vec::new();
        data.push(0);
        data.push(length);
        data.push(LeBluetoothDeviceAddress::data_type());
        data.append(&mut le_bluetooth_device_address.clone().to_le_bytes()[..6].to_vec());
        data.push(u8::from(address_type));
        let result = LeBluetoothDeviceAddress::from_with_offset(&data, 1);
        assert_eq!(length, result.length);
        assert_eq!(
            le_bluetooth_device_address,
            result.le_bluetooth_device_address
        );
        assert_eq!(address_type, result.address_type);

        let le_bluetooth_device_address = 0x0000060504030201u64;
        let address_type = true;
        let length = 8;
        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(LeBluetoothDeviceAddress::data_type());
        data.append(&mut le_bluetooth_device_address.clone().to_le_bytes()[..6].to_vec());
        data.push(u8::from(address_type));

        let result = LeBluetoothDeviceAddress::from_with_offset(&data, 0);
        assert_eq!(length, result.length);
        assert_eq!(
            le_bluetooth_device_address,
            result.le_bluetooth_device_address
        );
        assert_eq!(address_type, result.address_type);

        data = Vec::new();
        data.push(0);
        data.push(length);
        data.push(LeBluetoothDeviceAddress::data_type());
        data.append(&mut le_bluetooth_device_address.clone().to_le_bytes()[..6].to_vec());
        data.push(u8::from(address_type));
        let result = LeBluetoothDeviceAddress::from_with_offset(&data, 1);
        assert_eq!(length, result.length);
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
    fn test_from() {
        let le_bluetooth_device_address = 0x0000060504030201u64;
        let address_type = false;
        let length = 8;
        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(LeBluetoothDeviceAddress::data_type());
        data.append(&mut le_bluetooth_device_address.clone().to_le_bytes()[..6].to_vec());
        data.push(u8::from(address_type));

        let result = LeBluetoothDeviceAddress::from(&data);
        assert_eq!(length, result.length);
        assert_eq!(
            le_bluetooth_device_address,
            result.le_bluetooth_device_address
        );
        assert_eq!(address_type, result.address_type);

        let le_bluetooth_device_address = 0x0000060504030201u64;
        let address_type = true;
        let length = 8;
        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(LeBluetoothDeviceAddress::data_type());
        data.append(&mut le_bluetooth_device_address.clone().to_le_bytes()[..6].to_vec());
        data.push(u8::from(address_type));

        let result = LeBluetoothDeviceAddress::from(&data);
        assert_eq!(length, result.length);
        assert_eq!(
            le_bluetooth_device_address,
            result.le_bluetooth_device_address
        );
        assert_eq!(address_type, result.address_type);
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

        let result2 = LeBluetoothDeviceAddress::from(&data);
        let into_data: Vec<u8> = result2.into();
        assert_eq!(data, into_data);
    }

    #[test]
    fn test_data_type() {
        assert_eq!(0x17, LeBluetoothDeviceAddress::data_type());
    }

    #[test]
    fn test_is_le_bluetooth_device_address() {
        assert!(is_le_bluetooth_device_address(0x17));
        assert!(!is_le_bluetooth_device_address(0x00));
    }
}
