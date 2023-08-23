//! Channel Map Update Indication (Data Type Value: 0x28) module.

use crate::data_types::data_type::DataType;

/// Channel Map Update Indication.
pub struct ChannelMapUpdateIndication {
    /// data length
    pub length: u8,

    /// ChM
    pub ch_m: Vec<bool>,

    /// Instant
    pub instant: u16,
}

impl ChannelMapUpdateIndication {
    /// Create [ChannelMapUpdateIndication] from Parameters.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_class::data_types::channel_map_update_indication::ChannelMapUpdateIndication;
    ///
    /// let mut ch_m = [false; 37].to_vec();
    /// for i in 0..37 {
    ///     ch_m[i] = true;
    ///     let result = ChannelMapUpdateIndication::new(&ch_m, i as u16);
    ///     assert_eq!(8, result.length);
    ///     assert_eq!(ch_m, result.ch_m);
    ///     assert_eq!(i as u16, result.instant);
    ///     ch_m[i] = false;
    /// }
    /// ```
    pub fn new(ch_m: &Vec<bool>, instant: u16) -> Self {
        Self {
            length: 8,
            ch_m: ch_m[..37].to_vec(),
            instant,
        }
    }

    /// Create [ChannelMapUpdateIndication] from `Vec<u8>` with offset.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_class::data_types::{channel_map_update_indication::ChannelMapUpdateIndication, data_type::DataType};
    ///
    /// let mut ch_m = [0u8; 5].to_vec();
    ///
    /// for i in 0..37 {
    ///     ch_m[i / 8] = 0b1 << (i % 8);
    ///
    ///     let length = 8;
    ///     let mut data: Vec<u8> = Vec::new();
    ///     data.push(length);
    ///     data.push(ChannelMapUpdateIndication::data_type());
    ///     data.append(&mut ch_m.clone());
    ///     data.append(&mut (i as u16).to_le_bytes().to_vec());
    ///
    ///     let result = ChannelMapUpdateIndication::from_with_offset(&data, 0);
    ///     assert_eq!(length, result.length);
    ///     let bool_vec: Vec<bool> = ch_m
    ///         .clone()
    ///         .iter()
    ///         .flat_map(|x| {
    ///             let mut data: Vec<bool> = Vec::new();
    ///             data.push((x & 0b0000_0001) != 0);
    ///             data.push((x & 0b0000_0010) != 0);
    ///             data.push((x & 0b0000_0100) != 0);
    ///             data.push((x & 0b0000_1000) != 0);
    ///             data.push((x & 0b0001_0000) != 0);
    ///             data.push((x & 0b0010_0000) != 0);
    ///             data.push((x & 0b0100_0000) != 0);
    ///             data.push((x & 0b1000_0000) != 0);
    ///             data
    ///         })
    ///         .collect();
    ///
    ///     assert_eq!(bool_vec, result.ch_m);
    ///
    ///     ch_m[i / 8] = 0u8;
    /// }
    ///
    /// let mut ch_m = [0u8; 5].to_vec();
    ///
    /// for i in 0..37 {
    ///     ch_m[i / 8] = 0b1 << (i % 8);
    ///
    ///     let length = 8;
    ///     let mut data: Vec<u8> = Vec::new();
    ///     data.push(0);
    ///     data.push(length);
    ///     data.push(ChannelMapUpdateIndication::data_type());
    ///     data.append(&mut ch_m.clone());
    ///     data.append(&mut (i as u16).to_le_bytes().to_vec());
    ///
    ///     let result = ChannelMapUpdateIndication::from_with_offset(&data, 1);
    ///     assert_eq!(length, result.length);
    ///     let bool_vec: Vec<bool> = ch_m
    ///         .clone()
    ///         .iter()
    ///         .flat_map(|x| {
    ///             let mut data: Vec<bool> = Vec::new();
    ///             data.push((x & 0b0000_0001) != 0);
    ///             data.push((x & 0b0000_0010) != 0);
    ///             data.push((x & 0b0000_0100) != 0);
    ///             data.push((x & 0b0000_1000) != 0);
    ///             data.push((x & 0b0001_0000) != 0);
    ///             data.push((x & 0b0010_0000) != 0);
    ///             data.push((x & 0b0100_0000) != 0);
    ///             data.push((x & 0b1000_0000) != 0);
    ///             data
    ///         })
    ///         .collect();
    ///
    ///     assert_eq!(bool_vec, result.ch_m);
    ///
    ///     ch_m[i / 8] = 0u8;
    /// }
    /// ```
    pub fn from_with_offset(data: &Vec<u8>, offset: usize) -> Self {
        let length = data[offset];
        let ch_m: Vec<bool> = data[2 + offset..7 + offset]
            .iter()
            .flat_map(|x| {
                let mut data: Vec<bool> = Vec::new();
                data.push(x & 0b0000_0001 != 0);
                data.push(x & 0b0000_0010 != 0);
                data.push(x & 0b0000_0100 != 0);
                data.push(x & 0b0000_1000 != 0);
                data.push(x & 0b0001_0000 != 0);
                data.push(x & 0b0010_0000 != 0);
                data.push(x & 0b0100_0000 != 0);
                data.push(x & 0b1000_0000 != 0);
                data
            })
            .collect();
        Self {
            length,
            ch_m: ch_m.to_vec(),
            instant: u16::from_le_bytes(data[7 + offset..9 + offset].try_into().unwrap()),
        }
    }
}

impl From<&Vec<u8>> for ChannelMapUpdateIndication {
    /// Create [ChannelMapUpdateIndication] from `Vec<u8>`.
    ///
    /// [`ChannelMapUpdateIndication::from_with_offset`]
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_class::data_types::{channel_map_update_indication::ChannelMapUpdateIndication, data_type::DataType};
    ///
    /// let mut ch_m = [0u8; 5].to_vec();
    ///
    /// for i in 0..37 {
    ///     ch_m[i / 8] = 0b1 << (i % 8);
    ///
    ///     let length = 8;
    ///     let mut data: Vec<u8> = Vec::new();
    ///     data.push(length);
    ///     data.push(ChannelMapUpdateIndication::data_type());
    ///     data.append(&mut ch_m.clone());
    ///     data.append(&mut (i as u16).to_le_bytes().to_vec());
    ///
    ///     let result = ChannelMapUpdateIndication::from(&data);
    ///     assert_eq!(length, result.length);
    ///     let bool_vec: Vec<bool> = ch_m
    ///         .clone()
    ///         .iter()
    ///         .flat_map(|x| {
    ///             let mut data: Vec<bool> = Vec::new();
    ///             data.push((x & 0b0000_0001) != 0);
    ///             data.push((x & 0b0000_0010) != 0);
    ///             data.push((x & 0b0000_0100) != 0);
    ///             data.push((x & 0b0000_1000) != 0);
    ///             data.push((x & 0b0001_0000) != 0);
    ///             data.push((x & 0b0010_0000) != 0);
    ///             data.push((x & 0b0100_0000) != 0);
    ///             data.push((x & 0b1000_0000) != 0);
    ///             data
    ///         })
    ///         .collect();
    ///
    ///     assert_eq!(bool_vec, result.ch_m);
    ///
    ///     ch_m[i / 8] = 0u8;
    /// }
    /// ```
    fn from(data: &Vec<u8>) -> Self {
        Self::from_with_offset(data, 0)
    }
}

impl Into<Vec<u8>> for ChannelMapUpdateIndication {
    /// Create `Vec<u8>` from [ChannelMapUpdateIndication].
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_class::data_types::{channel_map_update_indication::ChannelMapUpdateIndication, data_type::DataType};
    ///
    /// let mut ch_m = [false; 37].to_vec();
    /// for i in 0..37 {
    ///     ch_m[i] = true;
    ///     let result1 = ChannelMapUpdateIndication::new(&ch_m, i as u16);
    ///
    ///     let mut data: Vec<u8> = Vec::new();
    ///     data.push(8);
    ///     data.push(ChannelMapUpdateIndication::data_type());
    ///     let mut u8_vec = [0u8; 5];
    ///     for (i, element) in ch_m.iter().enumerate() {
    ///         if *element {
    ///             u8_vec[i / 8] = u8_vec[i / 8] | 1 << i % 8
    ///         }
    ///     }
    ///     data.append(&mut u8_vec.clone().to_vec());
    ///     data.append(&mut (i as u16).to_le_bytes().to_vec());
    ///
    ///     let into_data: Vec<u8> = result1.into();
    ///     assert_eq!(data, into_data);
    ///
    ///     let result2 = ChannelMapUpdateIndication::from(&data);
    ///     let into_data: Vec<u8> = result2.into();
    ///     assert_eq!(data, into_data);
    ///
    ///     ch_m[i] = false;
    /// }
    /// ```
    fn into(self) -> Vec<u8> {
        let mut data: Vec<u8> = Vec::new();
        data.push(self.length);
        data.push(Self::data_type());
        let mut ch_m = [0u8; 5];
        for (i, element) in self.ch_m.iter().enumerate() {
            if *element {
                ch_m[i / 8] = ch_m[i / 8] | 1 << i % 8
            }
        }
        data.append(&mut ch_m.to_vec());
        data.append(&mut self.instant.to_le_bytes().to_vec());
        return data;
    }
}

impl DataType for ChannelMapUpdateIndication {
    /// return `0x28`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_class::data_types::{channel_map_update_indication::ChannelMapUpdateIndication, data_type::DataType};
    ///
    /// assert_eq!(0x28, ChannelMapUpdateIndication::data_type());
    /// ```
    fn data_type() -> u8 {
        0x28
    }
}

#[cfg(test)]
mod tests {
    use crate::data_types::{
        channel_map_update_indication::ChannelMapUpdateIndication, data_type::DataType,
    };

    #[test]
    fn test_new() {
        let mut ch_m = [false; 37].to_vec();
        for i in 0..37 {
            ch_m[i] = true;
            let result = ChannelMapUpdateIndication::new(&ch_m, i as u16);
            assert_eq!(8, result.length);
            assert_eq!(ch_m, result.ch_m);
            assert_eq!(i as u16, result.instant);
            ch_m[i] = false;
        }
    }

    #[test]
    fn test_from_with_offset() {
        let mut ch_m = [0u8; 5].to_vec();

        for i in 0..37 {
            ch_m[i / 8] = 0b1 << (i % 8);

            let length = 8;
            let mut data: Vec<u8> = Vec::new();
            data.push(length);
            data.push(ChannelMapUpdateIndication::data_type());
            data.append(&mut ch_m.clone());
            data.append(&mut (i as u16).to_le_bytes().to_vec());

            let result = ChannelMapUpdateIndication::from_with_offset(&data, 0);
            assert_eq!(length, result.length);
            let bool_vec: Vec<bool> = ch_m
                .clone()
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

            assert_eq!(bool_vec, result.ch_m);

            ch_m[i / 8] = 0u8;
        }

        let mut ch_m = [0u8; 5].to_vec();

        for i in 0..37 {
            ch_m[i / 8] = 0b1 << (i % 8);

            let length = 8;
            let mut data: Vec<u8> = Vec::new();
            data.push(0);
            data.push(length);
            data.push(ChannelMapUpdateIndication::data_type());
            data.append(&mut ch_m.clone());
            data.append(&mut (i as u16).to_le_bytes().to_vec());

            let result = ChannelMapUpdateIndication::from_with_offset(&data, 1);
            assert_eq!(length, result.length);
            let bool_vec: Vec<bool> = ch_m
                .clone()
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

            assert_eq!(bool_vec, result.ch_m);

            ch_m[i / 8] = 0u8;
        }
    }

    #[test]
    fn test_from() {
        let mut ch_m = [0u8; 5].to_vec();

        for i in 0..37 {
            ch_m[i / 8] = 0b1 << (i % 8);

            let length = 8;
            let mut data: Vec<u8> = Vec::new();
            data.push(length);
            data.push(ChannelMapUpdateIndication::data_type());
            data.append(&mut ch_m.clone());
            data.append(&mut (i as u16).to_le_bytes().to_vec());

            let result = ChannelMapUpdateIndication::from(&data);
            assert_eq!(length, result.length);
            let bool_vec: Vec<bool> = ch_m
                .clone()
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

            assert_eq!(bool_vec, result.ch_m);

            ch_m[i / 8] = 0u8;
        }
    }

    #[test]
    fn test_into() {
        let mut ch_m = [false; 37].to_vec();
        for i in 0..37 {
            ch_m[i] = true;
            let result1 = ChannelMapUpdateIndication::new(&ch_m, i as u16);

            let mut data: Vec<u8> = Vec::new();
            data.push(8);
            data.push(ChannelMapUpdateIndication::data_type());
            let mut u8_vec = [0u8; 5];
            for (i, element) in ch_m.iter().enumerate() {
                if *element {
                    u8_vec[i / 8] = u8_vec[i / 8] | 1 << i % 8
                }
            }
            data.append(&mut u8_vec.clone().to_vec());
            data.append(&mut (i as u16).to_le_bytes().to_vec());

            let into_data: Vec<u8> = result1.into();
            assert_eq!(data, into_data);

            let result2 = ChannelMapUpdateIndication::from(&data);
            let into_data: Vec<u8> = result2.into();
            assert_eq!(data, into_data);

            ch_m[i] = false;
        }
    }

    #[test]
    fn test_data_type() {
        assert_eq!(0x28, ChannelMapUpdateIndication::data_type());
    }
}
