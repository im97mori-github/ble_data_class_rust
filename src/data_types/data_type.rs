//! EIR/AD/SRD/ACAD/OOB data type module.

/// Trait for EIR/AD/SRD/ACAD/OOB data type.
pub trait DataType {
    /// Get EIR/AD/SRD/ACAD/OOB data type
    ///
    /// # Examples
    ///
    /// ```
    /// assert_eq!(0x08, CompleteLocalName::data_type());
    /// ```
    fn data_type() -> u8;
}