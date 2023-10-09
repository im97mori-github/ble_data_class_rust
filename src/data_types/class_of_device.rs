//! Class of Device (Data Type Value: 0x0d) module.

use crate::data_types::data_type::DataType;

/// Class of Device.
#[derive(Debug, PartialEq, Clone)]
pub struct ClassOfDevice {
    /// data length
    pub length: u8,

    /// Class of Device
    pub class_of_device: u32,
}

impl ClassOfDevice {
    /// Create [`ClassOfDevice`] from `Class of Device`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::class_of_device::ClassOfDevice;
    ///
    /// let name = "class_of_device".to_string();
    /// let major_service_classes = 0b10000000_00000000_00000000;
    /// let major_device_class = 0b00000000_00000001_00000000;
    /// let minor_device_class = 0b00000000_00000000_00000100;
    /// let class_of_device = major_service_classes | major_device_class | minor_device_class;
    /// let result = ClassOfDevice::new(class_of_device);
    /// assert_eq!(4, result.length);
    /// assert_eq!(class_of_device, result.class_of_device);
    /// ```
    pub fn new(class_of_device: u32) -> Self {
        Self {
            length: 4,
            class_of_device,
        }
    }

    /// Major Service Classes.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{class_of_device::ClassOfDevice, data_type::DataType};
    ///
    /// let major_service_classes = 0b10000000_00000000_00000000;
    /// let major_device_class = 0b00000000_00000001_00000000;
    /// let minor_device_class = 0b00000000_00000000_00000100;
    /// let class_of_device = major_service_classes | major_device_class | minor_device_class;
    /// let result = ClassOfDevice::new(class_of_device);
    /// assert_eq!(4, result.length);
    /// assert_eq!(class_of_device, result.class_of_device);
    /// assert_eq!(major_service_classes, result.major_service_classes());
    /// ```
    pub const fn major_service_classes(&self) -> u32 {
        self.class_of_device & CLASS_OF_DEVICE_MAJOR_SERVICE_CLASSES_MASK
    }

    /// Major Device Class.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::class_of_device::ClassOfDevice;
    ///
    /// let major_service_classes = 0b10000000_00000000_00000000;
    /// let major_device_class = 0b00000000_00000001_00000000;
    /// let minor_device_class = 0b00000000_00000000_00000100;
    /// let class_of_device = major_service_classes | major_device_class | minor_device_class;
    /// let result = ClassOfDevice::new(class_of_device);
    /// assert_eq!(4, result.length);
    /// assert_eq!(class_of_device, result.class_of_device);
    /// assert_eq!(major_device_class, result.major_device_class());
    /// ```
    pub const fn major_device_class(&self) -> u32 {
        self.class_of_device & CLASS_OF_DEVICE_MAJOR_DEVICE_CLASS_MASK
    }

    /// Minor Device Class.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::class_of_device::ClassOfDevice;
    ///
    /// let major_service_classes = 0b10000000_00000000_00000000;
    /// let major_device_class = 0b00000000_00000001_00000000;
    /// let minor_device_class = 0b00000000_00000000_00000100;
    /// let class_of_device = major_service_classes | major_device_class | minor_device_class;
    /// let result = ClassOfDevice::new(class_of_device);
    /// assert_eq!(4, result.length);
    /// assert_eq!(class_of_device, result.class_of_device);
    /// assert_eq!(minor_device_class, result.minor_device_class());
    /// ```
    pub const fn minor_device_class(&self) -> u32 {
        self.class_of_device & CLASS_OF_DEVICE_MINOR_DEVICE_CLASS_MASK
    }
}

/// Major Service Classes mask
pub const CLASS_OF_DEVICE_MAJOR_SERVICE_CLASSES_MASK: u32 = 0b11111111_11100000_00000000;

/// Major Device Class mask
pub const CLASS_OF_DEVICE_MAJOR_DEVICE_CLASS_MASK: u32 = 0b00000000_00011111_00000000;

/// Minor Device Class mask
pub const CLASS_OF_DEVICE_MINOR_DEVICE_CLASS_MASK: u32 = 0b00000000_00000000_11111100;

impl TryFrom<&Vec<u8>> for ClassOfDevice {
    type Error = String;
    /// Create [`ClassOfDevice`] from [`Vec<u8>`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{class_of_device::ClassOfDevice, data_type::DataType};
    ///
    /// let major_service_classes = 0b10000000_00000000_00000000;
    /// let major_device_class = 0b00000000_00000001_00000000;
    /// let minor_device_class = 0b00000000_00000000_00000100;
    /// let class_of_device = major_service_classes | major_device_class | minor_device_class;
    /// let length = 4;
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(length);
    /// data.push(ClassOfDevice::data_type());
    /// data.push(class_of_device as u8);
    /// data.push((class_of_device >> 8) as u8);
    /// data.push((class_of_device >> 16) as u8);
    ///
    /// let result = ClassOfDevice::try_from(&data);
    /// assert!(result.is_ok());
    /// let data_type = result.unwrap();
    /// assert_eq!(length, data_type.length);
    /// assert_eq!(class_of_device, data_type.class_of_device);
    ///
    /// let data: Vec<u8> = Vec::new();
    /// let result = ClassOfDevice::try_from(&data);
    /// assert!(result.is_err());
    /// assert_eq!(
    ///     format!("Invalid data size :{}", data.len()),
    ///     result.unwrap_err()
    /// );
    /// ```
    fn try_from(value: &Vec<u8>) -> Result<Self, String> {
        let len = value.len();
        if len < 5 {
            return Err(format!("Invalid data size :{}", len).to_string());
        }
        let mut value = value.to_vec();
        let length = value[0];
        value.push(0);
        Ok(Self {
            length,
            class_of_device: u32::from_le_bytes(value[2..6].try_into().unwrap()) & 0x00ffffff,
        })
    }
}

impl Into<Vec<u8>> for ClassOfDevice {
    /// Create [`Vec<u8>`] from [`ClassOfDevice`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{class_of_device::ClassOfDevice, data_type::DataType};
    ///
    /// let major_service_classes = 0b10000000_00000000_00000000;
    /// let major_device_class = 0b00000000_00000001_00000000;
    /// let minor_device_class = 0b00000000_00000000_00000100;
    /// let class_of_device = major_service_classes | major_device_class | minor_device_class;
    /// let result1 = ClassOfDevice::new(class_of_device);
    ///
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(4);
    /// data.push(ClassOfDevice::data_type());
    /// data.push(class_of_device as u8);
    /// data.push((class_of_device >> 8) as u8);
    /// data.push((class_of_device >> 16) as u8);
    ///
    /// let into_data: Vec<u8> = result1.into();
    /// assert_eq!(data, into_data);
    ///
    /// let result2 = ClassOfDevice::try_from(&data);
    /// assert!(result2.is_ok());
    /// let data_type = result2.unwrap();
    /// let into_data: Vec<u8> = data_type.into();
    /// assert_eq!(data, into_data);
    /// ```
    fn into(self) -> Vec<u8> {
        let mut data: Vec<u8> = Vec::new();
        data.push(self.length);
        data.push(Self::data_type());
        data.push(self.class_of_device as u8);
        data.push((self.class_of_device >> 8) as u8);
        data.push((self.class_of_device >> 16) as u8);
        return data;
    }
}

impl DataType for ClassOfDevice {
    /// return `0x0d`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{class_of_device::ClassOfDevice, data_type::DataType};
    ///
    /// assert_eq!(0x0d, ClassOfDevice::data_type());
    /// ```
    fn data_type() -> u8 {
        0x0d
    }
}

/// check `Class of Device` data type.
///
/// # Examples
///
/// ```
/// use ble_data_struct::data_types::class_of_device::*;
/// use ble_data_struct::data_types::data_type::DataType;
///
/// assert!(is_class_of_device(0x0d));
/// assert!(!is_class_of_device(0x00));
/// ```
pub fn is_class_of_device(data_type: u8) -> bool {
    ClassOfDevice::data_type() == data_type
}

#[cfg(test)]
mod tests {
    use crate::data_types::{class_of_device::*, data_type::DataType};

    #[test]
    fn test_new() {
        let major_service_classes = 0b10000000_00000000_00000000;
        let major_device_class = 0b00000000_00000001_00000000;
        let minor_device_class = 0b00000000_00000000_00000100;
        let class_of_device = major_service_classes | major_device_class | minor_device_class;
        let result = ClassOfDevice::new(class_of_device);
        assert_eq!(4, result.length);
        assert_eq!(class_of_device, result.class_of_device);
    }

    #[test]
    fn test_major_service_classes() {
        let major_service_classes = 0b10000000_00000000_00000000;
        let major_device_class = 0b00000000_00000001_00000000;
        let minor_device_class = 0b00000000_00000000_00000100;
        let class_of_device = major_service_classes | major_device_class | minor_device_class;
        let result = ClassOfDevice::new(class_of_device);
        assert_eq!(4, result.length);
        assert_eq!(class_of_device, result.class_of_device);
        assert_eq!(major_service_classes, result.major_service_classes());
    }

    #[test]
    fn test_major_device_class() {
        let major_service_classes = 0b10000000_00000000_00000000;
        let major_device_class = 0b00000000_00000001_00000000;
        let minor_device_class = 0b00000000_00000000_00000100;
        let class_of_device = major_service_classes | major_device_class | minor_device_class;
        let result = ClassOfDevice::new(class_of_device);
        assert_eq!(4, result.length);
        assert_eq!(class_of_device, result.class_of_device);
        assert_eq!(major_device_class, result.major_device_class());
    }

    #[test]
    fn test_minor_device_class() {
        let major_service_classes = 0b10000000_00000000_00000000;
        let major_device_class = 0b00000000_00000001_00000000;
        let minor_device_class = 0b00000000_00000000_00000100;
        let class_of_device = major_service_classes | major_device_class | minor_device_class;
        let result = ClassOfDevice::new(class_of_device);
        assert_eq!(4, result.length);
        assert_eq!(class_of_device, result.class_of_device);
        assert_eq!(minor_device_class, result.minor_device_class());
    }

    #[test]
    fn test_try_from() {
        let major_service_classes = 0b10000000_00000000_00000000;
        let major_device_class = 0b00000000_00000001_00000000;
        let minor_device_class = 0b00000000_00000000_00000100;
        let class_of_device = major_service_classes | major_device_class | minor_device_class;
        let length = 4;
        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(ClassOfDevice::data_type());
        data.push(class_of_device as u8);
        data.push((class_of_device >> 8) as u8);
        data.push((class_of_device >> 16) as u8);

        let result = ClassOfDevice::try_from(&data);
        assert!(result.is_ok());
        let data_type = result.unwrap();
        assert_eq!(length, data_type.length);
        assert_eq!(class_of_device, data_type.class_of_device);

        let mut data: Vec<u8> = vec![0u8; 4];
        data[0] = data.len() as u8 - 1;
        let result = ClassOfDevice::try_from(&data);
        assert!(result.is_err());
        assert_eq!(
            format!("Invalid data size :{}", data.len()),
            result.unwrap_err()
        );
    }

    #[test]
    fn test_into() {
        let major_service_classes = 0b10000000_00000000_00000000;
        let major_device_class = 0b00000000_00000001_00000000;
        let minor_device_class = 0b00000000_00000000_00000100;
        let class_of_device = major_service_classes | major_device_class | minor_device_class;
        let result1 = ClassOfDevice::new(class_of_device);

        let mut data: Vec<u8> = Vec::new();
        data.push(4);
        data.push(ClassOfDevice::data_type());
        data.push(class_of_device as u8);
        data.push((class_of_device >> 8) as u8);
        data.push((class_of_device >> 16) as u8);

        let into_data: Vec<u8> = result1.into();
        assert_eq!(data, into_data);

        let result2 = ClassOfDevice::try_from(&data);
        assert!(result2.is_ok());
        let data_type = result2.unwrap();
        let into_data: Vec<u8> = data_type.into();
        assert_eq!(data, into_data);
    }

    #[test]
    fn test_data_type() {
        assert_eq!(0x0d, ClassOfDevice::data_type());
    }

    #[test]
    fn test_is_class_of_device() {
        assert!(is_class_of_device(0x0d));
        assert!(!is_class_of_device(0x00));
    }
}
