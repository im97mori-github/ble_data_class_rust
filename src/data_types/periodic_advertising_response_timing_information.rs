//! Peripheral Connection Interval Range (Data Type Value: 0x32) module.

use crate::data_types::data_type::DataType;

/// Peripheral Connection Interval Range.
#[derive(Debug)]
pub struct PeriodicAdvertisingResponseTimingInformation {
    /// data length
    pub length: u8,

    /// RspAA
    pub rsp_aa: [u8; 4],

    /// numSubevents
    pub num_subevents: u8,

    /// subeventInterval
    pub subevent_interval: u8,

    /// responseSlotDelay
    pub response_slot_delay: u8,

    /// responseSlotSpacing
    pub response_slot_spacing: u8,
}

impl PeriodicAdvertisingResponseTimingInformation {
    /// Create [`PeriodicAdvertisingResponseTimingInformation`] from Parameters.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::periodic_advertising_response_timing_information::PeriodicAdvertisingResponseTimingInformation;
    ///
    /// let rsp_aa: [u8; 4] = [1, 2, 3, 4];
    /// let num_subevents = 6u8;
    /// let subevent_interval = 7u8;
    /// let response_slot_delay = 8u8;
    /// let response_slot_spacing = 9u8;
    /// let result = PeriodicAdvertisingResponseTimingInformation::new(
    ///     &rsp_aa,
    ///     num_subevents,
    ///     subevent_interval,
    ///     response_slot_delay,
    ///     response_slot_spacing,
    /// );
    /// assert_eq!(9, result.length);
    /// assert_eq!(rsp_aa, result.rsp_aa);
    /// assert_eq!(num_subevents, result.num_subevents);
    /// assert_eq!(subevent_interval, result.subevent_interval);
    /// assert_eq!(response_slot_delay, result.response_slot_delay);
    /// assert_eq!(response_slot_spacing, result.response_slot_spacing);
    /// ```
    pub fn new(
        rsp_aa: &[u8; 4],
        num_subevents: u8,
        subevent_interval: u8,
        response_slot_delay: u8,
        response_slot_spacing: u8,
    ) -> Self {
        Self {
            length: 9,
            rsp_aa: rsp_aa.clone(),
            num_subevents,
            subevent_interval,
            response_slot_delay,
            response_slot_spacing,
        }
    }
}

impl TryFrom<&Vec<u8>> for PeriodicAdvertisingResponseTimingInformation {
    type Error = String;
    /// Create [`PeriodicAdvertisingResponseTimingInformation`] from `Vec<u8>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{periodic_advertising_response_timing_information::PeriodicAdvertisingResponseTimingInformation, data_type::DataType};
    ///
    /// let rsp_aa: [u8; 4] = [1, 2, 3, 4];
    /// let num_subevents = 6u8;
    /// let subevent_interval = 7u8;
    /// let response_slot_delay = 8u8;
    /// let response_slot_spacing = 9u8;
    /// let length = 9;
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(length);
    /// data.push(PeriodicAdvertisingResponseTimingInformation::data_type());
    /// data.append(&mut rsp_aa.to_vec());
    /// data.push(num_subevents);
    /// data.push(subevent_interval);
    /// data.push(response_slot_delay);
    /// data.push(response_slot_spacing);
    ///
    /// let result = PeriodicAdvertisingResponseTimingInformation::try_from(&data);
    /// assert!(result.is_ok());
    /// let data_type = result.unwrap();
    /// assert_eq!(length, data_type.length);
    /// assert_eq!(rsp_aa, data_type.rsp_aa);
    /// assert_eq!(num_subevents, data_type.num_subevents);
    /// assert_eq!(subevent_interval, data_type.subevent_interval);
    /// assert_eq!(response_slot_delay, data_type.response_slot_delay);
    /// assert_eq!(response_slot_spacing, data_type.response_slot_spacing);
    ///
    /// let data: Vec<u8> = Vec::new();
    /// let result = PeriodicAdvertisingResponseTimingInformation::try_from(&data);
    /// assert!(result.is_err());
    /// assert_eq!(
    ///     format!("Invalid data size :{}", data.len()),
    ///     result.unwrap_err()
    /// );
    /// ```
    fn try_from(value: &Vec<u8>) -> Result<Self, String> {
        let len = value.len();
        if len < 10 {
            return Err(format!("Invalid data size :{}", len).to_string());
        }
        let length = value[0];
        Ok(Self {
            length,
            rsp_aa: value[2..6].try_into().unwrap(),
            num_subevents: value[6],
            subevent_interval: value[7],
            response_slot_delay: value[8],
            response_slot_spacing: value[9],
        })
    }
}

impl Into<Vec<u8>> for PeriodicAdvertisingResponseTimingInformation {
    /// Create `Vec<u8>` from [`PeriodicAdvertisingResponseTimingInformation`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{periodic_advertising_response_timing_information::PeriodicAdvertisingResponseTimingInformation, data_type::DataType};
    ///
    /// let rsp_aa: [u8; 4] = [1, 2, 3, 4];
    /// let num_subevents = 6u8;
    /// let subevent_interval = 7u8;
    /// let response_slot_delay = 8u8;
    /// let response_slot_spacing = 9u8;
    /// let length = 9;
    /// let result1 = PeriodicAdvertisingResponseTimingInformation::new(
    ///     &rsp_aa,
    ///     num_subevents,
    ///     subevent_interval,
    ///     response_slot_delay,
    ///     response_slot_spacing,
    /// );
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(length);
    /// data.push(PeriodicAdvertisingResponseTimingInformation::data_type());
    /// data.append(&mut rsp_aa.to_vec());
    /// data.push(num_subevents);
    /// data.push(subevent_interval);
    /// data.push(response_slot_delay);
    /// data.push(response_slot_spacing);
    ///
    /// let into_data: Vec<u8> = result1.into();
    /// assert_eq!(data, into_data);
    ///
    /// let result2 = PeriodicAdvertisingResponseTimingInformation::try_from(&data);
    /// assert!(result2.is_ok());
    /// let data_type = result2.unwrap();
    /// let into_data: Vec<u8> = data_type.into();
    /// assert_eq!(data, into_data);
    /// ```
    fn into(self) -> Vec<u8> {
        let mut data: Vec<u8> = Vec::new();
        data.push(self.length);
        data.push(Self::data_type());
        data.append(&mut self.rsp_aa.clone().to_vec());
        data.push(self.num_subevents);
        data.push(self.subevent_interval);
        data.push(self.response_slot_delay);
        data.push(self.response_slot_spacing);
        return data;
    }
}

impl DataType for PeriodicAdvertisingResponseTimingInformation {
    /// return `0x32`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{periodic_advertising_response_timing_information::PeriodicAdvertisingResponseTimingInformation, data_type::DataType};
    ///
    /// assert_eq!(0x32, PeriodicAdvertisingResponseTimingInformation::data_type());
    /// ```
    fn data_type() -> u8 {
        0x32
    }
}

/// check `Peripheral Connection Interval Range` data type.
///
/// # Examples
///
/// ```
/// use ble_data_struct::data_types::periodic_advertising_response_timing_information::*;
/// use ble_data_struct::data_types::data_type::DataType;
///
/// assert!(is_periodic_advertising_response_timing_information(0x32));
/// assert!(!is_periodic_advertising_response_timing_information(0x00));
/// ```
pub fn is_periodic_advertising_response_timing_information(data_type: u8) -> bool {
    PeriodicAdvertisingResponseTimingInformation::data_type() == data_type
}

#[cfg(test)]
mod tests {
    use crate::data_types::{
        data_type::DataType, periodic_advertising_response_timing_information::*,
    };

    #[test]
    fn test_new() {
        let rsp_aa: [u8; 4] = [1, 2, 3, 4];
        let num_subevents = 6u8;
        let subevent_interval = 7u8;
        let response_slot_delay = 8u8;
        let response_slot_spacing = 9u8;
        let result = PeriodicAdvertisingResponseTimingInformation::new(
            &rsp_aa,
            num_subevents,
            subevent_interval,
            response_slot_delay,
            response_slot_spacing,
        );
        assert_eq!(9, result.length);
        assert_eq!(rsp_aa, result.rsp_aa);
        assert_eq!(num_subevents, result.num_subevents);
        assert_eq!(subevent_interval, result.subevent_interval);
        assert_eq!(response_slot_delay, result.response_slot_delay);
        assert_eq!(response_slot_spacing, result.response_slot_spacing);
    }

    #[test]
    fn test_try_from() {
        let rsp_aa: [u8; 4] = [1, 2, 3, 4];
        let num_subevents = 6u8;
        let subevent_interval = 7u8;
        let response_slot_delay = 8u8;
        let response_slot_spacing = 9u8;
        let length = 9;
        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(PeriodicAdvertisingResponseTimingInformation::data_type());
        data.append(&mut rsp_aa.to_vec());
        data.push(num_subevents);
        data.push(subevent_interval);
        data.push(response_slot_delay);
        data.push(response_slot_spacing);

        let result = PeriodicAdvertisingResponseTimingInformation::try_from(&data);
        assert!(result.is_ok());
        let data_type = result.unwrap();
        assert_eq!(length, data_type.length);
        assert_eq!(rsp_aa, data_type.rsp_aa);
        assert_eq!(num_subevents, data_type.num_subevents);
        assert_eq!(subevent_interval, data_type.subevent_interval);
        assert_eq!(response_slot_delay, data_type.response_slot_delay);
        assert_eq!(response_slot_spacing, data_type.response_slot_spacing);

        let mut data: Vec<u8> = vec![0u8; 9];
        data[0] = data.len() as u8 - 1;
        let result = PeriodicAdvertisingResponseTimingInformation::try_from(&data);
        assert!(result.is_err());
        assert_eq!(
            format!("Invalid data size :{}", data.len()),
            result.unwrap_err()
        );
    }

    #[test]
    fn test_into() {
        let rsp_aa: [u8; 4] = [1, 2, 3, 4];
        let num_subevents = 6u8;
        let subevent_interval = 7u8;
        let response_slot_delay = 8u8;
        let response_slot_spacing = 9u8;
        let length = 9;
        let result1 = PeriodicAdvertisingResponseTimingInformation::new(
            &rsp_aa,
            num_subevents,
            subevent_interval,
            response_slot_delay,
            response_slot_spacing,
        );
        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(PeriodicAdvertisingResponseTimingInformation::data_type());
        data.append(&mut rsp_aa.to_vec());
        data.push(num_subevents);
        data.push(subevent_interval);
        data.push(response_slot_delay);
        data.push(response_slot_spacing);

        let into_data: Vec<u8> = result1.into();
        assert_eq!(data, into_data);

        let result2 = PeriodicAdvertisingResponseTimingInformation::try_from(&data);
        assert!(result2.is_ok());
        let data_type = result2.unwrap();
        let into_data: Vec<u8> = data_type.into();
        assert_eq!(data, into_data);
    }

    #[test]
    fn test_data_type() {
        assert_eq!(
            0x32,
            PeriodicAdvertisingResponseTimingInformation::data_type()
        );
    }

    #[test]
    fn test_is_periodic_advertising_response_timing_information() {
        assert!(is_periodic_advertising_response_timing_information(0x32));
        assert!(!is_periodic_advertising_response_timing_information(0x00));
    }
}
