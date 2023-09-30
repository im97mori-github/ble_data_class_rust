//! Data type parser module.

use super::{
    advertising_interval::{is_advertising_interval, AdvertisingInterval},
    advertising_interval_long::{is_advertising_interval_long, AdvertisingIntervalLong},
    appearance::{is_appearance, Appearance},
};

/// Data type parse result.
pub enum DataTypeParseResult {
    /// [`AdvertisingInterval`]'s [`TryFrom::try_from`] result.
    AdvertisingIntervalResult(Result<AdvertisingInterval, String>),

    /// [`AdvertisingIntervalLong`]'s [`TryFrom::try_from`] result.
    AdvertisingIntervalLongResult(Result<AdvertisingIntervalLong, String>),

    /// [`Appearance`]'s [`TryFrom::try_from`] result.
    AppearanceResult(Result<Appearance, String>),

    /// Occurs for unsupported data types.
    DataTypeParseErr(String),
}

impl DataTypeParseResult {
    /// Returns `true` if the result is [`DataTypeParseResult::AdvertisingIntervalResult`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{advertising_interval::AdvertisingInterval, parser::DataTypeParseResult};
    ///
    /// let advertising_interval = 0x01;
    /// let data: Vec<u8> = AdvertisingInterval::new(advertising_interval).into();
    /// assert!(DataTypeParseResult::from(&data).is_advertising_interval());
    ///
    /// let data: Vec<u8> = Vec::new();
    /// assert!(!DataTypeParseResult::from(&data).is_advertising_interval());
    /// ```
    pub fn is_advertising_interval(&self) -> bool {
        matches!(self, DataTypeParseResult::AdvertisingIntervalResult(_))
    }

    /// Returns `true` if the result is [`DataTypeParseResult::AdvertisingIntervalLongResult`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{advertising_interval_long::AdvertisingIntervalLong, parser::DataTypeParseResult};
    ///
    /// let advertising_interval_long: u32 = 0x01020304u32;
    /// let data: Vec<u8> = AdvertisingIntervalLong::new(true, advertising_interval_long).into();
    /// assert!(DataTypeParseResult::from(&data).is_advertising_interval_long());
    ///
    /// let data: Vec<u8> = Vec::new();
    /// assert!(!DataTypeParseResult::from(&data).is_advertising_interval_long());
    /// ```
    pub fn is_advertising_interval_long(&self) -> bool {
        matches!(self, DataTypeParseResult::AdvertisingIntervalLongResult(_))
    }

    /// Returns `true` if the result is [`DataTypeParseResult::AppearanceResult`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{appearance::Appearance, parser::DataTypeParseResult};
    ///
    /// let appearance: u16 = 0x1444;
    /// let data: Vec<u8> = Appearance::new(appearance).into();
    /// assert!(DataTypeParseResult::from(&data).is_appearance());
    ///
    /// let data: Vec<u8> = Vec::new();
    /// assert!(!DataTypeParseResult::from(&data).is_appearance());
    /// ```
    pub fn is_appearance(&self) -> bool {
        matches!(self, DataTypeParseResult::AppearanceResult(_))
    }
}

impl From<&Vec<u8>> for DataTypeParseResult {
    /// Create [`DataTypeParseResult`] from `Vec<u8>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{advertising_interval::AdvertisingInterval, parser::DataTypeParseResult};
    ///
    /// let advertising_interval = 0x01;
    /// let data: Vec<u8> = AdvertisingInterval::new(advertising_interval).into();
    /// assert!(matches!(
    ///     DataTypeParseResult::from(&data),
    ///     DataTypeParseResult::AdvertisingIntervalResult(_)
    /// ));
    /// let data: Vec<u8> = Vec::new();
    /// assert!(matches!(
    ///     DataTypeParseResult::from(&data),
    ///     DataTypeParseResult::DataTypeParseErr(_)
    /// ));
    fn from(value: &Vec<u8>) -> Self {
        if let Some(data_type) = value.get(1) {
            if is_advertising_interval(data_type.to_owned()) {
                DataTypeParseResult::AdvertisingIntervalResult(AdvertisingInterval::try_from(value))
            } else if is_advertising_interval_long(data_type.to_owned()) {
                DataTypeParseResult::AdvertisingIntervalLongResult(
                    AdvertisingIntervalLong::try_from(value),
                )
            } else if is_appearance(data_type.to_owned()) {
                DataTypeParseResult::AppearanceResult(Appearance::try_from(value))
            } else {
                DataTypeParseResult::DataTypeParseErr(
                    format!("Unknown data type :{}", data_type).to_string(),
                )
            }
        } else {
            DataTypeParseResult::DataTypeParseErr("Invalid data size".to_string())
        }
    }
}

/// Data types parse results.
pub struct DataTypeParseResults {
    /// Parse results.
    pub results: Vec<DataTypeParseResult>,
}

impl From<&Vec<Vec<u8>>> for DataTypeParseResults {
    /// Create [`DataTypeParseResults`] from `Vec<Vec<u8>>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{advertising_interval::AdvertisingInterval, parser::{DataTypeParseResult, DataTypeParseResults}};
    ///
    /// let mut vec: Vec<Vec<u8>> = Vec::new();
    /// let advertising_interval = 0x01;
    /// let data: Vec<u8> = AdvertisingInterval::new(advertising_interval).into();
    /// vec.push(data);
    /// vec.push(vec![]);
    ///
    /// let results = DataTypeParseResults::from(&vec);
    /// assert!(matches!(
    ///     results.results.get(0),
    ///     Some(DataTypeParseResult::AdvertisingIntervalResult(_))
    /// ));
    /// assert!(matches!(
    ///     results.results.get(1),
    ///     Some(DataTypeParseResult::DataTypeParseErr(_))
    /// ));
    /// assert!(matches!(results.results.get(2), None));
    /// ```
    fn from(value: &Vec<Vec<u8>>) -> Self {
        Self {
            results: value
                .iter()
                .map(|f| DataTypeParseResult::from(f))
                .collect::<Vec<DataTypeParseResult>>(),
        }
    }
}

impl From<&Vec<u8>> for DataTypeParseResults {
    /// Create [`DataTypeParseResults`] from `Vec<u8>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{advertising_interval::AdvertisingInterval, parser::{DataTypeParseResult, DataTypeParseResults}};
    ///
    /// let mut vec: Vec<Vec<u8>> = Vec::new();
    /// let advertising_interval = 0x01;
    /// vec.push(AdvertisingInterval::new(advertising_interval).into());
    /// vec.push(vec![100]);
    ///
    /// let results = DataTypeParseResults::from(&vec);
    /// assert!(matches!(
    ///     results.results.get(0),
    ///     Some(DataTypeParseResult::AdvertisingIntervalResult(_))
    /// ));
    /// assert!(matches!(
    ///     results.results.get(1),
    ///     Some(DataTypeParseResult::DataTypeParseErr(_))
    /// ));
    /// assert!(matches!(results.results.get(2), None));
    /// ```
    fn from(value: &Vec<u8>) -> Self {
        let mut vec = Vec::new();
        let mut index = 0;
        let len = value.len();
        while index < len {
            let mut inner: Vec<u8> = Vec::new();
            let size = value[index];
            inner.append(&mut value[index..index + 1 + size as usize].to_vec());
            vec.push(inner);

            index += 1;
            index += size as usize;
        }
        Self::from(&vec)
    }
}

#[cfg(test)]
mod tests {
    use crate::data_types::{
        advertising_interval::AdvertisingInterval,
        advertising_interval_long::AdvertisingIntervalLong, appearance::Appearance,
        parser::DataTypeParseResult,
    };

    use super::DataTypeParseResults;

    #[test]
    fn test_is_advertising_interval() {
        let advertising_interval = 0x01;
        let data: Vec<u8> = AdvertisingInterval::new(advertising_interval).into();
        assert!(DataTypeParseResult::from(&data).is_advertising_interval());

        let data: Vec<u8> = Vec::new();
        assert!(!DataTypeParseResult::from(&data).is_advertising_interval());
    }

    #[test]
    fn test_is_advertising_interval_long() {
        let advertising_interval_long: u32 = 0x01020304u32;
        let data: Vec<u8> = AdvertisingIntervalLong::new(true, advertising_interval_long).into();
        assert!(DataTypeParseResult::from(&data).is_advertising_interval_long());

        let data: Vec<u8> = Vec::new();
        assert!(!DataTypeParseResult::from(&data).is_advertising_interval_long());
    }

    #[test]
    fn test_is_appearance() {
        let appearance: u16 = 0x1444;
        let data: Vec<u8> = Appearance::new(appearance).into();
        assert!(DataTypeParseResult::from(&data).is_appearance());

        let data: Vec<u8> = Vec::new();
        assert!(!DataTypeParseResult::from(&data).is_appearance());
    }

    #[test]
    fn test_result_from_vec() {
        let advertising_interval = 0x01;
        let data: Vec<u8> = AdvertisingInterval::new(advertising_interval).into();
        assert!(matches!(
            DataTypeParseResult::from(&data),
            DataTypeParseResult::AdvertisingIntervalResult(_)
        ));

        let data: Vec<u8> = Vec::new();
        assert!(matches!(
            DataTypeParseResult::from(&data),
            DataTypeParseResult::DataTypeParseErr(_)
        ));
    }

    #[test]
    fn test_results_from_vec_vec() {
        let mut vec: Vec<Vec<u8>> = Vec::new();
        let advertising_interval = 0x01;
        let data: Vec<u8> = AdvertisingInterval::new(advertising_interval).into();
        vec.push(data);
        vec.push(vec![]);

        let results = DataTypeParseResults::from(&vec);
        assert!(matches!(
            results.results.get(0),
            Some(DataTypeParseResult::AdvertisingIntervalResult(_))
        ));
        assert!(matches!(
            results.results.get(1),
            Some(DataTypeParseResult::DataTypeParseErr(_))
        ));
        assert!(matches!(results.results.get(2), None));
    }

    #[test]
    fn test_results_from_vec() {
        let mut vec: Vec<Vec<u8>> = Vec::new();
        let advertising_interval = 0x01;
        vec.push(AdvertisingInterval::new(advertising_interval).into());
        vec.push(vec![100]);

        let results = DataTypeParseResults::from(&vec);
        assert!(matches!(
            results.results.get(0),
            Some(DataTypeParseResult::AdvertisingIntervalResult(_))
        ));
        assert!(matches!(
            results.results.get(1),
            Some(DataTypeParseResult::DataTypeParseErr(_))
        ));
        assert!(matches!(results.results.get(2), None));
    }
}
