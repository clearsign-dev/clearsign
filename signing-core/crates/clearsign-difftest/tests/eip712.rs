//! EIP-712 hashing, clearsign against alloy-dyn-abi, on random documents.
//!
//! Each case builds a random schema — up to four struct types, nested, with
//! every atomic type EIP-712 defines, dynamic and fixed arrays, and arrays of
//! structs — fills it with random values, and requires the two
//! implementations to produce the same domain separator, message hash and
//! signing hash. A schema clearsign refuses is a failure too: everything here
//! is canonical.

#![allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::panic
)]

use proptest::prelude::*;
use serde_json::{Map, Value, json};

/// xorshift64*: deterministic from the proptest seed, so failures replay.
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        self.0.wrapping_mul(0x2545_f491_4f6c_dd1d)
    }
    fn below(&mut self, n: u64) -> u64 {
        self.next() % n
    }
    fn bytes(&mut self, n: usize) -> Vec<u8> {
        (0..n).map(|_| self.next() as u8).collect()
    }
    fn hex(&mut self, n: usize) -> String {
        format!("0x{}", clearsign::hex::encode(&self.bytes(n)))
    }
}

const ATOMIC: &[&str] = &[
    "address", "bool", "string", "bytes", "uint8", "uint16", "uint48", "uint64", "uint128",
    "uint160", "uint256", "int8", "int32", "int128", "int256", "bytes1", "bytes4", "bytes20",
    "bytes32",
];

fn value_for(rng: &mut Rng, ty: &str, structs: &Map<String, Value>) -> Value {
    if let Some(open) = ty.rfind('[') {
        let inner = &ty[..open];
        let n = match &ty[open..] {
            "[]" => rng.below(4) as usize,
            fixed => fixed[1..fixed.len() - 1].parse().unwrap(),
        };
        return Value::Array((0..n).map(|_| value_for(rng, inner, structs)).collect());
    }
    if let Some(fields) = structs.get(ty) {
        let mut obj = Map::new();
        for f in fields.as_array().unwrap() {
            let name = f["name"].as_str().unwrap();
            let t = f["type"].as_str().unwrap();
            obj.insert(name.into(), value_for(rng, t, structs));
        }
        return Value::Object(obj);
    }
    match ty {
        "address" => json!(rng.hex(20)),
        "bool" => json!(rng.below(2) == 1),
        "string" => {
            let pool = [
                "",
                "Hello",
                "Ünïcödé ✓",
                "a much longer string than a single word, with commas, and digits 0123",
            ];
            json!(pool[rng.below(pool.len() as u64) as usize])
        }
        "bytes" => {
            let n = rng.below(70) as usize;
            json!(rng.hex(n))
        }
        t if t.starts_with("uint") => {
            let bits: u32 = t[4..].parse().unwrap();
            let bytes = bits as usize / 8;
            let mut raw = rng.bytes(bytes);
            match rng.below(4) {
                0 => raw.iter_mut().for_each(|b| *b = 0),
                1 => raw.iter_mut().for_each(|b| *b = 0xff),
                _ => {}
            }
            let v = alloy_primitives::U256::from_be_slice(&raw);
            if rng.below(2) == 0 {
                json!(v.to_string())
            } else {
                json!(format!("{v:#x}"))
            }
        }
        t if t.starts_with("int") => {
            let bits: u32 = t[3..].parse().unwrap();
            let raw = rng.bytes(bits as usize / 8);
            let unsigned = alloy_primitives::U256::from_be_slice(&raw);
            // Interpret as two's complement at the declared width.
            let half = alloy_primitives::U256::from(1u8) << (bits - 1);
            if unsigned >= half {
                let magnitude = (alloy_primitives::U256::from(1u8) << bits) - unsigned;
                json!(format!("-{magnitude}"))
            } else {
                json!(unsigned.to_string())
            }
        }
        t if t.starts_with("bytes") => {
            let n: usize = t[5..].parse().unwrap();
            json!(rng.hex(n))
        }
        other => panic!("no generator for {other}"),
    }
}

fn random_doc(seed: u64) -> Value {
    let mut rng = Rng(seed | 1);
    let n_structs = 1 + rng.below(4) as usize;
    let names: Vec<String> = (0..n_structs).map(|i| format!("S{i}")).collect();
    let mut structs = Map::new();
    for (i, name) in names.iter().enumerate() {
        let n_fields = rng.below(5) as usize;
        let mut fields = Vec::new();
        for f in 0..n_fields {
            // Refer only to later structs, so the schema is a finite tree.
            let mut ty = if i + 1 < names.len() && rng.below(3) == 0 {
                names[i + 1 + rng.below((names.len() - i - 1) as u64) as usize].clone()
            } else {
                ATOMIC[rng.below(ATOMIC.len() as u64) as usize].to_string()
            };
            match rng.below(6) {
                0 => ty.push_str("[]"),
                1 => ty.push_str(&format!("[{}]", 1 + rng.below(3))),
                _ => {}
            }
            fields.push(json!({"name": format!("f{f}"), "type": ty}));
        }
        structs.insert(name.clone(), Value::Array(fields));
    }
    let message = value_for(&mut rng, &names[0], &structs);

    // A domain using a canonical subset, in canonical order.
    let mut domain_types = Vec::new();
    let mut domain = Map::new();
    if rng.below(2) == 0 {
        domain_types.push(json!({"name": "name", "type": "string"}));
        domain.insert("name".into(), json!("Random Domain"));
    }
    if rng.below(2) == 0 {
        domain_types.push(json!({"name": "version", "type": "string"}));
        domain.insert("version".into(), json!("1"));
    }
    if rng.below(3) != 0 {
        domain_types.push(json!({"name": "chainId", "type": "uint256"}));
        domain.insert("chainId".into(), json!(rng.below(100_000) + 1));
    }
    if rng.below(3) != 0 {
        domain_types.push(json!({"name": "verifyingContract", "type": "address"}));
        domain.insert("verifyingContract".into(), json!(rng.hex(20)));
    }
    if rng.below(4) == 0 {
        domain_types.push(json!({"name": "salt", "type": "bytes32"}));
        domain.insert("salt".into(), json!(rng.hex(32)));
    }
    structs.insert("EIP712Domain".into(), Value::Array(domain_types));
    json!({"types": structs, "primaryType": names[0], "domain": domain, "message": message})
}

proptest! {
    #![proptest_config(ProptestConfig { cases: 5000, .. ProptestConfig::default() })]

    #[test]
    fn eip712_hashes_match_alloy(seed in any::<u64>()) {
        let doc = random_doc(seed);
        let text = doc.to_string();
        let ours = clearsign_safe_json::typed_data::parse(&text)
            .map_err(|e| TestCaseError::fail(format!("refused a canonical document: {e}\n{text}")))
            .and_then(|td| clearsign::typed::hash_typed_data(&td).map_err(|e| TestCaseError::fail(format!("{e}\n{text}"))))?;
        let theirs: alloy_dyn_abi::TypedData = serde_json::from_value(doc.clone()).unwrap();
        let signing = theirs.eip712_signing_hash().unwrap();
        prop_assert_eq!(ours.signing_hash, signing.0, "signing hash differs for\n{}", text);
        prop_assert_eq!(ours.domain_separator, theirs.domain().separator().0);
        prop_assert_eq!(ours.message_hash.unwrap(), theirs.hash_struct().unwrap().0);
        // And the review never fails on what hashing accepted.
        let td = clearsign_safe_json::typed_data::parse(&text).unwrap();
        prop_assert!(clearsign::typed::review_typed_data(&td).is_ok());
    }
}
