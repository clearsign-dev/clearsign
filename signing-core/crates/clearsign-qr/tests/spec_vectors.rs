//! Test vectors copied from the specifications, not from this implementation.
//!
//! - Bytewords: BCR-2020-012, "Example/Test Vector".
//! - Xoshiro256**, random sampler, degree chooser, Fisher-Yates shuffle,
//!   fragment partitioning and part CBOR: BCR-2024-001.
//!
//! If this crate and the specification disagree, a test here fails.

#![allow(
    clippy::expect_used,
    clippy::unwrap_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::panic
)]

use clearsign_qr::bytewords;
use clearsign_qr::crc32::crc32;
use clearsign_qr::fountain::{
    RandomSampler, Xoshiro256, choose_degree, choose_fragments, find_nominal_fragment_length,
    shuffled,
};

/// BCR-2024-001: `makeMessage(len:seed:)`.
fn make_message(len: usize) -> Vec<u8> {
    let mut rng = Xoshiro256::from_seed_bytes(b"Wolf");
    (0..len).map(|_| rng.next_int(0, 256) as u8).collect()
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

fn unhex(s: &str) -> Vec<u8> {
    (0..s.len() / 2)
        .map(|i| u8::from_str_radix(&s[i * 2..i * 2 + 2], 16).unwrap())
        .collect()
}

const RNG1: [u64; 100] = [
    42, 81, 85, 8, 82, 84, 76, 73, 70, 88, 2, 74, 40, 48, 77, 54, 88, 7, 5, 88, 37, 25, 82, 13, 69,
    59, 30, 39, 11, 82, 19, 99, 45, 87, 30, 15, 32, 22, 89, 44, 92, 77, 29, 78, 4, 92, 44, 68, 92,
    69, 1, 42, 89, 50, 37, 84, 63, 34, 32, 3, 17, 62, 40, 98, 82, 89, 24, 43, 85, 39, 15, 3, 99,
    29, 20, 42, 27, 10, 85, 66, 50, 35, 69, 70, 70, 74, 30, 13, 72, 54, 11, 5, 70, 55, 91, 52, 10,
    43, 43, 52,
];

const SAMPLES: [usize; 500] = [
    3, 3, 3, 3, 3, 3, 3, 0, 2, 3, 3, 3, 3, 1, 2, 2, 1, 3, 3, 2, 3, 3, 1, 1, 2, 1, 1, 3, 1, 3, 1, 2,
    0, 2, 1, 0, 3, 3, 3, 1, 3, 3, 3, 3, 1, 3, 2, 3, 2, 2, 3, 3, 3, 3, 2, 3, 3, 0, 3, 3, 3, 3, 1, 2,
    3, 3, 2, 2, 2, 1, 2, 2, 1, 2, 3, 1, 3, 0, 3, 2, 3, 3, 3, 3, 3, 3, 3, 3, 2, 3, 1, 3, 3, 2, 0, 2,
    2, 3, 1, 1, 2, 3, 2, 3, 3, 3, 3, 2, 3, 3, 3, 3, 3, 2, 3, 1, 2, 1, 1, 3, 1, 3, 2, 2, 3, 3, 3, 1,
    3, 3, 3, 3, 3, 3, 3, 3, 2, 3, 2, 3, 3, 1, 2, 3, 3, 1, 3, 2, 3, 3, 3, 2, 3, 1, 3, 0, 3, 2, 1, 1,
    3, 1, 3, 2, 3, 3, 3, 3, 2, 0, 3, 3, 1, 3, 0, 2, 1, 3, 3, 1, 1, 3, 1, 2, 3, 3, 3, 0, 2, 3, 2, 0,
    1, 3, 3, 3, 2, 2, 2, 3, 3, 3, 3, 3, 2, 3, 3, 3, 3, 2, 3, 3, 2, 0, 2, 3, 3, 3, 3, 2, 1, 1, 1, 2,
    1, 3, 3, 3, 2, 2, 3, 3, 1, 2, 3, 0, 3, 2, 3, 3, 3, 3, 0, 2, 2, 3, 2, 2, 3, 3, 3, 3, 1, 3, 2, 3,
    3, 3, 3, 3, 2, 2, 3, 1, 3, 0, 2, 1, 3, 3, 3, 3, 3, 3, 3, 3, 1, 3, 3, 3, 3, 2, 2, 2, 3, 1, 1, 3,
    2, 2, 0, 3, 2, 1, 2, 1, 0, 3, 3, 3, 2, 2, 3, 2, 1, 2, 0, 0, 3, 3, 2, 3, 3, 2, 3, 3, 3, 3, 3, 2,
    2, 2, 3, 3, 3, 3, 3, 1, 1, 3, 2, 2, 3, 1, 1, 0, 1, 3, 2, 3, 3, 2, 3, 3, 2, 3, 3, 2, 2, 2, 2, 3,
    2, 2, 2, 2, 2, 1, 2, 3, 3, 2, 2, 2, 2, 3, 3, 2, 0, 2, 1, 3, 3, 3, 3, 0, 3, 3, 3, 3, 2, 2, 3, 1,
    3, 3, 3, 2, 3, 3, 3, 2, 3, 3, 3, 3, 2, 3, 2, 1, 3, 3, 3, 3, 2, 2, 0, 1, 2, 3, 2, 0, 3, 3, 3, 3,
    3, 3, 1, 3, 3, 2, 3, 2, 2, 3, 3, 3, 3, 3, 2, 2, 3, 3, 2, 2, 2, 1, 3, 3, 3, 3, 1, 2, 3, 2, 3, 3,
    2, 3, 2, 3, 3, 3, 2, 3, 1, 2, 3, 2, 1, 1, 3, 3, 2, 3, 3, 2, 3, 3, 0, 0, 1, 3, 3, 2, 3, 3, 3, 3,
    1, 3, 3, 0, 3, 2, 3, 3, 1, 3, 3, 3, 3, 3, 3, 3, 0, 3, 3, 2,
];

const DEGREES: [usize; 1000] = [
    7, 9, 2, 1, 4, 2, 1, 1, 3, 10, 7, 1, 1, 4, 3, 8, 6, 2, 3, 2, 1, 1, 4, 5, 8, 4, 4, 1, 6, 1, 5,
    2, 3, 3, 5, 2, 1, 10, 2, 5, 1, 1, 1, 5, 5, 11, 1, 1, 8, 2, 1, 1, 2, 1, 1, 1, 1, 1, 1, 11, 1, 1,
    5, 1, 1, 1, 3, 7, 3, 3, 2, 2, 4, 2, 1, 3, 1, 1, 8, 2, 1, 1, 2, 7, 1, 1, 2, 1, 2, 1, 4, 1, 1, 1,
    2, 1, 8, 1, 5, 4, 2, 1, 1, 1, 1, 4, 1, 8, 1, 5, 4, 9, 1, 8, 6, 6, 7, 5, 4, 8, 5, 1, 2, 2, 11,
    10, 1, 4, 3, 1, 2, 1, 2, 5, 1, 6, 2, 1, 3, 1, 8, 6, 3, 8, 1, 4, 1, 7, 6, 11, 1, 6, 1, 5, 5, 1,
    3, 2, 4, 6, 3, 5, 1, 8, 1, 1, 1, 11, 3, 1, 2, 1, 4, 1, 2, 7, 5, 5, 5, 4, 6, 4, 3, 2, 3, 9, 1,
    2, 3, 1, 2, 2, 5, 1, 1, 10, 3, 7, 2, 6, 1, 1, 1, 1, 3, 9, 1, 3, 1, 8, 4, 1, 3, 2, 3, 1, 1, 2,
    4, 3, 4, 4, 4, 2, 6, 1, 7, 10, 3, 8, 1, 7, 6, 7, 1, 1, 1, 3, 11, 1, 1, 1, 2, 2, 3, 2, 8, 3, 1,
    1, 2, 1, 3, 1, 3, 10, 1, 9, 11, 10, 3, 2, 5, 6, 1, 3, 3, 5, 1, 8, 8, 1, 2, 3, 1, 7, 6, 1, 11,
    4, 9, 1, 1, 8, 1, 5, 3, 1, 8, 1, 1, 1, 3, 4, 2, 5, 1, 2, 10, 1, 8, 2, 11, 7, 4, 9, 2, 1, 1, 1,
    3, 10, 1, 2, 1, 1, 8, 1, 1, 7, 1, 2, 9, 1, 1, 1, 11, 6, 6, 1, 8, 8, 4, 5, 6, 3, 5, 1, 7, 9, 1,
    9, 2, 7, 8, 1, 1, 1, 2, 2, 2, 1, 8, 8, 2, 3, 2, 5, 8, 1, 5, 1, 3, 8, 8, 10, 2, 8, 3, 9, 3, 5,
    2, 4, 2, 2, 2, 10, 6, 2, 2, 2, 11, 5, 4, 11, 1, 6, 10, 1, 10, 8, 1, 10, 6, 1, 2, 1, 3, 5, 1, 1,
    4, 10, 2, 7, 2, 5, 8, 2, 2, 3, 11, 1, 6, 1, 6, 1, 4, 5, 1, 2, 5, 2, 1, 1, 2, 9, 2, 10, 1, 3, 1,
    10, 3, 2, 7, 6, 1, 1, 4, 3, 6, 6, 1, 2, 1, 4, 2, 1, 2, 1, 1, 1, 3, 1, 4, 7, 11, 1, 4, 5, 2, 1,
    2, 1, 9, 7, 1, 1, 2, 1, 6, 1, 1, 7, 11, 1, 1, 9, 5, 1, 1, 1, 4, 2, 1, 1, 4, 6, 2, 3, 1, 1, 1,
    2, 1, 9, 1, 7, 1, 7, 1, 1, 1, 11, 1, 11, 11, 1, 8, 3, 5, 6, 4, 3, 9, 4, 1, 3, 1, 3, 2, 1, 1, 1,
    1, 2, 1, 8, 1, 1, 6, 6, 3, 1, 8, 7, 2, 1, 2, 7, 6, 4, 3, 6, 1, 6, 3, 3, 2, 9, 9, 5, 2, 1, 2, 1,
    9, 8, 8, 3, 7, 1, 5, 1, 2, 3, 1, 5, 2, 7, 8, 5, 1, 1, 2, 1, 1, 4, 3, 3, 2, 6, 2, 2, 1, 3, 4, 1,
    2, 8, 2, 1, 4, 1, 2, 1, 2, 4, 1, 3, 1, 1, 1, 10, 1, 1, 2, 5, 11, 4, 1, 1, 1, 4, 3, 7, 1, 6, 8,
    1, 3, 5, 1, 4, 1, 7, 8, 1, 4, 1, 2, 2, 7, 3, 1, 9, 11, 7, 1, 9, 4, 5, 2, 1, 5, 2, 4, 5, 1, 4,
    2, 5, 2, 1, 10, 2, 1, 7, 4, 1, 7, 11, 5, 2, 11, 7, 6, 2, 1, 11, 3, 1, 5, 1, 1, 4, 10, 4, 1, 2,
    1, 4, 11, 3, 1, 1, 1, 7, 1, 3, 1, 1, 7, 10, 6, 3, 6, 3, 9, 1, 3, 4, 7, 4, 1, 1, 1, 5, 7, 4, 5,
    1, 6, 1, 4, 4, 8, 9, 1, 1, 2, 1, 10, 3, 1, 2, 1, 2, 3, 6, 2, 9, 1, 1, 6, 2, 3, 5, 2, 10, 5, 4,
    10, 5, 2, 1, 5, 2, 1, 4, 4, 1, 2, 1, 1, 1, 9, 3, 3, 4, 2, 6, 7, 1, 1, 8, 3, 11, 1, 1, 2, 3, 8,
    7, 11, 1, 1, 9, 3, 2, 2, 9, 3, 1, 8, 3, 7, 2, 4, 4, 1, 1, 5, 1, 1, 1, 2, 3, 10, 1, 11, 5, 3, 1,
    1, 7, 9, 1, 1, 3, 5, 7, 5, 1, 5, 1, 2, 1, 11, 2, 1, 3, 3, 1, 1, 1, 2, 7, 9, 9, 5, 1, 4, 3, 5,
    5, 8, 2, 1, 1, 2, 1, 2, 5, 4, 3, 3, 2, 4, 2, 4, 1, 8, 1, 2, 8, 3, 1, 8, 1, 1, 3, 2, 1, 1, 7, 1,
    8, 1, 1, 1, 1, 2, 3, 6, 7, 1, 4, 4, 9, 6, 3, 4, 7, 6, 10, 1, 5, 6, 2, 3, 2, 3, 2, 11, 5, 3, 3,
    6, 2, 1, 8, 5, 1, 8, 7, 2, 10, 1, 3, 1, 9, 2, 1, 10, 3, 3, 1, 1, 1, 1, 1, 8, 7, 3, 3, 1, 3, 4,
    2, 8, 5, 6, 1, 10, 7, 4, 8, 1, 1, 1, 2, 3, 10, 2, 3, 3, 5, 3, 2, 3, 3, 5, 2, 2, 7, 1, 2, 6, 1,
    1, 6, 1, 8, 7, 5, 10, 3, 9, 6, 3, 3, 11, 10, 4, 10, 5, 2, 1, 4, 1, 2, 6, 6, 3, 4, 1, 1, 2, 2,
    1, 2, 1, 1, 3, 1, 1, 3,
];

/// The 11 fragments of `makeMessage(1024)`, at the fragment length the spec's
/// own algorithm picks for min 10 / max 100.
const FRAGMENTS: [&str; 11] = [
    "916ec65cf77cadf55cd7f9cda1a1030026ddd42e905b77adc36e4f2d3ccba44f7f04f2de44f42d84c374a0e149136f25b01852545961d55f7f7a8cde6d0e2ec43f3b2dcb644a2209e8c9e34af5c4747984a5e873c9cf5f965e25ee29039f",
    "df8ca74f1c769fc07eb7ebaec46e0695aea6cbd60b3ec4bbff1b9ffe8a9e7240129377b9d3711ed38d412fbb4442256f1e6f595e0fc57fed451fb0a0101fb76b1fb1e1b88cfdfdaa946294a47de8fff173f021c0e6f65b05c0a494e50791",
    "270a0050a73ae69b6725505a2ec8a5791457c9876dd34aadd192a53aa0dc66b556c0c215c7ceb8248b717c22951e65305b56a3706e3e86eb01c803bbf915d80edcd64d4d41977fa6f78dc07eecd072aae5bc8a852397e06034dba6a0b570",
    "797c3a89b16673c94838d884923b8186ee2db5c98407cab15e13678d072b43e406ad49477c2e45e85e52ca82a94f6df7bbbe7afbed3a3a830029f29090f25217e48d1f42993a640a67916aa7480177354cc7440215ae41e4d02eae9a1912",
    "33a6d4922a792c1b7244aa879fefdb4628dc8b0923568869a983b8c661ffab9b2ed2c149e38d41fba090b94155adbed32f8b18142ff0d7de4eeef2b04adf26f2456b46775c6c20b37602df7da179e2332feba8329bbb8d727a138b4ba7a5",
    "03215eda2ef1e953d89383a382c11d3f2cad37a4ee59a91236a3e56dcf89f6ac81dd4159989c317bd649d9cbc617f73fe10033bd288c60977481a09b343d3f676070e67da757b86de27bfca74392bac2996f7822a7d8f71a489ec6180390",
    "089ea80a8fcd6526413ec6c9a339115f111d78ef21d456660aa85f790910ffa2dc58d6a5b93705caef1091474938bd312427021ad1eeafbd19e0d916ddb111fabd8dcab5ad6a6ec3a9c6973809580cb2c164e26686b5b98cfb017a337968",
    "c7daaa14ae5152a067277b1b3902677d979f8e39cc2aafb3bc06fcf69160a853e6869dcc09a11b5009f91e6b89e5b927ab1527a735660faa6012b420dd926d940d742be6a64fb01cdc0cff9faa323f02ba41436871a0eab851e7f5782d10",
    "fbefde2a7e9ae9dc1e5c2c48f74f6c824ce9ef3c89f68800d44587bedc4ab417cfb3e7447d90e1e417e6e05d30e87239d3a5d1d45993d4461e60a0192831640aa32dedde185a371ded2ae15f8a93dba8809482ce49225daadfbb0fec629e",
    "23880789bdf9ed73be57fa84d555134630e8d0f7df48349f29869a477c13ccca9cd555ac42ad7f568416c3d61959d0ed568b2b81c7771e9088ad7fd55fd4386bafbf5a528c30f107139249357368ffa980de2c76ddd9ce4191376be0e6b5",
    "170010067e2e75ebe2d2904aeb1f89d5dc98cd4a6f2faaa8be6d03354c990fd895a97feb54668473e9d942bb99e196d897e8f1b01625cf48a7b78d249bb4985c065aa8cd1402ed2ba1b6f908f63dcd84b66425df00000000000000000000",
];

/// The first 20 parts of `makeMessage(256)` at fragment length 29, as CBOR.
/// Parts 1-9 are the plain fragments; parts 10-20 are fountain mixtures.
const PARTS: [&str; 20] = [
    "8501091901001a0167aa07581d916ec65cf77cadf55cd7f9cda1a1030026ddd42e905b77adc36e4f2d3c",
    "8502091901001a0167aa07581dcba44f7f04f2de44f42d84c374a0e149136f25b01852545961d55f7f7a",
    "8503091901001a0167aa07581d8cde6d0e2ec43f3b2dcb644a2209e8c9e34af5c4747984a5e873c9cf5f",
    "8504091901001a0167aa07581d965e25ee29039fdf8ca74f1c769fc07eb7ebaec46e0695aea6cbd60b3e",
    "8505091901001a0167aa07581dc4bbff1b9ffe8a9e7240129377b9d3711ed38d412fbb4442256f1e6f59",
    "8506091901001a0167aa07581d5e0fc57fed451fb0a0101fb76b1fb1e1b88cfdfdaa946294a47de8fff1",
    "8507091901001a0167aa07581d73f021c0e6f65b05c0a494e50791270a0050a73ae69b6725505a2ec8a5",
    "8508091901001a0167aa07581d791457c9876dd34aadd192a53aa0dc66b556c0c215c7ceb8248b717c22",
    "8509091901001a0167aa07581d951e65305b56a3706e3e86eb01c803bbf915d80edcd64d4d0000000000",
    "850a091901001a0167aa07581d330f0f33a05eead4f331df229871bee733b50de71afd2e5a79f196de09",
    "850b091901001a0167aa07581d3b205ce5e52d8c24a52cffa34c564fa1af3fdffcd349dc4258ee4ee828",
    "850c091901001a0167aa07581ddd7bf725ea6c16d531b5f03254783803048ca08b87148daacd1cd7a006",
    "850d091901001a0167aa07581d760be7ad1c6187902bbc04f539b9ee5eb8ea6833222edea36031306c01",
    "850e091901001a0167aa07581d5bf4031217d2c3254b088fa7553778b5003632f46e21db129416f65b55",
    "850f091901001a0167aa07581d73f021c0e6f65b05c0a494e50791270a0050a73ae69b6725505a2ec8a5",
    "8510091901001a0167aa07581db8546ebfe2048541348910267331c643133f828afec9337c318f71b7df",
    "8511091901001a0167aa07581d23dedeea74e3a0fb052befabefa13e2f80e4315c9dceed4c8630612e64",
    "8512091901001a0167aa07581dd01a8daee769ce34b6b35d3ca0005302724abddae405bdb419c0a6b208",
    "8513091901001a0167aa07581d3171c5dc365766eff25ae47c6f10e7de48cfb8474e050e5fe997a6dc24",
    "8514091901001a0167aa07581de055c2433562184fa71b4be94f262e200f01c6f74c284b0dc6fae6673f",
];

#[test]
fn bytewords_matches_the_specification_example() {
    // BCR-2020-012: a tagged CBOR seed, its CRC-32, and its minimal Bytewords.
    let body = unhex("d99d6ca20150c7098580125e2ab0981253468b2dbc5202c11947da");
    assert_eq!(hex(&crc32(&body).to_be_bytes()), "c904f40b");
    let encoded = bytewords::encode(&body);
    assert_eq!(
        encoded,
        "tantjzoeadgdstaslplabghydrpfmkbggufgludprfgmaosecffltnsoaawkbd"
    );
    assert_eq!(bytewords::decode(&encoded).unwrap(), body);
}

#[test]
fn bytewords_rejects_a_corrupted_payload() {
    // Replace the first byte with a *different valid* Byteword, so the only
    // thing that can catch the change is the CRC-32. A corruption that happens
    // to spell a non-word would be caught by the word list instead, and would
    // not test the checksum at all.
    let encoded = bytewords::encode(b"clearsign");
    assert!(bytewords::decode(&encoded).is_ok());
    let original = &encoded[..2];
    let replacement = if original == "ae" { "ad" } else { "ae" };
    let corrupted = format!("{replacement}{}", &encoded[2..]);
    assert_ne!(corrupted, encoded);
    assert!(
        bytewords::decode(&corrupted).is_err(),
        "a changed payload with a valid word must fail its checksum"
    );
}

#[test]
fn bytewords_rejects_characters_that_are_not_words() {
    assert!(bytewords::decode("zzzzzzzzzz").is_err());
    assert!(
        bytewords::decode("tantjzoeadgdstaslplabghydrpfmkbggufgludprfgmaosecffltnsoaawkb").is_err()
    );
    assert!(bytewords::decode("").is_err());
}

#[test]
fn xoshiro_matches_the_specification() {
    let mut rng = Xoshiro256::from_seed_bytes(b"Wolf");
    let got: Vec<u64> = (0..100).map(|_| rng.next_u64() % 100).collect();
    assert_eq!(got, RNG1.to_vec());
}

#[test]
fn xoshiro_next_int_matches_the_specification() {
    // testRNG3: 100 values in 1...10.
    let expected = [
        6, 5, 8, 4, 10, 5, 7, 10, 4, 9, 10, 9, 7, 7, 1, 1, 2, 9, 9, 2, 6, 4, 5, 7, 8, 5, 4, 2, 3,
        8, 7, 4, 5, 1, 10, 9, 3, 10, 2, 6, 8, 5, 7, 9, 3, 1, 5, 2, 7, 1, 4, 4, 4, 4, 9, 4, 5, 5, 6,
        9, 5, 1, 2, 8, 3, 3, 2, 8, 4, 3, 2, 1, 10, 8, 9, 3, 10, 8, 5, 5, 6, 7, 10, 5, 8, 9, 4, 6,
        4, 2, 10, 2, 1, 7, 9, 6, 7, 4, 2, 5,
    ];
    let mut rng = Xoshiro256::from_seed_bytes(b"Wolf");
    let got: Vec<usize> = (0..100).map(|_| rng.next_int(1, 11)).collect();
    assert_eq!(got, expected.to_vec());
}

#[test]
fn random_sampler_matches_the_specification() {
    let sampler = RandomSampler::new(&[1.0, 2.0, 4.0, 8.0]).unwrap();
    let mut rng = Xoshiro256::from_seed_bytes(b"Wolf");
    let got: Vec<usize> = (0..500).map(|_| sampler.next(&mut rng)).collect();
    assert_eq!(got, SAMPLES.to_vec());
}

#[test]
fn degree_chooser_matches_the_specification() {
    // 1024-byte message at fragment length 93 gives 11 fragments.
    let mut rng = Xoshiro256::from_seed_bytes(b"Wolf");
    let got: Vec<usize> = (0..1000)
        .map(|_| choose_degree(11, &mut rng).unwrap())
        .collect();
    assert_eq!(got, DEGREES.to_vec());
}

#[test]
fn shuffle_matches_the_specification() {
    let values: Vec<usize> = (1..=10).collect();
    let expected = [
        vec![6],
        vec![6, 4],
        vec![6, 4, 9],
        vec![6, 4, 9, 3],
        vec![6, 4, 9, 3, 10],
        vec![6, 4, 9, 3, 10, 5],
        vec![6, 4, 9, 3, 10, 5, 7],
        vec![6, 4, 9, 3, 10, 5, 7, 8],
        vec![6, 4, 9, 3, 10, 5, 7, 8, 1],
        vec![6, 4, 9, 3, 10, 5, 7, 8, 1, 2],
    ];
    for (i, want) in expected.iter().enumerate() {
        let mut rng = Xoshiro256::from_seed_bytes(b"Wolf");
        assert_eq!(&shuffled(&values, &mut rng, i + 1), want);
    }
}

#[test]
fn message_generation_and_partitioning_match_the_specification() {
    let message = make_message(1024);
    let fragment_len = find_nominal_fragment_length(message.len(), 10, 100).unwrap();
    assert_eq!(fragment_len, 94);
    let got: Vec<String> = message
        .chunks(fragment_len)
        .map(|c| {
            let mut padded = c.to_vec();
            padded.resize(fragment_len, 0);
            hex(&padded)
        })
        .collect();
    assert_eq!(got, FRAGMENTS.to_vec());
}

#[test]
fn fragment_chooser_agrees_with_the_published_parts() {
    // The spec's own encoder output for makeMessage(256) at fragment length 29.
    // Parts 1..=9 must be the plain fragments in order; part 15 must repeat
    // part 7's mixture, which is what the published data shows.
    assert_eq!(choose_fragments(1, 9, 0x0167_aa07), vec![0]);
    assert_eq!(choose_fragments(9, 9, 0x0167_aa07), vec![8]);
    assert_eq!(
        choose_fragments(15, 9, 0x0167_aa07),
        choose_fragments(7, 9, 0x0167_aa07)
    );
}

#[test]
fn fragment_length_matches_the_specification() {
    assert_eq!(
        find_nominal_fragment_length(12_345, 1_005, 1_955).unwrap(),
        1_764
    );
    assert_eq!(
        find_nominal_fragment_length(12_345, 1_005, 30_000).unwrap(),
        12_345
    );
    assert_eq!(find_nominal_fragment_length(0, 10, 100), None);
    assert_eq!(find_nominal_fragment_length(100, 10, 5), None);
}

#[test]
fn part_cbor_matches_the_specification() {
    // testCBOR: [12, 8, 100, 0x12345678, h'0105030305']
    let part = unhex("850c0818641a12345678450105030305");
    let (header, data) = clearsign_qr::ur::decode_part_cbor(&part).unwrap();
    assert_eq!(header.seq_num, 12);
    assert_eq!(header.seq_len, 8);
    assert_eq!(header.message_len, 100);
    assert_eq!(header.checksum, 0x1234_5678);
    assert_eq!(data, vec![1, 5, 3, 3, 5]);
    // The spec's example is a synthetic part: 8 fragments of 5 bytes cannot hold
    // a 100-byte message, so a scanner must refuse it even though it decodes.
    let ur = format!("ur:bytes/12-8/{}", bytewords::encode(&part));
    assert!(clearsign_qr::ur::parse(&ur).is_err());
}

#[test]
fn a_message_is_reassembled_from_fountain_parts() {
    // Feed the spec's published parts, but withhold the plain fragments for
    // indexes 2 and 6 so they can only come from the mixed parts.
    let message = make_message(256);
    let mut decoder = clearsign_qr::Decoder::new();
    let mut done = None;
    for (i, part) in PARTS.iter().enumerate() {
        let seq_num = i + 1;
        if seq_num == 3 || seq_num == 7 {
            continue;
        }
        let ur = format!("ur:bytes/{}-9/{}", seq_num, bytewords::encode(&unhex(part)));
        if let Some(m) = decoder.receive(&ur).unwrap() {
            done = Some(m.to_vec());
            break;
        }
    }
    assert_eq!(done.expect("message never completed"), message);
}

#[test]
fn parts_from_a_different_message_are_refused() {
    let mut decoder = clearsign_qr::Decoder::new();
    decoder
        .receive(&format!(
            "ur:bytes/1-9/{}",
            bytewords::encode(&unhex(PARTS[0]))
        ))
        .unwrap();
    // Same shape, different checksum: a part from another transfer.
    let mut other = unhex(PARTS[1]);
    other[8] ^= 0xff;
    let err = decoder
        .receive(&format!("ur:bytes/2-9/{}", bytewords::encode(&other)))
        .unwrap_err();
    assert!(matches!(err, clearsign_qr::Error::Ur(_)), "{err}");
}
