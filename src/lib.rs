// Copyright 2022 Developers of the Lunes Platform.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
//
//! # 📦 Lunes SDK
//!
//! This crate is a piece of lunesjs.
//! Lunesjs is one of the libraries that make up the Lunes SDK.
//!
//! ## ⚠️ Caution
//!
//! This repository is full of cryptography functions with some abstraction, be sure what you are doing
//!
//! ## 🔭 Telescope
//!
//! For the user guide and further documentation, please read
//! [Telescope](https:blockchain.lunes.io/telescope)
//!
//! ## 🏗 Archtecture
//!
//! ```rust
//! mod utils {
//!     pub mod random {
//!         fn randomTripleNumber -> Vec<u32>
//!         fn randomBytes(usize) -> Vec<u32>
//!     }
//!     pub mod strings {
//!         fn stringToB58
//!         fn stringToVecu32
//!         fn vecu32ToString
//!         fn stringToUint32Array
//!         fn uint32ArrayToString
//!     }
//!     pub mod base58 {
//!         fn b58ToVec
//!         fn vecToB58
//!     }
//!     pub mod hexadecimals {
//!         fn fromStrHex
//!         fn hexToB58
//!         fn vecu32ToHex
//!     }
//!     pub mod vectors {
//!         fn toVecu32(Vec<u8>) -> Vec<u32>
//!         fn toVecu8(Vec<u32>) -> Vec<u8>
//!     }
//!     mod serialize {
//!         fn serializeString(String) -> Vec<u8>
//!         fn serializeUInteger(u64) -> Vec<u8>
//!     }
//! }
//! mod account {
//!     mod signature {
//!         fn validateAddress
//!         fn validateSignature
//!         fn decode_message
//!         fn fullSignature
//!         fn fastSignature
//!     }
//!     mod crypto {
//!         fn toPrivateKeyHex
//!         fn toPublicKeyHex
//!         fn toAddressHex
//!         fn hiddenSeed
//!         fn keypair_new
//!         fn keypair_privateKey
//!         fn keypair_publiKey
//!     }
//! }
//!

/// Functions for wallets for Lunes Blockchain
pub mod account;
/// Utils functions
pub mod utils;
