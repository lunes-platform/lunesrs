pub mod base58;
pub mod hex;
pub mod random;
pub mod serialize;
pub mod strings;
pub mod vectors;

pub fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}
