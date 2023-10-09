//! Flags (Data Type Value: 0x01) module.

use crate::data_types::data_type::DataType;

/// Flags.
#[derive(Debug, PartialEq, Clone)]
pub struct Flags {
    /// data length
    pub length: u8,

    /// Flags
    pub flags: Vec<bool>,
}

impl Flags {
    /// Create [`Flags`] from `Flags`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::flags::Flags;
    ///
    /// let flags = [true, false, false, false, false, false, false, false].to_vec();
    /// let result = Flags::new(&flags);
    /// assert_eq!(2, result.length);
    /// assert_eq!(flags, result.flags);
    ///
    /// let flags = [false, true, false, false, false, false, false, false].to_vec();
    /// let result = Flags::new(&flags);
    /// assert_eq!(2, result.length);
    /// assert_eq!(flags, result.flags);
    ///
    /// let flags = [false, false, true, false, false, false, false, false].to_vec();
    /// let result = Flags::new(&flags);
    /// assert_eq!(2, result.length);
    /// assert_eq!(flags, result.flags);
    ///
    /// let flags = [
    ///     true, false, false, false, false, false, false, false, false, false, false, false,
    ///     false, false, false, false,
    /// ]
    /// .to_vec();
    /// let result = Flags::new(&flags);
    /// assert_eq!(3, result.length);
    /// assert_eq!(flags, result.flags);
    /// ```
    pub fn new(flags: &Vec<bool>) -> Self {
        Self {
            length: (flags.len() / 8 + 1) as u8,
            flags: flags.clone(),
        }
    }

    /// check LE Limited Discoverable Mode.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::flags::Flags;
    ///
    /// let flags = [true, false, false, false, false, false, false, false].to_vec();
    /// let result = Flags::new(&flags);
    /// assert!(result.is_le_limited_discoverable_mode());
    ///
    /// let flags: Vec<bool> = [false, false, false, false, false, false, false, false].to_vec();
    /// let result = Flags::new(&flags);
    /// assert!(!result.is_le_limited_discoverable_mode());
    ///
    /// let flags: Vec<bool> = [].to_vec();
    /// let result = Flags::new(&flags);
    /// assert!(!result.is_le_limited_discoverable_mode());
    /// ```
    pub fn is_le_limited_discoverable_mode(&self) -> bool {
        *self.flags.get(0).unwrap_or(&false)
    }

    /// check LE General Discoverable Mode.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::flags::Flags;
    ///
    /// let flags = [false, true, false, false, false, false, false, false].to_vec();
    /// let result = Flags::new(&flags);
    /// assert!(result.is_le_general_discoverable_mode());
    ///
    /// let flags: Vec<bool> = [false, false, false, false, false, false, false, false].to_vec();
    /// let result = Flags::new(&flags);
    /// assert!(!result.is_le_general_discoverable_mode());
    ///
    /// let flags: Vec<bool> = [].to_vec();
    /// let result = Flags::new(&flags);
    /// assert!(!result.is_le_general_discoverable_mode());
    /// ```
    pub fn is_le_general_discoverable_mode(&self) -> bool {
        *self.flags.get(1).unwrap_or(&false)
    }

    /// check BR/EDR Not Supported.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::flags::Flags;
    ///
    /// let flags = [false, false, true, false, false, false, false, false].to_vec();
    /// let result = Flags::new(&flags);
    /// assert!(result.is_br_edr_not_supported());
    ///
    /// let flags: Vec<bool> = [false, false, false, false, false, false, false, false].to_vec();
    /// let result = Flags::new(&flags);
    /// assert!(!result.is_br_edr_not_supported());
    ///
    /// let flags: Vec<bool> = [].to_vec();
    /// let result = Flags::new(&flags);
    /// assert!(!result.is_br_edr_not_supported());
    /// ```
    pub fn is_br_edr_not_supported(&self) -> bool {
        *self.flags.get(2).unwrap_or(&false)
    }

    /// check Simultaneous LE and BR/EDR to Same Device Capable (Controller).
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::flags::Flags;
    ///
    /// let flags = [false, false, false, true, false, false, false, false].to_vec();
    /// let result = Flags::new(&flags);
    /// assert!(result.is_simultaneous_controller());
    ///
    /// let flags: Vec<bool> = [false, false, false, false, false, false, false, false].to_vec();
    /// let result = Flags::new(&flags);
    /// assert!(!result.is_simultaneous_controller());
    ///
    /// let flags: Vec<bool> = [].to_vec();
    /// let result = Flags::new(&flags);
    /// assert!(!result.is_simultaneous_controller());
    /// ```
    pub fn is_simultaneous_controller(&self) -> bool {
        *self.flags.get(3).unwrap_or(&false)
    }
}

impl TryFrom<&Vec<u8>> for Flags {
    type Error = String;
    /// Create [`Flags`] from [`Vec<u8>`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{flags::Flags, data_type::DataType};
    ///
    /// let flags_bytes = [0b00000001u8].to_vec();
    /// let flags: Vec<bool> = flags_bytes
    ///     .iter()
    ///     .flat_map(|x| {
    ///         let mut data: Vec<bool> = Vec::new();
    ///         data.push((x & 0b0000_0001) != 0);
    ///         data.push((x & 0b0000_0010) != 0);
    ///         data.push((x & 0b0000_0100) != 0);
    ///         data.push((x & 0b0000_1000) != 0);
    ///         data.push((x & 0b0001_0000) != 0);
    ///         data.push((x & 0b0010_0000) != 0);
    ///         data.push((x & 0b0100_0000) != 0);
    ///         data.push((x & 0b1000_0000) != 0);
    ///         data
    ///     })
    ///     .collect();
    /// let length = 2;
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(length);
    /// data.push(Flags::data_type());
    /// data.append(&mut flags_bytes.clone());
    ///
    /// let result = Flags::try_from(&data);
    /// assert!(result.is_ok());
    /// let data_type = result.unwrap();
    /// assert_eq!(length, data_type.length);
    /// assert_eq!(flags, data_type.flags);
    ///
    /// let flags_bytes = [0b00000010u8].to_vec();
    /// let flags: Vec<bool> = flags_bytes
    ///     .iter()
    ///     .flat_map(|x| {
    ///         let mut data: Vec<bool> = Vec::new();
    ///         data.push((x & 0b0000_0001) != 0);
    ///         data.push((x & 0b0000_0010) != 0);
    ///         data.push((x & 0b0000_0100) != 0);
    ///         data.push((x & 0b0000_1000) != 0);
    ///         data.push((x & 0b0001_0000) != 0);
    ///         data.push((x & 0b0010_0000) != 0);
    ///         data.push((x & 0b0100_0000) != 0);
    ///         data.push((x & 0b1000_0000) != 0);
    ///         data
    ///     })
    ///     .collect();
    /// let length = 2;
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(length);
    /// data.push(Flags::data_type());
    /// data.append(&mut flags_bytes.clone());
    ///
    /// let result = Flags::try_from(&data);
    /// assert!(result.is_ok());
    /// let data_type = result.unwrap();
    /// assert_eq!(length, data_type.length);
    /// assert_eq!(flags, data_type.flags);
    ///
    /// let flags_bytes = [0b00000100u8].to_vec();
    /// let flags: Vec<bool> = flags_bytes
    ///     .iter()
    ///     .flat_map(|x| {
    ///         let mut data: Vec<bool> = Vec::new();
    ///         data.push((x & 0b0000_0001) != 0);
    ///         data.push((x & 0b0000_0010) != 0);
    ///         data.push((x & 0b0000_0100) != 0);
    ///         data.push((x & 0b0000_1000) != 0);
    ///         data.push((x & 0b0001_0000) != 0);
    ///         data.push((x & 0b0010_0000) != 0);
    ///         data.push((x & 0b0100_0000) != 0);
    ///         data.push((x & 0b1000_0000) != 0);
    ///         data
    ///     })
    ///     .collect();
    /// let length = 2;
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(length);
    /// data.push(Flags::data_type());
    /// data.append(&mut flags_bytes.clone());
    ///
    /// let result = Flags::try_from(&data);
    /// assert!(result.is_ok());
    /// let data_type = result.unwrap();
    /// assert_eq!(length, data_type.length);
    /// assert_eq!(flags, data_type.flags);
    ///
    /// let length = 1;
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(length);
    /// data.push(Flags::data_type());
    ///
    /// let result = Flags::try_from(&data);
    /// assert!(result.is_ok());
    /// let data_type = result.unwrap();
    /// assert_eq!(length, data_type.length);
    /// assert_eq!(Vec::<bool>::new(), data_type.flags);
    ///
    /// let data: Vec<u8> = Vec::new();
    /// let result = Flags::try_from(&data);
    /// assert!(result.is_err());
    /// assert_eq!(
    ///     format!("Invalid data size :{}", data.len()),
    ///     result.unwrap_err()
    /// );
    /// ```
    fn try_from(value: &Vec<u8>) -> Result<Self, String> {
        let len = value.len();
        if len < 2 {
            return Err(format!("Invalid data size :{}", len).to_string());
        }
        let length = value[0];
        Ok(Self {
            length,
            flags: value[2..(2 + length - 1) as usize]
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
                .collect(),
        })
    }
}

impl Into<Vec<u8>> for Flags {
    /// Create [`Vec<u8>`] from [`Flags`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{flags::Flags, data_type::DataType};
    ///
    /// let flags = [true, false, false, false, false, false, false, false].to_vec();
    /// let length = (flags.len() / 8 + 1) as u8;
    /// let result1 = Flags::new(&flags);
    ///
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(length);
    /// data.push(Flags::data_type());
    /// data.append(
    ///     &mut flags
    ///         .windows(8)
    ///         .step_by(8)
    ///         .map(|w| {
    ///             let mut flag = 0u8;
    ///             for (i, element) in w[0..8].iter().enumerate() {
    ///                 if *element {
    ///                     flag |= 1 << i;
    ///                 }
    ///             }
    ///             flag
    ///         })
    ///         .collect(),
    /// );
    ///
    /// let into_data: Vec<u8> = result1.into();
    /// assert_eq!(data, into_data);
    ///
    /// let result2 = Flags::try_from(&data);
    /// assert!(result2.is_ok());
    /// let data_type = result2.unwrap();
    /// let into_data: Vec<u8> = data_type.into();
    /// assert_eq!(data, into_data);
    ///
    /// let flags = [false, true, false, false, false, false, false, false].to_vec();
    /// let length = (flags.len() / 8 + 1) as u8;
    /// let result1 = Flags::new(&flags);
    ///
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(length);
    /// data.push(Flags::data_type());
    /// data.append(
    ///     &mut flags
    ///         .windows(8)
    ///         .step_by(8)
    ///         .map(|w| {
    ///             let mut flag = 0u8;
    ///             for (i, element) in w[0..8].iter().enumerate() {
    ///                 if *element {
    ///                     flag |= 1 << i;
    ///                 }
    ///             }
    ///             flag
    ///         })
    ///         .collect(),
    /// );
    ///
    /// let into_data: Vec<u8> = result1.into();
    /// assert_eq!(data, into_data);
    ///
    /// let result2 = Flags::try_from(&data);
    /// assert!(result2.is_ok());
    /// let data_type = result2.unwrap();
    /// let into_data: Vec<u8> = data_type.into();
    /// assert_eq!(data, into_data);
    ///
    /// let flags = [false, false, true, false, false, false, false, false].to_vec();
    /// let length = (flags.len() / 8 + 1) as u8;
    /// let result1 = Flags::new(&flags);
    ///
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(length);
    /// data.push(Flags::data_type());
    /// data.append(
    ///     &mut flags
    ///         .windows(8)
    ///         .step_by(8)
    ///         .map(|w| {
    ///             let mut flag = 0u8;
    ///             for (i, element) in w[0..8].iter().enumerate() {
    ///                 if *element {
    ///                     flag |= 1 << i;
    ///                 }
    ///             }
    ///             flag
    ///         })
    ///         .collect(),
    /// );
    ///
    /// let into_data: Vec<u8> = result1.into();
    /// assert_eq!(data, into_data);
    ///
    /// let result2 = Flags::try_from(&data);
    /// assert!(result2.is_ok());
    /// let data_type = result2.unwrap();
    /// let into_data: Vec<u8> = data_type.into();
    /// assert_eq!(data, into_data);
    ///
    /// let flags = [
    ///     true, false, false, false, false, false, false, false, false, false, false, false,
    ///     false, false, false, false,
    /// ]
    /// .to_vec();
    /// let length = (flags.len() / 8 + 1) as u8;
    /// let result1 = Flags::new(&flags);
    ///
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(length);
    /// data.push(Flags::data_type());
    /// data.append(
    ///     &mut flags
    ///         .windows(8)
    ///         .step_by(8)
    ///         .map(|w| {
    ///             let mut flag = 0u8;
    ///             for (i, element) in w[0..8].iter().enumerate() {
    ///                 if *element {
    ///                     flag |= 1 << i;
    ///                 }
    ///             }
    ///             flag
    ///         })
    ///         .collect(),
    /// );
    ///
    /// let into_data: Vec<u8> = result1.into();
    /// assert_eq!(data, into_data);
    ///
    /// let result1 = Flags::new(&Vec::<bool>::new());
    ///
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(1);
    /// data.push(Flags::data_type());
    ///
    /// let into_data: Vec<u8> = result1.into();
    /// assert_eq!(data, into_data);
    ///
    /// let result2 = Flags::try_from(&data);
    /// assert!(result2.is_ok());
    /// let data_type = result2.unwrap();
    /// let into_data: Vec<u8> = data_type.into();
    /// assert_eq!(data, into_data);
    /// ```
    fn into(self) -> Vec<u8> {
        let mut data: Vec<u8> = Vec::new();
        data.push(self.length);
        data.push(Self::data_type());

        let mut flags: Vec<u8> = self
            .flags
            .windows(8)
            .step_by(8)
            .map(|w| {
                let mut flag = 0u8;
                for (i, element) in w[0..8].iter().enumerate() {
                    if *element {
                        flag |= 1 << i;
                    }
                }
                flag
            })
            .collect();
        data.append(&mut flags);
        return data;
    }
}

impl DataType for Flags {
    /// return `0x01`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{flags::Flags, data_type::DataType};
    ///
    /// assert_eq!(0x01, Flags::data_type());
    /// ```
    fn data_type() -> u8 {
        0x01
    }
}

/// check `Flags` data type.
///
/// # Examples
///
/// ```
/// use ble_data_struct::data_types::flags::*;
/// use ble_data_struct::data_types::data_type::DataType;
///
/// assert!(is_flags(0x01));
/// assert!(!is_flags(0x00));
/// ```
pub fn is_flags(data_type: u8) -> bool {
    Flags::data_type() == data_type
}

#[cfg(test)]
mod tests {
    use crate::data_types::{data_type::DataType, flags::*};

    #[test]
    fn test_new() {
        let flags = [true, false, false, false, false, false, false, false].to_vec();
        let result = Flags::new(&flags);
        assert_eq!(2, result.length);
        assert_eq!(flags, result.flags);

        let flags = [false, true, false, false, false, false, false, false].to_vec();
        let result = Flags::new(&flags);
        assert_eq!(2, result.length);
        assert_eq!(flags, result.flags);

        let flags = [false, false, true, false, false, false, false, false].to_vec();
        let result = Flags::new(&flags);
        assert_eq!(2, result.length);
        assert_eq!(flags, result.flags);

        let flags = [
            true, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false,
        ]
        .to_vec();
        let result = Flags::new(&flags);
        assert_eq!(3, result.length);
        assert_eq!(flags, result.flags);
    }

    #[test]
    fn test_is_le_limited_discoverable_mode() {
        let flags = [true, false, false, false, false, false, false, false].to_vec();
        let result = Flags::new(&flags);
        assert!(result.is_le_limited_discoverable_mode());

        let flags: Vec<bool> = [false, false, false, false, false, false, false, false].to_vec();
        let result = Flags::new(&flags);
        assert!(!result.is_le_limited_discoverable_mode());

        let flags: Vec<bool> = [].to_vec();
        let result = Flags::new(&flags);
        assert!(!result.is_le_limited_discoverable_mode());
    }

    #[test]
    fn test_is_le_general_discoverable_mode() {
        let flags = [false, true, false, false, false, false, false, false].to_vec();
        let result = Flags::new(&flags);
        assert!(result.is_le_general_discoverable_mode());

        let flags: Vec<bool> = [false, false, false, false, false, false, false, false].to_vec();
        let result = Flags::new(&flags);
        assert!(!result.is_le_general_discoverable_mode());

        let flags: Vec<bool> = [].to_vec();
        let result = Flags::new(&flags);
        assert!(!result.is_le_general_discoverable_mode());
    }

    #[test]
    fn test_is_br_edr_not_supported() {
        let flags = [false, false, true, false, false, false, false, false].to_vec();
        let result = Flags::new(&flags);
        assert!(result.is_br_edr_not_supported());

        let flags: Vec<bool> = [false, false, false, false, false, false, false, false].to_vec();
        let result = Flags::new(&flags);
        assert!(!result.is_br_edr_not_supported());

        let flags: Vec<bool> = [].to_vec();
        let result = Flags::new(&flags);
        assert!(!result.is_br_edr_not_supported());
    }

    #[test]
    fn test_is_simultaneous_controller() {
        let flags = [false, false, false, true, false, false, false, false].to_vec();
        let result = Flags::new(&flags);
        assert!(result.is_simultaneous_controller());

        let flags: Vec<bool> = [false, false, false, false, false, false, false, false].to_vec();
        let result = Flags::new(&flags);
        assert!(!result.is_simultaneous_controller());

        let flags: Vec<bool> = [].to_vec();
        let result = Flags::new(&flags);
        assert!(!result.is_simultaneous_controller());
    }

    #[test]
    fn test_try_from() {
        let flags_bytes = [0b00000001u8].to_vec();
        let flags: Vec<bool> = flags_bytes
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
        let length = 2;
        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(Flags::data_type());
        data.append(&mut flags_bytes.clone());

        let result = Flags::try_from(&data);
        assert!(result.is_ok());
        let data_type = result.unwrap();
        assert_eq!(length, data_type.length);
        assert_eq!(flags, data_type.flags);

        let flags_bytes = [0b00000010u8].to_vec();
        let flags: Vec<bool> = flags_bytes
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
        let length = 2;
        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(Flags::data_type());
        data.append(&mut flags_bytes.clone());

        let result = Flags::try_from(&data);
        assert!(result.is_ok());
        let data_type = result.unwrap();
        assert_eq!(length, data_type.length);
        assert_eq!(flags, data_type.flags);

        let flags_bytes = [0b00000100u8].to_vec();
        let flags: Vec<bool> = flags_bytes
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
        let length = 2;
        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(Flags::data_type());
        data.append(&mut flags_bytes.clone());

        let result = Flags::try_from(&data);
        assert!(result.is_ok());
        let data_type = result.unwrap();
        assert_eq!(length, data_type.length);
        assert_eq!(flags, data_type.flags);

        let length = 1;
        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(Flags::data_type());

        let result = Flags::try_from(&data);
        assert!(result.is_ok());
        let data_type = result.unwrap();
        assert_eq!(length, data_type.length);
        assert_eq!(Vec::<bool>::new(), data_type.flags);

        let mut data: Vec<u8> = vec![0u8; 1];
        data[0] = data.len() as u8 - 1;
        let result = Flags::try_from(&data);
        assert!(result.is_err());
        assert_eq!(
            format!("Invalid data size :{}", data.len()),
            result.unwrap_err()
        );
    }

    #[test]
    fn test_into() {
        let flags = [true, false, false, false, false, false, false, false].to_vec();
        let length = (flags.len() / 8 + 1) as u8;
        let result1 = Flags::new(&flags);

        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(Flags::data_type());
        data.append(
            &mut flags
                .windows(8)
                .step_by(8)
                .map(|w| {
                    let mut flag = 0u8;
                    for (i, element) in w[0..8].iter().enumerate() {
                        if *element {
                            flag |= 1 << i;
                        }
                    }
                    flag
                })
                .collect(),
        );

        let into_data: Vec<u8> = result1.into();
        assert_eq!(data, into_data);

        let result2 = Flags::try_from(&data);
        assert!(result2.is_ok());
        let data_type = result2.unwrap();
        let into_data: Vec<u8> = data_type.into();
        assert_eq!(data, into_data);

        let flags = [false, true, false, false, false, false, false, false].to_vec();
        let length = (flags.len() / 8 + 1) as u8;
        let result1 = Flags::new(&flags);

        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(Flags::data_type());
        data.append(
            &mut flags
                .windows(8)
                .step_by(8)
                .map(|w| {
                    let mut flag = 0u8;
                    for (i, element) in w[0..8].iter().enumerate() {
                        if *element {
                            flag |= 1 << i;
                        }
                    }
                    flag
                })
                .collect(),
        );

        let into_data: Vec<u8> = result1.into();
        assert_eq!(data, into_data);

        let result2 = Flags::try_from(&data);
        assert!(result2.is_ok());
        let data_type = result2.unwrap();
        let into_data: Vec<u8> = data_type.into();
        assert_eq!(data, into_data);

        let flags = [false, false, true, false, false, false, false, false].to_vec();
        let length = (flags.len() / 8 + 1) as u8;
        let result1 = Flags::new(&flags);

        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(Flags::data_type());
        data.append(
            &mut flags
                .windows(8)
                .step_by(8)
                .map(|w| {
                    let mut flag = 0u8;
                    for (i, element) in w[0..8].iter().enumerate() {
                        if *element {
                            flag |= 1 << i;
                        }
                    }
                    flag
                })
                .collect(),
        );

        let into_data: Vec<u8> = result1.into();
        assert_eq!(data, into_data);

        let result2 = Flags::try_from(&data);
        assert!(result2.is_ok());
        let data_type = result2.unwrap();
        let into_data: Vec<u8> = data_type.into();
        assert_eq!(data, into_data);

        let flags = [
            true, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false,
        ]
        .to_vec();
        let length = (flags.len() / 8 + 1) as u8;
        let result1 = Flags::new(&flags);

        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(Flags::data_type());
        data.append(
            &mut flags
                .windows(8)
                .step_by(8)
                .map(|w| {
                    let mut flag = 0u8;
                    for (i, element) in w[0..8].iter().enumerate() {
                        if *element {
                            flag |= 1 << i;
                        }
                    }
                    flag
                })
                .collect(),
        );

        let into_data: Vec<u8> = result1.into();
        assert_eq!(data, into_data);

        let result1 = Flags::new(&Vec::<bool>::new());

        let mut data: Vec<u8> = Vec::new();
        data.push(1);
        data.push(Flags::data_type());

        let into_data: Vec<u8> = result1.into();
        assert_eq!(data, into_data);

        let result2 = Flags::try_from(&data);
        assert!(result2.is_ok());
        let data_type = result2.unwrap();
        let into_data: Vec<u8> = data_type.into();
        assert_eq!(data, into_data);
    }

    #[test]
    fn test_data_type() {
        assert_eq!(0x01, Flags::data_type());
    }

    #[test]
    fn test_is_flags() {
        assert!(is_flags(0x01));
        assert!(!is_flags(0x00));
    }
}
