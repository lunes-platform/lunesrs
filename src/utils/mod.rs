/// Functions for get vetor of random numbers
pub mod random;
/// convert vectors between u32 and u8
pub mod vectors;

pub fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}
