/// Sign and validate signatures
pub mod signatures;
/// Generate private and public keys
pub mod assembly;

pub const ADDRESS_VERSION: [u8; 2] = [1, 11];
pub const ADDRESS_CHECKSUM_LENGTH: u8 = 4;
pub const ADDRESS_HASH_LENGTH: u8 = 20;
pub const ADDRESS_LENGTH: u8 = 1 + 1 + ADDRESS_CHECKSUM_LENGTH + ADDRESS_HASH_LENGTH;
