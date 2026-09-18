//! Recovery phrase to Ethereum accounts: BIP-39 seed, BIP-32 path m/44'/60'/0'/0/i.

use alloc::string::String;

use bip32::{ChildNumber, XPrv};
use k256::ecdsa::{SigningKey, VerifyingKey};
use zeroize::Zeroizing;

use crate::KeyError;
use crate::approval::Approval;
use crate::signature::{self, Signature};

pub struct Wallet {
    seed: Zeroizing<[u8; 64]>,
}

impl Wallet {
    /// Parse a 12- to 24-word English BIP-39 phrase and an optional passphrase.
    ///
    /// Words are matched case-insensitively and any whitespace separates them.
    /// The passphrase must be printable ASCII: BIP-39 requires Unicode NFKD
    /// normalisation, and restricting input avoids wallets disagreeing about it.
    pub fn from_mnemonic(phrase: &str, passphrase: &str) -> Result<Wallet, KeyError> {
        if !passphrase.bytes().all(|b| (0x20..=0x7e).contains(&b)) {
            return Err(KeyError::PassphraseNotPrintableAscii);
        }
        if !phrase.is_ascii() {
            return Err(KeyError::InvalidMnemonic);
        }
        let mut normalized = Zeroizing::new(String::with_capacity(phrase.len()));
        for (i, word) in phrase.split_whitespace().enumerate() {
            if i != 0 {
                normalized.push(' ');
            }
            for c in word.chars() {
                normalized.push(c.to_ascii_lowercase());
            }
        }
        let m = bip39::Mnemonic::parse_in_normalized(bip39::Language::English, &normalized)
            .map_err(|_| KeyError::InvalidMnemonic)?;
        Ok(Wallet {
            seed: Zeroizing::new(m.to_seed_normalized(passphrase)),
        })
    }

    /// The fingerprint of the master key: the first four bytes of the RIPEMD-160
    /// of the SHA-256 of the master public key, as BIP-32 defines it.
    ///
    /// A signing request can name the wallet it expects. Comparing that here is
    /// how a device answers "is this request even for me" before it derives
    /// anything, rather than producing a valid signature from the wrong seed and
    /// leaving the wallet to discover it.
    pub fn master_fingerprint(&self) -> Result<[u8; 4], KeyError> {
        let key = XPrv::new(self.seed.as_slice()).map_err(|_| KeyError::Derivation)?;
        Ok(key.public_key().fingerprint())
    }

    /// The account at m/44'/60'/0'/0/`index`, the path MetaMask and Foundry use for
    /// successive accounts. Ledger Live instead varies the third component
    /// (m/44'/60'/`index`'/0/0), so accounts created there appear under different indexes.
    pub fn ethereum_account(&self, index: u32) -> Result<Account, KeyError> {
        let mut key = XPrv::new(self.seed.as_slice()).map_err(|_| KeyError::Derivation)?;
        for (n, hardened) in [
            (44, true),
            (60, true),
            (0, true),
            (0, false),
            (index, false),
        ] {
            let child = ChildNumber::new(n, hardened).map_err(|_| KeyError::Derivation)?;
            key = key.derive_child(child).map_err(|_| KeyError::Derivation)?;
        }
        let signing_key = key.private_key().clone();
        let address = address_of(signing_key.verifying_key());
        Ok(Account {
            signing_key,
            address,
            index,
        })
    }
}

pub struct Account {
    signing_key: SigningKey,
    address: [u8; 20],
    index: u32,
}

impl Account {
    pub fn address(&self) -> [u8; 20] {
        self.address
    }

    pub fn index(&self) -> u32 {
        self.index
    }

    /// Sign an approved review. There is no way to sign anything else.
    pub fn sign(&self, approval: &Approval<'_>) -> Result<Signature, KeyError> {
        signature::sign(&self.signing_key, approval)
    }
}

pub(crate) fn address_of(key: &VerifyingKey) -> [u8; 20] {
    let point = key.to_encoded_point(false);
    let bytes = point.as_bytes();
    // Uncompressed SEC1 is 0x04 || X (32) || Y (32); the address hashes X || Y.
    let hash = clearsign::keccak::keccak256(bytes.get(1..).unwrap_or(&[]));
    let mut out = [0u8; 20];
    if let Some(tail) = hash.get(12..) {
        out.copy_from_slice(tail);
    }
    out
}
