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
//! [Telescope](https://lunes-platform.github.io/telescope)
//!
//! ## 🏗 Archtecture
//!
//! - **Account**
//!     - Signature
//!         - validate_signature `(Vec<u32>, Vec<u32>, Vec<u32>)` -> `Vec<u32>`
//!         - full_signature `(Vec<u32>, Vec<u32>)` -> `Vec<u32>`
//!         - fast_signature `(Vec<u32>, Vec<u32>)` -> `Vec<u32>`
//!         - validate_address `(u8, Vec<u8>)` -> `bool`
//!
//!      - Wallet
//!         - to_address `(u8, u8, Vec<u8>)` -> `Vec<u8>`
//!         - hidden_seed `(u32, String)` -> `Vec<u8>`
//!         - to_private_key `Vec<u8>` -> `Vec<u8>`
//!         - to_public_key `Vec<u8>` -> `Vec<u8>`
//!
//! - **Utils**
//!     - Serialize
//!         - serialize_string `String` -> `Vec<u8>`
//!         - serialize_uinteger `u64` -> `Vec<u8>`
//!
//!      - Crypto
//!         - to_blake2b32b `Vec<u8>` -> `Vec<u8>`
//!         - to_keccak256 `Vec<u8>` -> `Vec<u8>`
//!         - to_sha256 `Vec<u8>` -> `Vec<u8>`
//!
//!      - Random
//!         - random_triple_number -> `Vec<u32>`
//!         - random_bytes `usize` -> `Vec<u32>`
//!
//!      - Vectors
//!         - to_vecu32 `Vec<u8>` -> `Vec<u32>`
//!         - to_vecu8 `Vec<u32>` -> `Vec<u8>`
//!
//!      - Base58
//!         - vec_to_b58 `Vec<u8>` -> `String`
//!         - b58_to_vec `String` -> `Vec<u8>`
//!

/// Functions for wallets for Lunes Blockchain
pub mod wallet;
/// Utils functions
pub mod utils;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
