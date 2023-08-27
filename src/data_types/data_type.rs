//! EIR/AD/SRD/ACAD/OOB data type module.

/// Trait for EIR/AD/SRD/ACAD/OOB data type.
pub trait DataType {
    /// Get EIR/AD/SRD/ACAD/OOB data type
    fn data_type() -> u8;
}
