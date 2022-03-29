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
//! - **Utils**
//!     - Serialize
//!         - serializeString *String* -> *Vec<u8>*
//!         - serializeUInteger *u64* -> *Vec<u8>*
//!
//!      - Crypto
//!         - to_blake2b32b *Vec<u8>* -> *Vec<u8>*
//!         - to_keccak256 *Vec<u8>* -> *Vec<u8>*
//!         - to_sha256 *Vec<u8>* -> *Vec<u8>*
//!
//!      - Random
//!         - randomTripleNumber -> *Vec<u32>*
//!         - randomBytes *usize* -> *Vec<u32>*
//!
//!      - Vectors
//!         - toVecu32 *Vec<u8>* -> *Vec<u32>*
//!         - toVecu8 *Vec<u32>* -> *Vec<u8>*
//!
//!      - Base58
//!         - vecToB58 *Vec<u8>* -> *String*
//!         - b58ToVec *String* -> *Vec<u8>*
//! - **Account**
//!     - Signature
//!         - validateSignature
//!         - validateAddress
//!         - decodeMessage
//!         - fullSignature
//!         - fastSignature
//!
//!      - Wallet
//!         - toPrivateKey
//!         - toPublicKey
//!         - hiddenSeed
//!         - toAddress
//!

/// Functions for wallets for Lunes Blockchain
pub mod account;
/// Utils functions
pub mod utils;
