//! EIR/AD/SRD/ACAD/OOB data type module.

use crate::vec_converter::VecConverter;

/// Trait for EIR/AD/SRD/ACAD/OOB data type.
pub trait DataType<T>: VecConverter<T> {
    /// Get EIR/AD/SRD/ACAD/OOB data type
    ///
    /// # Examples
    ///
    /// ```
    /// assert_eq!(0x08, CompleteLocalName::data_type());
    /// ```
    fn data_type() -> u8;
}
