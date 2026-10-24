//! EIP-712 typed data: hashed exactly as the standard specifies, and reviewed.
//!
//! A typed-data signature needs no transaction. Whoever holds it can submit it,
//! today or months from now, and the contract that checks it does whatever the
//! structure says. That is why wallet drainers ask for these more than for
//! anything else: an ERC-2612 `Permit` or a Permit2 `PermitSingle` moves tokens
//! without the victim ever sending a transaction they might look at twice.
//!
//! The rules here are the same as everywhere else in this crate:
//!
//! - Everything shown comes from the signed structure itself. The domain's
//!   `name` is text inside the signed message, chosen by whoever prepared it;
//!   the token is identified by `verifyingContract`, never by that text.
//! - Structures the reviewer understands are named field by field: ERC-2612
//!   permits, DAI-style permits, Permit2's four signature types, and Safe
//!   transactions. Anything else is still displayed field by field, but is
//!   BLIND: what signing it permits is decided by code these bytes do not show.
//! - Anything not encoded exactly as the standard requires is refused rather
//!   than guessed at: undeclared or missing fields, non-canonical type names,
//!   values out of range for their type, duplicate names, unbounded nesting.

use alloc::boxed::Box;
use alloc::collections::BTreeSet;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use core::fmt;

use crate::address::{self, Address};
use crate::calls::PERMIT2;
use crate::keccak::{keccak256, keccak256_concat};
use crate::review::{Review, Section, Severity};
use crate::text::escape_untrusted;
use crate::{U256, hex};

/// Structs nested deeper than this are refused.
pub const MAX_DEPTH: usize = 16;
/// More declared types than this are refused.
pub const MAX_TYPES: usize = 64;
/// More fields in one struct than this are refused.
pub const MAX_FIELDS: usize = 128;
/// Longer arrays than this are refused.
pub const MAX_ARRAY: usize = 1024;
/// More array dimensions than this in one type (`T[][][]...`) are refused,
/// before the type is parsed: parsing peels one dimension per level, and an
/// unbounded count was a stack overflow waiting for a long enough request.
pub const MAX_ARRAY_DIMENSIONS: usize = 8;
/// Displayed values in a generic review are capped at this many; the rest are
/// counted, and the review is BLIND either way.
pub const MAX_GENERIC_LINES: usize = 256;

/// A JSON value as the typed data carried it. Numbers keep their sign and fit
/// in 64 bits; anything larger must arrive as a string, so no value is ever
/// rounded on its way in.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Null,
    Bool(bool),
    Int {
        negative: bool,
        magnitude: u64,
    },
    Str(String),
    Array(Vec<Value>),
    /// Keys in the order given; the reader refuses duplicates.
    Object(Vec<(String, Value)>),
}

impl Value {
    fn get(&self, key: &str) -> Option<&Value> {
        match self {
            Value::Object(fields) => fields.iter().find(|(k, _)| k == key).map(|(_, v)| v),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Field {
    pub name: String,
    pub ty: String,
}

/// An `eth_signTypedData_v4` request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypedData {
    pub types: Vec<(String, Vec<Field>)>,
    pub primary_type: String,
    pub domain: Value,
    pub message: Value,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypedDataError(pub String);

impl fmt::Display for TypedDataError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

fn err<T>(msg: String) -> Result<T, TypedDataError> {
    Err(TypedDataError(msg))
}

// ------------------------------------------------------------------ types

#[derive(Debug, Clone, PartialEq, Eq)]
enum Ty<'a> {
    Address,
    Bool,
    String,
    Bytes,
    Uint(usize),
    Int(usize),
    FixedBytes(usize),
    Struct(&'a str),
    Array(Box<Ty<'a>>, Option<usize>),
}

fn is_identifier(s: &str) -> bool {
    let mut chars = s.chars();
    match chars.next() {
        Some(c) if c.is_ascii_alphabetic() || c == '_' || c == '$' => {}
        _ => return false,
    }
    chars.all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '$')
}

/// Parse a field type. Arrays bind right to left, as in Solidity: `T[][3]` is
/// three dynamic arrays of `T`.
fn parse_ty(s: &str) -> Result<Ty<'_>, TypedDataError> {
    if s.bytes().filter(|b| *b == b'[').count() > MAX_ARRAY_DIMENSIONS {
        return err(format!(
            "a type has more than {MAX_ARRAY_DIMENSIONS} array dimensions"
        ));
    }
    if let Some(open) = s.rfind('[') {
        let inner = s.get(..open).unwrap_or("");
        let dims = s.get(open..).unwrap_or("");
        let len = match dims {
            "[]" => None,
            d => {
                let digits = d
                    .strip_prefix('[')
                    .and_then(|r| r.strip_suffix(']'))
                    .unwrap_or("");
                if digits.is_empty()
                    || digits.starts_with('0')
                    || !digits.bytes().all(|b| b.is_ascii_digit())
                {
                    return err(format!("{s:?} is not a valid array type"));
                }
                let n: usize = digits
                    .parse()
                    .map_err(|_| TypedDataError(format!("{s:?} is too long an array")))?;
                if n > MAX_ARRAY {
                    return err(format!(
                        "{s:?} is longer than the {MAX_ARRAY} elements accepted"
                    ));
                }
                Some(n)
            }
        };
        return Ok(Ty::Array(Box::new(parse_ty(inner)?), len));
    }
    let sized = |prefix: &str| -> Option<usize> {
        let digits = s.strip_prefix(prefix)?;
        if digits.is_empty()
            || digits.starts_with('0')
            || !digits.bytes().all(|b| b.is_ascii_digit())
        {
            return None;
        }
        digits.parse().ok()
    };
    Ok(match s {
        "address" => Ty::Address,
        "bool" => Ty::Bool,
        "string" => Ty::String,
        "bytes" => Ty::Bytes,
        // The short aliases are Solidity conveniences; EIP-712 type strings
        // must be canonical, because they are hashed as written.
        "uint" | "int" | "byte" => {
            return err(format!(
                "{s:?} is not a canonical EIP-712 type; write it with its size"
            ));
        }
        _ => {
            if let Some(bits) = sized("uint") {
                if bits % 8 != 0 || bits > 256 {
                    return err(format!("{s:?} is not a valid integer type"));
                }
                Ty::Uint(bits)
            } else if let Some(bits) = sized("int") {
                if bits % 8 != 0 || bits > 256 {
                    return err(format!("{s:?} is not a valid integer type"));
                }
                Ty::Int(bits)
            } else if let Some(n) = sized("bytes") {
                if n > 32 {
                    return err(format!("{s:?} is not a valid fixed-size bytes type"));
                }
                Ty::FixedBytes(n)
            } else if is_identifier(s) {
                Ty::Struct(s)
            } else {
                return err(format!("{s:?} is not a type"));
            }
        }
    })
}

struct Types<'a> {
    defs: &'a [(String, Vec<Field>)],
    /// `keccak256(encodeType(name))` for every declared type, in `defs` order,
    /// computed once per request. Recomputing it for every struct value let a
    /// 1 MiB request with many values and a wide dependency tree cost minutes.
    typehashes: Vec<[u8; 32]>,
}

impl<'a> Types<'a> {
    /// Validate every declaration, then hash every type once.
    fn new(defs: &'a [(String, Vec<Field>)]) -> Result<Self, TypedDataError> {
        let mut t = Types {
            defs,
            typehashes: Vec::new(),
        };
        t.validate()?;
        let mut hashes = Vec::with_capacity(defs.len());
        for (name, _) in defs {
            hashes.push(keccak256(t.encode_type(name)?.as_bytes()));
        }
        t.typehashes = hashes;
        Ok(t)
    }

    fn typehash(&self, name: &str) -> Result<[u8; 32], TypedDataError> {
        self.defs
            .iter()
            .position(|(n, _)| n == name)
            .and_then(|i| self.typehashes.get(i).copied())
            .ok_or_else(|| TypedDataError(format!("type {name:?} is used but never declared")))
    }

    fn fields(&self, name: &str) -> Result<&'a [Field], TypedDataError> {
        self.defs
            .iter()
            .find(|(n, _)| n == name)
            .map(|(_, f)| f.as_slice())
            .ok_or_else(|| TypedDataError(format!("type {name:?} is used but never declared")))
    }

    /// Check every declaration once, before anything is hashed.
    fn validate(&self) -> Result<(), TypedDataError> {
        if self.defs.len() > MAX_TYPES {
            return err(format!("more than {MAX_TYPES} types are declared"));
        }
        let mut seen = BTreeSet::new();
        for (name, fields) in self.defs {
            if !is_identifier(name) {
                return err(format!("{name:?} is not a valid type name"));
            }
            if !seen.insert(name.as_str()) {
                return err(format!("type {name:?} is declared twice"));
            }
            if parse_ty(name)
                .map(|t| !matches!(t, Ty::Struct(_)))
                .unwrap_or(true)
            {
                return err(format!("{name:?} is the name of a built-in type"));
            }
            if fields.len() > MAX_FIELDS {
                return err(format!("type {name:?} has more than {MAX_FIELDS} fields"));
            }
            let mut names = BTreeSet::new();
            for f in fields {
                if !is_identifier(&f.name) {
                    return err(format!("{:?} in {name} is not a valid field name", f.name));
                }
                if !names.insert(f.name.as_str()) {
                    return err(format!("field {:?} appears twice in {name}", f.name));
                }
                let mut t = parse_ty(&f.ty)?;
                while let Ty::Array(inner, _) = t {
                    t = *inner;
                }
                if let Ty::Struct(s) = t {
                    self.fields(s)?;
                }
            }
        }
        Ok(())
    }

    /// The struct types `name` refers to, directly or not, excluding itself.
    fn dependencies(
        &self,
        name: &str,
        out: &mut BTreeSet<String>,
        depth: usize,
    ) -> Result<(), TypedDataError> {
        if depth > MAX_DEPTH {
            return err(format!(
                "types are nested more than {MAX_DEPTH} levels deep"
            ));
        }
        for f in self.fields(name)? {
            let mut t = parse_ty(&f.ty)?;
            while let Ty::Array(inner, _) = t {
                t = *inner;
            }
            if let Ty::Struct(s) = t {
                if out.insert(String::from(s)) {
                    self.dependencies(s, out, depth.saturating_add(1))?;
                }
            }
        }
        Ok(())
    }

    /// `encodeType`: the struct, then everything it refers to, alphabetically.
    fn encode_type(&self, name: &str) -> Result<String, TypedDataError> {
        let mut deps = BTreeSet::new();
        self.dependencies(name, &mut deps, 0)?;
        deps.remove(name);
        let mut out = self.type_string(name)?;
        for d in deps {
            out.push_str(&self.type_string(&d)?);
        }
        Ok(out)
    }

    fn type_string(&self, name: &str) -> Result<String, TypedDataError> {
        let fields = self.fields(name)?;
        let inner: Vec<String> = fields
            .iter()
            .map(|f| format!("{} {}", f.ty, f.name))
            .collect();
        Ok(format!("{name}({})", inner.join(",")))
    }

    fn hash_struct(
        &self,
        name: &str,
        value: &Value,
        depth: usize,
    ) -> Result<[u8; 32], TypedDataError> {
        if depth > MAX_DEPTH {
            return err(format!(
                "values are nested more than {MAX_DEPTH} levels deep"
            ));
        }
        let fields = self.fields(name)?;
        let Value::Object(given) = value else {
            return err(format!("a {name} must be an object"));
        };
        // Every declared field, and nothing else. An undeclared field is not
        // hashed, so a screen that showed it would show something unsigned.
        for (k, _) in given {
            if !fields.iter().any(|f| &f.name == k) {
                return err(format!(
                    "{name} has a field {k:?} its type does not declare"
                ));
            }
        }
        let mut words: Vec<[u8; 32]> = Vec::with_capacity(fields.len().saturating_add(1));
        words.push(self.typehash(name)?);
        for f in fields {
            let v = value.get(&f.name).ok_or_else(|| {
                TypedDataError(format!("{name} is missing its field {:?}", f.name))
            })?;
            words.push(self.encode_value(&parse_ty(&f.ty)?, v, depth)?);
        }
        let parts: Vec<&[u8]> = words.iter().map(|w| w.as_slice()).collect();
        Ok(keccak256_concat(&parts))
    }

    fn encode_value(
        &self,
        ty: &Ty<'_>,
        v: &Value,
        depth: usize,
    ) -> Result<[u8; 32], TypedDataError> {
        match ty {
            Ty::Struct(name) => self.hash_struct(name, v, depth.saturating_add(1)),
            Ty::Array(inner, len) => {
                let Value::Array(items) = v else {
                    return err(String::from(
                        "an array type was given something that is not an array",
                    ));
                };
                if items.len() > MAX_ARRAY {
                    return err(format!("an array has more than {MAX_ARRAY} elements"));
                }
                if let Some(n) = len {
                    if items.len() != *n {
                        return err(format!(
                            "a fixed array of {n} was given {} elements",
                            items.len()
                        ));
                    }
                }
                let mut words = Vec::with_capacity(items.len());
                for item in items {
                    words.push(self.encode_value(inner, item, depth.saturating_add(1))?);
                }
                let parts: Vec<&[u8]> = words.iter().map(|w| w.as_slice()).collect();
                Ok(keccak256_concat(&parts))
            }
            Ty::Bytes => Ok(keccak256(&bytes_value(v)?)),
            Ty::String => match v {
                Value::Str(s) => Ok(keccak256(s.as_bytes())),
                _ => err(String::from(
                    "a string field was given something that is not a string",
                )),
            },
            atomic => Ok(atomic_word(atomic, v)?.0),
        }
    }
}

/// The digits of a hex string written the one way the standard writes it: a
/// single `0x` prefix, then hex digits and nothing else. `hex::decode` is
/// lenient for its other callers (it trims whitespace, and the prefix is
/// optional, so `0x0x12` reads as `0x12`). A signature request gets no such
/// leniency: two tools reading one string two ways is how a wallet signs a
/// hash other than the one shown.
fn hex_digits(s: &str) -> Option<&str> {
    let digits = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X"))?;
    digits
        .bytes()
        .all(|b| b.is_ascii_hexdigit())
        .then_some(digits)
}

fn bytes_value(v: &Value) -> Result<Vec<u8>, TypedDataError> {
    match v {
        Value::Str(s) => match hex_digits(s) {
            Some(digits) if digits.len() % 2 == 0 => {
                hex::decode(digits).map_err(|_| TypedDataError(format!("{s:?} is not hexadecimal")))
            }
            _ => err(format!(
                "{s:?} is not a 0x-prefixed hex string of whole bytes"
            )),
        },
        _ => err(String::from(
            "a bytes field must be a 0x-prefixed hex string",
        )),
    }
}

/// An unsigned integer from a JSON number, a decimal string or a 0x hex string.
fn uint_value(v: &Value) -> Result<U256, TypedDataError> {
    match v {
        Value::Int {
            negative: false,
            magnitude,
        } => Ok(U256::from_u64(*magnitude)),
        Value::Int { negative: true, .. } => err(String::from(
            "a negative number was given for an unsigned type",
        )),
        Value::Str(s) => {
            if s.starts_with("0x") || s.starts_with("0X") {
                let h = hex_digits(s)
                    .filter(|digits| !digits.is_empty())
                    .ok_or_else(|| TypedDataError(format!("{s:?} is not a number")))?;
                let padded = if h.len() % 2 == 1 {
                    format!("0{h}")
                } else {
                    String::from(h)
                };
                let bytes = hex::decode(&padded)
                    .map_err(|_| TypedDataError(format!("{s:?} is not a number")))?;
                // Leading zero bytes are harmless in hex; strip them so a long
                // zero-padded value is not mistaken for an oversized one.
                let first = bytes.iter().position(|b| *b != 0).unwrap_or(bytes.len());
                U256::from_be_slice(bytes.get(first..).unwrap_or(&[]))
                    .map_err(|_| TypedDataError(format!("{s:?} does not fit in 256 bits")))
            } else {
                U256::from_decimal(s)
                    .map_err(|_| TypedDataError(format!("{s:?} is not a whole number")))
            }
        }
        _ => err(String::from(
            "a number field was given something that is not a number",
        )),
    }
}

/// A signed integer as a 256-bit two's-complement word.
fn int_word(v: &Value, bits: usize) -> Result<[u8; 32], TypedDataError> {
    let (negative, magnitude) = match v {
        Value::Int {
            negative,
            magnitude,
        } => (*negative, U256::from_u64(*magnitude)),
        Value::Str(s) => match s.strip_prefix('-') {
            Some(rest) => (true, uint_value(&Value::Str(String::from(rest)))?),
            None => (false, uint_value(v)?),
        },
        _ => {
            return err(String::from(
                "a number field was given something that is not a number",
            ));
        }
    };
    // Range: -2^(bits-1) <= x < 2^(bits-1).
    let fits_below = |m: &U256, allow_equal: bool| -> bool {
        // m < 2^(bits-1), or m == 2^(bits-1) when negative.
        let limit = pow2(bits.saturating_sub(1));
        match m.cmp(&limit) {
            core::cmp::Ordering::Less => true,
            core::cmp::Ordering::Equal => allow_equal,
            core::cmp::Ordering::Greater => false,
        }
    };
    if !fits_below(&magnitude, negative) {
        return err(format!("a value does not fit in int{bits}"));
    }
    if !negative || magnitude.is_zero() {
        return Ok(magnitude.0);
    }
    Ok(twos_complement(&magnitude).0)
}

fn pow2(n: usize) -> U256 {
    let mut out = [0u8; 32];
    let byte = 31usize.saturating_sub(n / 8);
    if let Some(b) = out.get_mut(byte) {
        *b = 1u8 << (n % 8);
    }
    U256(out)
}

fn twos_complement(v: &U256) -> U256 {
    let mut out = [0u8; 32];
    let mut carry = 1u16;
    for i in (0..32).rev() {
        let inverted = u16::from(!v.0.get(i).copied().unwrap_or(0));
        let sum = inverted.saturating_add(carry);
        if let Some(o) = out.get_mut(i) {
            *o = (sum & 0xff) as u8;
        }
        carry = sum >> 8;
    }
    U256(out)
}

fn address_value(v: &Value) -> Result<Address, TypedDataError> {
    match v {
        Value::Str(s) => {
            let bytes = hex_digits(s)
                .filter(|digits| digits.len() == 40)
                .and_then(|digits| hex::decode(digits).ok())
                .ok_or_else(|| {
                    TypedDataError(format!("{s:?} is not a 0x-prefixed 20-byte address"))
                })?;
            let mut a = [0u8; 20];
            a.copy_from_slice(&bytes);
            Ok(a)
        }
        _ => err(String::from(
            "an address field was given something that is not a string",
        )),
    }
}

fn atomic_word(ty: &Ty<'_>, v: &Value) -> Result<U256, TypedDataError> {
    match ty {
        Ty::Address => {
            let a = address_value(v)?;
            let mut w = [0u8; 32];
            if let Some(tail) = w.get_mut(12..) {
                tail.copy_from_slice(&a);
            }
            Ok(U256(w))
        }
        Ty::Bool => match v {
            Value::Bool(b) => Ok(U256::from_u64(u64::from(*b))),
            _ => err(String::from(
                "a bool field was given something that is not true or false",
            )),
        },
        Ty::Uint(bits) => {
            let n = uint_value(v)?;
            let zero = 32usize.saturating_sub(bits / 8);
            if n.0.iter().take(zero).any(|b| *b != 0) {
                return err(format!("a value does not fit in uint{bits}"));
            }
            Ok(n)
        }
        Ty::Int(bits) => Ok(U256(int_word(v, *bits)?)),
        Ty::FixedBytes(n) => {
            let bytes = bytes_value(v)?;
            if bytes.len() != *n {
                return err(format!("a bytes{n} field was given {} bytes", bytes.len()));
            }
            let mut w = [0u8; 32];
            if let Some(head) = w.get_mut(..*n) {
                head.copy_from_slice(&bytes);
            }
            Ok(U256(w))
        }
        _ => err(String::from("not an atomic type")),
    }
}

// ------------------------------------------------------------------ hashing

/// The three hashes a typed-data signature is built from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TypedDataHashes {
    pub domain_separator: [u8; 32],
    /// `None` when the primary type is `EIP712Domain` itself, which signs the
    /// domain alone.
    pub message_hash: Option<[u8; 32]>,
    /// `keccak256(0x19 || 0x01 || domainSeparator || hashStruct(message))`.
    pub signing_hash: [u8; 32],
}

pub fn hash_typed_data(td: &TypedData) -> Result<TypedDataHashes, TypedDataError> {
    hashes_with(&Types::new(&td.types)?, td)
}

fn hashes_with(types: &Types<'_>, td: &TypedData) -> Result<TypedDataHashes, TypedDataError> {
    let domain_separator = types.hash_struct("EIP712Domain", &td.domain, 0)?;
    if td.primary_type == "EIP712Domain" {
        return Ok(TypedDataHashes {
            domain_separator,
            message_hash: None,
            signing_hash: keccak256_concat(&[&[0x19, 0x01], &domain_separator]),
        });
    }
    let message_hash = types.hash_struct(&td.primary_type, &td.message, 0)?;
    Ok(TypedDataHashes {
        domain_separator,
        message_hash: Some(message_hash),
        signing_hash: keccak256_concat(&[&[0x19, 0x01], &domain_separator, &message_hash]),
    })
}

// ------------------------------------------------------------------ review

const PERMIT: &str =
    "Permit(address owner,address spender,uint256 value,uint256 nonce,uint256 deadline)";
const DAI_PERMIT: &str =
    "Permit(address holder,address spender,uint256 nonce,uint256 expiry,bool allowed)";
const PERMIT_DETAILS: &str =
    "PermitDetails(address token,uint160 amount,uint48 expiration,uint48 nonce)";
const TOKEN_PERMISSIONS: &str = "TokenPermissions(address token,uint256 amount)";

/// Review a typed-data request: its domain, what the structure permits, and the
/// hashes to compare. The review is for display; it carries no signing target.
pub fn review_typed_data(td: &TypedData) -> Result<Review, TypedDataError> {
    review_and_hash_typed_data(td).map(|(review, _)| review)
}

/// [`review_typed_data`] together with the hashes it shows, each computed once.
pub fn review_and_hash_typed_data(
    td: &TypedData,
) -> Result<(Review, TypedDataHashes), TypedDataError> {
    let types = Types::new(&td.types)?;
    let hashes = hashes_with(&types, td)?;
    let review = review_with(td, &types, &hashes)?;
    Ok((review, hashes))
}

fn review_with(
    td: &TypedData,
    types: &Types<'_>,
    hashes: &TypedDataHashes,
) -> Result<Review, TypedDataError> {
    // The domain: who checks the signature, and on which chain.
    let mut d = Section::new("Signing domain");
    let domain_fields = types.fields("EIP712Domain")?;
    let mut verifying: Option<Address> = None;
    let mut chain_bound = false;
    for f in domain_fields {
        let Some(v) = td.domain.get(&f.name) else {
            continue;
        };
        match (f.name.as_str(), parse_ty(&f.ty)?) {
            ("verifyingContract", Ty::Address) => {
                let a = address_value(v)?;
                verifying = Some(a);
                d.field("Verifying contract", address::display(&a));
            }
            ("chainId", Ty::Uint(_)) => {
                chain_bound = true;
                d.field("Chain ID", uint_value(v)?.to_decimal());
            }
            ("name", Ty::String) => {
                if let Value::Str(s) = v {
                    d.field(
                        "Name (text in the request, not an identity)",
                        escape_untrusted(s),
                    );
                }
            }
            ("version", Ty::String) => {
                if let Value::Str(s) = v {
                    d.field("Version", escape_untrusted(s));
                }
            }
            ("salt", Ty::FixedBytes(32)) => d.field("Salt", hex::encode_prefixed(&bytes_value(v)?)),
            (other, _) => d.field(other, generic_text(&parse_ty(&f.ty)?, v)?),
        }
    }
    let encoded = if td.primary_type == "EIP712Domain" {
        String::new()
    } else {
        types.encode_type(&td.primary_type)?
    };
    if encoded == crate::safe::SAFE_TX_TYPE {
        let mut review = safe_tx(td, verifying, hashes)?;
        review.sections.insert(0, d);
        unverified_typed_contract(&mut review, verifying, "SafeTx");
        no_transaction_finding(&mut review);
        return Ok(review);
    }

    let mut review = Review::new("EIP-712 typed data (signature)");
    review.sections.push(d);
    if !chain_bound {
        review.find(
            Severity::Warning,
            "TYPED_DATA_NOT_CHAIN_BOUND",
            String::from(
                "This domain names no chain, so the signature may be accepted on every chain where \
                 the verifying contract's address holds the same code.",
            ),
        );
    }

    let m = &td.message;
    match encoded.as_str() {
        "" => {
            review.find(
                Severity::Blind,
                "DOMAIN_ONLY_SIGNATURE",
                String::from("This signs the domain alone, with no message. What a contract does with that is not established here."),
            );
        }
        e if e == PERMIT => {
            permit(&mut review, verifying, m)?;
            unverified_typed_contract(&mut review, verifying, "ERC-2612 Permit");
        }
        e if e == DAI_PERMIT => {
            dai_permit(&mut review, verifying, m)?;
            unverified_typed_contract(&mut review, verifying, "DAI-style Permit");
        }
        e if e
            == alloc::format!(
                "PermitSingle(PermitDetails details,address spender,uint256 sigDeadline){PERMIT_DETAILS}"
            ) =>
        {
            permit2_allowance(&mut review, verifying, m, false)?
        }
        e if e
            == alloc::format!(
                "PermitBatch(PermitDetails[] details,address spender,uint256 sigDeadline){PERMIT_DETAILS}"
            ) =>
        {
            permit2_allowance(&mut review, verifying, m, true)?
        }
        e if e
            == alloc::format!(
                "PermitTransferFrom(TokenPermissions permitted,address spender,uint256 nonce,uint256 deadline){TOKEN_PERMISSIONS}"
            ) =>
        {
            permit2_transfer(&mut review, verifying, m, false)?
        }
        e if e
            == alloc::format!(
                "PermitBatchTransferFrom(TokenPermissions[] permitted,address spender,uint256 nonce,uint256 deadline){TOKEN_PERMISSIONS}"
            ) =>
        {
            permit2_transfer(&mut review, verifying, m, true)?
        }
        _ => generic(&mut review, types, td)?,
    }

    review.digests.push((
        String::from("EIP-712 signing hash"),
        hex::encode_prefixed(&hashes.signing_hash),
    ));
    review.digests.push((
        String::from("Domain separator"),
        hex::encode_prefixed(&hashes.domain_separator),
    ));
    if let Some(h) = hashes.message_hash {
        review
            .digests
            .push((String::from("Message hash"), hex::encode_prefixed(&h)));
    }
    no_transaction_finding(&mut review);
    Ok(review)
}

fn no_transaction_finding(review: &mut Review) {
    review.find(
        Severity::Info,
        "SIGNATURE_NEEDS_NO_TRANSACTION",
        String::from(
            "A typed-data signature is not a transaction. Whoever holds it can submit it later, \
             from any account, until it expires or its nonce is used.",
        ),
    );
}

fn unverified_typed_contract(review: &mut Review, verifying: Option<Address>, structure: &str) {
    let contract = verifying
        .map(|address| address::checksummed(&address))
        .unwrap_or_else(|| String::from("no verifying contract"));
    review.find(
        Severity::Blind,
        "TYPED_SCHEMA_NOT_BEHAVIOUR",
        format!(
            "The message has the {structure} field layout, but that layout does not prove what {contract} does with the signature. ClearSign has not verified the contract code."
        ),
    );
}

fn field_addr(m: &Value, k: &str) -> Result<Address, TypedDataError> {
    address_value(
        m.get(k)
            .ok_or_else(|| TypedDataError(format!("missing {k}")))?,
    )
}

fn field_uint(m: &Value, k: &str) -> Result<U256, TypedDataError> {
    uint_value(
        m.get(k)
            .ok_or_else(|| TypedDataError(format!("missing {k}")))?,
    )
}

fn token_of(verifying: Option<Address>) -> String {
    match verifying {
        Some(a) => address::checksummed(&a),
        None => String::from("the token this domain names"),
    }
}

fn permit(
    review: &mut Review,
    verifying: Option<Address>,
    m: &Value,
) -> Result<(), TypedDataError> {
    let owner = field_addr(m, "owner")?;
    let spender = field_addr(m, "spender")?;
    let value = field_uint(m, "value")?;
    let deadline = field_uint(m, "deadline")?;
    let mut s = Section::new("Token permission (ERC-2612 permit)");
    s.field(
        "Token (the verifying contract)",
        verifying.map(|a| address::display(&a)).unwrap_or_default(),
    );
    s.field("Owner", address::display(&owner));
    s.field("Spender", address::display(&spender));
    s.field("Nonce", field_uint(m, "nonce")?.to_decimal());
    s.field("Deadline", crate::calls::unix_time(&deadline));
    allowance(review, &mut s, value, &spender, &token_of(verifying), false);
    review.sections.push(s);
    Ok(())
}

fn dai_permit(
    review: &mut Review,
    verifying: Option<Address>,
    m: &Value,
) -> Result<(), TypedDataError> {
    let holder = field_addr(m, "holder")?;
    let spender = field_addr(m, "spender")?;
    let allowed = match m.get("allowed") {
        Some(Value::Bool(b)) => *b,
        _ => return err(String::from("allowed must be true or false")),
    };
    let mut s = Section::new("Token permission (DAI-style permit)");
    s.field(
        "Token (the verifying contract)",
        verifying.map(|a| address::display(&a)).unwrap_or_default(),
    );
    s.field("Holder", address::display(&holder));
    s.field("Spender", address::display(&spender));
    s.field("Nonce", field_uint(m, "nonce")?.to_decimal());
    let expiry = field_uint(m, "expiry")?;
    s.field(
        "Expiry",
        if expiry.is_zero() {
            String::from("0 (never expires)")
        } else {
            crate::calls::unix_time(&expiry)
        },
    );
    if allowed {
        s.field("Allowance", String::from("UNLIMITED (allowed = true)"));
        review.find(
            Severity::Critical,
            "UNLIMITED_APPROVAL",
            format!(
                "Signing this lets {} move ALL of {} from {}, without any transaction from you.",
                address::checksummed(&spender),
                token_of(verifying),
                address::checksummed(&holder)
            ),
        );
    } else {
        s.field("Allowance", String::from("0 (allowed = false: revokes)"));
    }
    review.sections.push(s);
    Ok(())
}

fn check_permit2_domain(review: &mut Review, verifying: Option<Address>) {
    if verifying != Some(PERMIT2) {
        review.find(
            Severity::Blind,
            "NOT_PERMIT2_ADDRESS",
            format!(
                "This is the shape of a Permit2 signature, but its verifying contract is not Permit2's \
                 address ({}). A contract imitating Permit2 can do anything with this signature.",
                address::checksummed(&PERMIT2)
            ),
        );
    }
}

fn permit2_allowance(
    review: &mut Review,
    verifying: Option<Address>,
    m: &Value,
    batch: bool,
) -> Result<(), TypedDataError> {
    check_permit2_domain(review, verifying);
    let spender = field_addr(m, "spender")?;
    let deadline = field_uint(m, "sigDeadline")?;
    let details: Vec<&Value> = match (batch, m.get("details")) {
        (false, Some(d)) => alloc::vec![d],
        (true, Some(Value::Array(items))) => items.iter().collect(),
        _ => return err(String::from("details are missing")),
    };
    let total = details.len();
    for (i, d) in details.into_iter().enumerate() {
        let token = field_addr(d, "token")?;
        let amount = field_uint(d, "amount")?;
        let title = if batch {
            format!("Permit2 allowance {} of {total}", i.saturating_add(1))
        } else {
            String::from("Permit2 allowance")
        };
        let mut s = Section::new(&title);
        s.field("Token", address::display(&token));
        s.field("Spender", address::display(&spender));
        s.field(
            "Allowance expires",
            crate::calls::permit2_expiry(&field_uint(d, "expiration")?),
        );
        s.field("Signature valid until", crate::calls::unix_time(&deadline));
        s.field("Nonce", field_uint(d, "nonce")?.to_decimal());
        allowance(
            review,
            &mut s,
            amount,
            &spender,
            &address::checksummed(&token),
            true,
        );
        review.sections.push(s);
    }
    Ok(())
}

fn permit2_transfer(
    review: &mut Review,
    verifying: Option<Address>,
    m: &Value,
    batch: bool,
) -> Result<(), TypedDataError> {
    check_permit2_domain(review, verifying);
    let spender = field_addr(m, "spender")?;
    let deadline = field_uint(m, "deadline")?;
    let permitted: Vec<&Value> = match (batch, m.get("permitted")) {
        (false, Some(p)) => alloc::vec![p],
        (true, Some(Value::Array(items))) => items.iter().collect(),
        _ => return err(String::from("permitted tokens are missing")),
    };
    let total = permitted.len();
    for (i, p) in permitted.into_iter().enumerate() {
        let token = field_addr(p, "token")?;
        let amount = field_uint(p, "amount")?;
        let title = if batch {
            format!(
                "Permit2 one-time transfer {} of {total}",
                i.saturating_add(1)
            )
        } else {
            String::from("Permit2 one-time transfer")
        };
        let mut s = Section::new(&title);
        s.field("Token", address::display(&token));
        s.field(
            "Spender (chooses where it goes)",
            address::display(&spender),
        );
        s.field("Signature valid until", crate::calls::unix_time(&deadline));
        s.field("Nonce", field_uint(m, "nonce")?.to_decimal());
        allowance(
            review,
            &mut s,
            amount,
            &spender,
            &address::checksummed(&token),
            false,
        );
        review.sections.push(s);
    }
    Ok(())
}

/// The shared judgement for every permission a signature grants: unlimited is
/// CRITICAL, a bounded amount is a WARNING, zero revokes.
fn allowance(
    review: &mut Review,
    s: &mut Section,
    amount: U256,
    spender: &Address,
    token: &str,
    permit2: bool,
) {
    let unlimited = if permit2 {
        crate::calls::is_unlimited_permit2_amount(&amount)
    } else {
        amount.is_max() || amount.is_effectively_unlimited()
    };
    if unlimited {
        s.field("Amount", String::from("UNLIMITED"));
        review.find(
            Severity::Critical,
            "UNLIMITED_APPROVAL",
            format!(
                "Signing this lets {} move ALL of {token} out of the signing account, without any \
                 transaction from you.",
                address::checksummed(spender)
            ),
        );
    } else if amount.is_zero() {
        s.field("Amount", String::from("0"));
    } else {
        s.field("Amount (raw integer units)", amount.to_grouped_decimal());
        review.find(
            Severity::Warning,
            "TOKEN_APPROVAL",
            format!(
                "Signing this lets {} move up to this amount of {token} out of the signing account, \
                 without any transaction from you.",
                address::checksummed(spender)
            ),
        );
    }
}

/// A Safe transaction requested as typed data goes through the same review as
/// one read from the Safe's service, so the two paths cannot disagree. Like
/// every typed-data review it is for display: the request's signature is the
/// wallet's to make, and nothing here may become an approval to sign it.
fn safe_tx(
    td: &TypedData,
    verifying: Option<Address>,
    hashes: &TypedDataHashes,
) -> Result<Review, TypedDataError> {
    let m = &td.message;
    let safe = verifying.ok_or_else(|| {
        TypedDataError(String::from(
            "a Safe transaction's domain must name the Safe",
        ))
    })?;
    let chain = td.domain.get("chainId").map(uint_value).transpose()?;
    let version = if chain.is_some() {
        crate::DomainVersion::V1_3Plus
    } else {
        crate::DomainVersion::Legacy
    };
    let operation = field_uint(m, "operation")?
        .to_u64()
        .and_then(|v| u8::try_from(v).ok())
        .ok_or_else(|| TypedDataError(String::from("operation does not fit in uint8")))?;
    let tx = crate::SafeTransaction {
        chain_id: chain.unwrap_or(U256::ZERO),
        safe,
        to: field_addr(m, "to")?,
        value: field_uint(m, "value")?,
        data: bytes_value(
            m.get("data")
                .ok_or_else(|| TypedDataError(String::from("missing data")))?,
        )?,
        operation,
        safe_tx_gas: field_uint(m, "safeTxGas")?,
        base_gas: field_uint(m, "baseGas")?,
        gas_price: field_uint(m, "gasPrice")?,
        gas_token: field_addr(m, "gasToken")?,
        refund_receiver: field_addr(m, "refundReceiver")?,
        nonce: field_uint(m, "nonce")?,
    };
    // The domain declared must be exactly the one the Safe hashes, or the
    // signature would not be over the transaction shown.
    let expected = crate::safe::safe_transaction_hash(&tx, version);
    if hashes.signing_hash != expected {
        return err(String::from(
            "this Safe transaction's typed data does not hash to the Safe transaction hash; its domain is not one a Safe uses",
        ));
    }
    let mut review = crate::review_safe_transaction(&tx, version);
    review.clear_signing_target();
    review.title = String::from("EIP-712 typed data: Safe transaction (owner signature)");
    Ok(review)
}

/// Anything else: shown field by field, and BLIND.
fn generic(review: &mut Review, types: &Types<'_>, td: &TypedData) -> Result<(), TypedDataError> {
    let mut s = Section::new(&format!(
        "{} (a structure this reviewer does not interpret)",
        td.primary_type
    ));
    let mut lines = Vec::new();
    flatten(
        types,
        &Ty::Struct(&td.primary_type),
        &td.message,
        String::new(),
        &mut lines,
        0,
    )?;
    let total = lines.len();
    for (k, v) in lines.into_iter().take(MAX_GENERIC_LINES) {
        s.field(&k, v);
    }
    if total > MAX_GENERIC_LINES {
        s.field(
            "Not shown",
            format!("{} more values", total.saturating_sub(MAX_GENERIC_LINES)),
        );
    }
    review.sections.push(s);
    review.find(
        Severity::Blind,
        "UNKNOWN_TYPED_DATA",
        format!(
            "{} is not a structure this reviewer interprets. Every field is shown above, but what \
             signing it permits is decided by the code at the verifying contract, which these bytes \
             do not contain.",
            escape_untrusted(&td.primary_type)
        ),
    );
    Ok(())
}

fn flatten(
    types: &Types<'_>,
    ty: &Ty<'_>,
    v: &Value,
    path: String,
    out: &mut Vec<(String, String)>,
    depth: usize,
) -> Result<(), TypedDataError> {
    if depth > MAX_DEPTH {
        return err(format!(
            "values are nested more than {MAX_DEPTH} levels deep"
        ));
    }
    match ty {
        Ty::Struct(name) => {
            for f in types.fields(name)? {
                let child = if path.is_empty() {
                    f.name.clone()
                } else {
                    format!("{path}.{}", f.name)
                };
                let fv = v
                    .get(&f.name)
                    .ok_or_else(|| TypedDataError(format!("missing {}", f.name)))?;
                flatten(
                    types,
                    &parse_ty(&f.ty)?,
                    fv,
                    child,
                    out,
                    depth.saturating_add(1),
                )?;
            }
        }
        Ty::Array(inner, _) => {
            if let Value::Array(items) = v {
                for (i, item) in items.iter().enumerate() {
                    flatten(
                        types,
                        inner,
                        item,
                        format!("{path}[{i}]"),
                        out,
                        depth.saturating_add(1),
                    )?;
                }
            }
        }
        leaf => out.push((path, generic_text(leaf, v)?)),
    }
    Ok(())
}

fn generic_text(ty: &Ty<'_>, v: &Value) -> Result<String, TypedDataError> {
    Ok(match ty {
        Ty::Address => address::display(&address_value(v)?),
        Ty::Bool => String::from(if atomic_word(ty, v)?.is_zero() {
            "false"
        } else {
            "true"
        }),
        Ty::Uint(_) => uint_value(v)?.to_grouped_decimal(),
        Ty::Int(bits) => {
            let w = U256(int_word(v, *bits)?);
            if w.0.first().is_some_and(|b| *b & 0x80 != 0) {
                format!("-{}", twos_complement(&w).to_grouped_decimal())
            } else {
                w.to_grouped_decimal()
            }
        }
        Ty::FixedBytes(_) | Ty::Bytes => {
            let b = bytes_value(v)?;
            if b.len() <= 64 {
                hex::encode_prefixed(&b)
            } else {
                format!(
                    "{} bytes, keccak256 {}",
                    b.len(),
                    hex::encode_prefixed(&keccak256(&b))
                )
            }
        }
        Ty::String => match v {
            Value::Str(s) => escape_untrusted(s),
            _ => {
                return err(String::from(
                    "a string field was given something that is not a string",
                ));
            }
        },
        Ty::Struct(_) | Ty::Array(..) => String::from("(structure)"),
    })
}
