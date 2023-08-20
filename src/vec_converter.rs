//! [Vec`<u8>`] <-> BLE Data class converter module.

/// Trait for [Vec`<u8>`] <-> BLE Data class converter.
pub trait VecConverter<T> {
    /// Create [Vec`<u8>`] format BLE data.
    fn into_bytes(&self) -> Vec<u8>;

    /// Create BLE data class from [Vec`<u8>`] format.
    fn from_vec(data: &Vec<u8>) -> T;
}
