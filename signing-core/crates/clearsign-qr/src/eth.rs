//! EIP-4527: `eth-sign-request` in, `eth-signature` out.
//!
//! This is the format MetaMask and Keystone already speak, which is the whole
//! point: the signer plugs into wallets nobody here controls.

use alloc::string::String;
use alloc::vec::Vec;

use crate::Error;
use crate::cbor::{self, Kind};

/// CBOR tag 37: a UUID byte string.
const TAG_UUID: u64 = 37;
/// CBOR tag 304: `crypto-keypath`.
const TAG_KEYPATH: u64 = 304;

/// Map keys of `eth-sign-request`.
const KEY_REQUEST_ID: u64 = 1;
const KEY_SIGN_DATA: u64 = 2;
const KEY_DATA_TYPE: u64 = 3;
const KEY_CHAIN_ID: u64 = 4;
const KEY_DERIVATION_PATH: u64 = 5;
const KEY_ADDRESS: u64 = 6;
const KEY_ORIGIN: u64 = 7;

/// Map keys of `crypto-keypath`.
const KEY_COMPONENTS: u64 = 1;
const KEY_SOURCE_FINGERPRINT: u64 = 2;

/// Largest payload accepted inside a signing request.
pub const MAX_SIGN_DATA: usize = 32 * 1024;
/// Longest derivation path accepted.
pub const MAX_PATH_COMPONENTS: usize = 16;

/// What the bytes in a request are.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    /// Legacy transaction, RLP encoded.
    Transaction,
    /// EIP-712 typed data, as JSON bytes.
    TypedData,
    /// Raw bytes for `personal_sign` (EIP-191).
    RawBytes,
    /// EIP-2718 typed transaction, e.g. EIP-1559.
    TypedTransaction,
}

impl DataType {
    fn from_u64(v: u64) -> Result<DataType, Error> {
        match v {
            1 => Ok(DataType::Transaction),
            2 => Ok(DataType::TypedData),
            3 => Ok(DataType::RawBytes),
            4 => Ok(DataType::TypedTransaction),
            _ => Err(Error::Eip4527("unknown sign-data type")),
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            DataType::Transaction => "legacy transaction",
            DataType::TypedData => "EIP-712 typed data",
            DataType::RawBytes => "raw message bytes",
            DataType::TypedTransaction => "typed transaction (EIP-2718)",
        }
    }
}

/// One step of a BIP-32 path.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PathComponent {
    pub index: u32,
    pub hardened: bool,
}

/// A decoded `eth-sign-request`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignRequest {
    pub request_id: Option<Vec<u8>>,
    pub sign_data: Vec<u8>,
    pub data_type: DataType,
    pub chain_id: Option<u64>,
    pub path: Vec<PathComponent>,
    pub source_fingerprint: Option<u32>,
    pub address: Option<[u8; 20]>,
    pub origin: Option<String>,
}

impl SignRequest {
    /// The derivation path as text, e.g. `m/44'/60'/0'/0/0`.
    pub fn path_string(&self) -> String {
        let mut out = String::from("m");
        for c in &self.path {
            out.push('/');
            let mut digits = [0u8; 10];
            let mut n = c.index;
            let mut len = 0usize;
            loop {
                let digit = (n % 10) as u8;
                if let Some(slot) = digits.get_mut(len) {
                    *slot = b'0'.saturating_add(digit);
                }
                len = len.saturating_add(1);
                n /= 10;
                if n == 0 {
                    break;
                }
            }
            for i in (0..len).rev() {
                out.push(char::from(digits.get(i).copied().unwrap_or(b'0')));
            }
            if c.hardened {
                out.push('\'');
            }
        }
        out
    }
}

/// Decode the CBOR body of an `eth-sign-request` UR.
pub fn decode_sign_request(message: &[u8]) -> Result<SignRequest, Error> {
    let mut r = cbor::Reader::new(message);
    let pairs = r.map()?;
    if pairs > 16 {
        return Err(Error::Eip4527("too many fields"));
    }

    let mut request_id = None;
    let mut sign_data: Option<Vec<u8>> = None;
    let mut data_type = None;
    let mut chain_id = None;
    let mut path = Vec::new();
    let mut source_fingerprint = None;
    let mut address = None;
    let mut origin = None;
    let mut seen: Vec<u64> = Vec::new();

    for _ in 0..pairs {
        let key = r.uint()?;
        if seen.contains(&key) {
            return Err(Error::Eip4527("duplicate field"));
        }
        seen.push(key);
        match key {
            KEY_REQUEST_ID => {
                if let Kind::Tag(TAG_UUID) = r.kind()? {
                    r.tag(TAG_UUID)?;
                }
                request_id = Some(r.bytes()?.to_vec());
            }
            KEY_SIGN_DATA => {
                let data = r.bytes()?;
                if data.is_empty() || data.len() > MAX_SIGN_DATA {
                    return Err(Error::Eip4527("sign-data is empty or too large"));
                }
                sign_data = Some(data.to_vec());
            }
            KEY_DATA_TYPE => {
                // The CDDL tags this #3.401; wallets in the field send a bare
                // integer. Accept either, and nothing else.
                if let Kind::Tag(tag) = r.kind()? {
                    if tag != 401 {
                        return Err(Error::Eip4527("unexpected tag on data-type"));
                    }
                    r.tag(401)?;
                }
                data_type = Some(DataType::from_u64(r.uint()?)?);
            }
            KEY_CHAIN_ID => chain_id = Some(r.uint()?),
            KEY_DERIVATION_PATH => {
                let (components, fingerprint) = decode_keypath(&mut r)?;
                path = components;
                source_fingerprint = fingerprint;
            }
            KEY_ADDRESS => {
                let bytes = r.bytes()?;
                address = Some(
                    <[u8; 20]>::try_from(bytes)
                        .map_err(|_| Error::Eip4527("address is not 20 bytes"))?,
                );
            }
            KEY_ORIGIN => origin = Some(String::from(r.text()?)),
            _ => r.skip()?,
        }
    }
    r.expect_end()?;

    let sign_data = sign_data.ok_or(Error::Eip4527("no sign-data"))?;
    let data_type = data_type.ok_or(Error::Eip4527("no data-type"))?;
    if path.is_empty() {
        return Err(Error::Eip4527("no derivation path"));
    }
    Ok(SignRequest {
        request_id,
        sign_data,
        data_type,
        chain_id,
        path,
        source_fingerprint,
        address,
        origin,
    })
}

fn decode_keypath(r: &mut cbor::Reader<'_>) -> Result<(Vec<PathComponent>, Option<u32>), Error> {
    if let Kind::Tag(TAG_KEYPATH) = r.kind()? {
        r.tag(TAG_KEYPATH)?;
    }
    let pairs = r.map()?;
    let mut components = Vec::new();
    let mut fingerprint = None;
    for _ in 0..pairs {
        match r.uint()? {
            KEY_COMPONENTS => {
                let len = r.array()?;
                if len % 2 != 0 {
                    return Err(Error::Eip4527("keypath components are not pairs"));
                }
                let steps =
                    usize::try_from(len / 2).map_err(|_| Error::Eip4527("path too long"))?;
                if steps > MAX_PATH_COMPONENTS {
                    return Err(Error::Eip4527("derivation path is too long"));
                }
                for _ in 0..steps {
                    // Wildcards and ranges are arrays; a signer must know the
                    // exact key it is using, so only a plain index is accepted.
                    let index = u32::try_from(r.uint()?)
                        .map_err(|_| Error::Eip4527("path index out of range"))?;
                    let hardened = r.boolean()?;
                    components.push(PathComponent { index, hardened });
                }
            }
            KEY_SOURCE_FINGERPRINT => {
                fingerprint = Some(
                    u32::try_from(r.uint()?)
                        .map_err(|_| Error::Eip4527("fingerprint out of range"))?,
                );
            }
            _ => r.skip()?,
        }
    }
    Ok((components, fingerprint))
}

/// Encode an `eth-signature` CBOR body: the reply a wallet expects.
pub fn encode_signature(
    request_id: Option<&[u8]>,
    signature: &[u8; 65],
    origin: Option<&str>,
) -> Vec<u8> {
    let mut out = Vec::new();
    let fields = 1u64
        .saturating_add(u64::from(request_id.is_some()))
        .saturating_add(u64::from(origin.is_some()));
    cbor::write_map(&mut out, fields);
    if let Some(id) = request_id {
        cbor::write_uint(&mut out, 1);
        cbor::write_tag(&mut out, TAG_UUID);
        cbor::write_bytes(&mut out, id);
    }
    cbor::write_uint(&mut out, 2);
    cbor::write_bytes(&mut out, signature);
    if let Some(text) = origin {
        cbor::write_uint(&mut out, 3);
        cbor::write_text(&mut out, text);
    }
    out
}
