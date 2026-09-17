//! Key handling tests. Expected values come from independent implementations:
//! - `bip39_vectors.json`: the official Trezor python-mnemonic vectors (passphrase "TREZOR")
//! - addresses and signatures: Foundry `cast` 1.7.1 (`cast wallet address`, `cast wallet sign --no-hash`,
//!   `cast wallet new-mnemonic --entropy`, `cast mktx`)
//! - dice entropy: `shasum -a 256` and Python `hashlib`

#![allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::panic
)]

use clearsign::{DomainVersion, SafeTransaction, U256, hex};
use clearsign_keys::*;

const TEST_MNEMONIC: &str = "test test test test test test test test test test test junk";

fn addr_hex(a: [u8; 20]) -> String {
    clearsign::address::checksummed(&a)
}

#[test]
fn bip39_official_vectors_all_24() {
    let text = include_str!("bip39_vectors.json");
    // The "english" array holds [entropy, mnemonic, seed, xprv] quadruples of plain strings.
    let english = &text[text.find("\"english\"").unwrap() + "\"english\"".len()..];
    let strings: Vec<&str> = english.split('"').skip(1).step_by(2).collect();
    let mut count = 0;
    // The file continues with other languages; English quadruples all end in an xprv.
    for quad in strings
        .chunks_exact(4)
        .take_while(|q| q[3].starts_with("xprv"))
    {
        let (entropy_hex, mnemonic, seed_hex) = (quad[0], quad[1], quad[2]);
        let m = bip39::Mnemonic::parse_in_normalized(bip39::Language::English, mnemonic).unwrap();
        assert_eq!(
            hex::encode(&m.to_entropy()),
            entropy_hex,
            "entropy for {mnemonic}"
        );
        assert_eq!(
            hex::encode(&m.to_seed_normalized("TREZOR")),
            seed_hex,
            "seed for {mnemonic}"
        );
        assert!(Wallet::from_mnemonic(mnemonic, "TREZOR").is_ok());
        count += 1;
    }
    assert_eq!(count, 24);
}

#[test]
fn derivation_matches_cast_wallet_address() {
    let w = Wallet::from_mnemonic(TEST_MNEMONIC, "").unwrap();
    assert_eq!(
        addr_hex(w.ethereum_account(0).unwrap().address()),
        "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266"
    );
    assert_eq!(
        addr_hex(w.ethereum_account(1).unwrap().address()),
        "0x70997970C51812dc3A010C7d01b50e0d17dc79C8"
    );
}

#[test]
fn phrase_input_is_normalised_but_checksum_enforced() {
    let messy = "  TEST test\ttest test test test test test test test test   JUNK ";
    let a = Wallet::from_mnemonic(messy, "")
        .unwrap()
        .ethereum_account(0)
        .unwrap()
        .address();
    assert_eq!(addr_hex(a), "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266");
    // Same words, wrong final word: checksum must fail.
    assert_eq!(
        Wallet::from_mnemonic(
            "test test test test test test test test test test test test",
            ""
        )
        .err(),
        Some(KeyError::InvalidMnemonic)
    );
    assert_eq!(
        Wallet::from_mnemonic(TEST_MNEMONIC, "pässword").err(),
        Some(KeyError::PassphraseNotPrintableAscii)
    );
}

#[test]
fn dice_only_mnemonic_matches_sha256_and_cast() {
    let rolls = "314153265352313323246264332321356222413116333331516522631434453236121646622626233262263422534211166";
    let phrase = mnemonic_from_dice(&DiceRolls::parse(rolls).unwrap()).unwrap();
    // sha256 = 6410aee9...d50f ; `cast wallet new-mnemonic --entropy 0x6410...`
    assert_eq!(
        phrase.as_str(),
        "goat lyrics ripple sunset gasp pair nothing advance shadow follow cave chief fade useful park manual whisper brick face medal cannon fragile feature unknown"
    );
    let w = Wallet::from_mnemonic(&phrase, "").unwrap();
    assert_eq!(
        addr_hex(w.ethereum_account(0).unwrap().address()),
        "0x4F5FCD95A07547CAbdB33C3862D980D219D7aD8B"
    );
}

#[test]
fn dice_rules() {
    assert_eq!(
        DiceRolls::parse("1234560").err(),
        Some(KeyError::InvalidDiceRoll)
    );
    let spaced = DiceRolls::parse("12 34\n56").unwrap();
    assert_eq!(spaced.count(), 6);
    assert_eq!(
        mnemonic_from_dice(&DiceRolls::parse(&"1".repeat(98)).unwrap()).err(),
        Some(KeyError::NotEnoughDiceRolls {
            required: 99,
            found: 98
        })
    );
}

#[test]
fn mixed_entropy_matches_reference_and_rejects_stuck_hardware() {
    let hw: [u8; 32] = core::array::from_fn(|i| (i + 1) as u8);
    let rolls = DiceRolls::parse("31415326535231332324626433232135622241311633333151").unwrap();
    // Python hashlib reference entropy c07ecf77...52a1, then `cast wallet new-mnemonic --entropy`.
    assert_eq!(
        mnemonic_from_mixed_entropy(&hw, &rolls).unwrap().as_str(),
        "scatter wait tape churn hurdle kitten accuse remember coconut icon electric robust define dune speed heavy ticket sunny thank dumb release expose enhance coast"
    );
    assert_eq!(
        mnemonic_from_mixed_entropy(&[0u8; 32], &rolls).err(),
        Some(KeyError::DegenerateHardwareEntropy)
    );
    assert_eq!(
        mnemonic_from_mixed_entropy(&hw, &DiceRolls::parse(&"2".repeat(49)).unwrap()).err(),
        Some(KeyError::NotEnoughDiceRolls {
            required: 50,
            found: 49
        })
    );
}

/// A deterministic generator that looks random passes the sanity check, which is
/// exactly why dice are mandatory. This test documents the limitation.
#[test]
fn sanity_check_cannot_detect_a_deterministic_generator() {
    let mut x: u64 = 0x2545_f491_4f6c_dd1d;
    let predictable: [u8; 32] = core::array::from_fn(|_| {
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        x as u8
    });
    let rolls = DiceRolls::parse(&"6".repeat(50)).unwrap();
    assert!(mnemonic_from_mixed_entropy(&predictable, &rolls).is_ok());
}

fn safe_vector_a() -> SafeTransaction {
    SafeTransaction {
        chain_id: U256::from_u64(1),
        safe: hex::decode("0x1Db92e2EeBC8E0c075a02BeA49a2935BcD2dFCF4").unwrap().try_into().unwrap(),
        to: hex::decode("0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48").unwrap().try_into().unwrap(),
        value: U256::ZERO,
        data: hex::decode("0xa9059cbb00000000000000000000000070997970c51812dc3a010c7d01b50e0d17dc79c8000000000000000000000000000000000000000000000000000000003b9aca00").unwrap(),
        operation: 0,
        safe_tx_gas: U256::ZERO,
        base_gas: U256::ZERO,
        gas_price: U256::ZERO,
        gas_token: [0; 20],
        refund_receiver: [0; 20],
        nonce: U256::from_u64(42),
    }
}

#[test]
fn safe_owner_signature_matches_cast() {
    let review = clearsign::review_safe_transaction(&safe_vector_a(), DomainVersion::V1_3Plus);
    let approval = approve(&review, &[]).unwrap();
    let account = Wallet::from_mnemonic(TEST_MNEMONIC, "")
        .unwrap()
        .ethereum_account(0)
        .unwrap();
    let sig = account.sign(&approval).unwrap();
    assert_eq!(
        hex::encode_prefixed(&sig.to_rsv65()),
        "0x4110d55809304240fef94c308ec8af6b3ab2e34cbba895d45cdbeca4154d6f4a5391d10edbcd9a5bea632e64cb5f69291fd2777e2ec9589d89f8709d180c1b7f1b"
    );
    assert_eq!(sig.v().unwrap(), 27);
}

#[test]
fn eip1559_signature_matches_cast_mktx() {
    let bytes = hex::decode("0x02f86e0107843b9aca008506fc23ac00830186a094a0b86991c6218b36c1d19d4a2e9eb0ce3606eb4880b844095ea7b30000000000000000000000003fc91a3afd70395cd496c647d5a6cc9d4b2b7fadffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc0").unwrap();
    let review = clearsign::review_transaction_bytes(&bytes).unwrap();
    // Unlimited approval is CRITICAL: signing must be refused until acknowledged.
    assert_eq!(
        approve(&review, &[]).err(),
        Some(KeyError::UnacknowledgedFinding("UNLIMITED_APPROVAL"))
    );
    let approval = approve(&review, &["UNLIMITED_APPROVAL"]).unwrap();
    let account = Wallet::from_mnemonic(TEST_MNEMONIC, "")
        .unwrap()
        .ethereum_account(0)
        .unwrap();
    let sig = account.sign(&approval).unwrap();
    // From the signed transaction `cast mktx` produced for the same fields: y_parity 1, r, s.
    assert_eq!(
        hex::encode(&sig.r),
        "c55747f9c503dc35085332f2e78d211f698bac3af570be76b7046beb83df4463"
    );
    assert_eq!(
        hex::encode(&sig.s),
        "335c7b57e2c2732a7873b001aeab6cb907e090edd4981bc64938c6902ac63e51"
    );
    assert_eq!(sig.v().unwrap(), 1);
}

#[test]
fn legacy_eip155_v_value() {
    let bytes = hex::decode("0xed058504a817c8008252089470997970c51812dc3a010c7d01b50e0d17dc79c8880de0b6b3a76400008081898080").unwrap();
    let review = clearsign::review_transaction_bytes(&bytes).unwrap();
    let approval = approve(&review, &[]).unwrap();
    let account = Wallet::from_mnemonic(TEST_MNEMONIC, "")
        .unwrap()
        .ethereum_account(0)
        .unwrap();
    let sig = account.sign(&approval).unwrap();
    // `cast wallet sign --no-hash` over keccak of these bytes ends in 0x1b: recovery id 0.
    assert_eq!(
        hex::encode_prefixed(&sig.to_rsv65()),
        "0xe00bb049b1b8dab8bf49ffff9bb9762d0f6d03da589d5e033d7b3693c06cb7cf7c2765a69334726d479adf3723c352a4b2341d33f17414117ebc0f927beb123d1b"
    );
    assert_eq!(sig.v().unwrap(), 137 * 2 + 35);
}

#[test]
fn approval_requires_exact_acknowledgements() {
    // Bybit pattern: DELEGATECALL is CRITICAL.
    let mut tx = safe_vector_a();
    tx.operation = 1;
    let review = clearsign::review_safe_transaction(&tx, DomainVersion::V1_3Plus);
    assert_eq!(
        approve(&review, &[]).err(),
        Some(KeyError::UnacknowledgedFinding("SAFE_DELEGATECALL"))
    );
    // Extra, unrelated acknowledgements are refused: no blanket lists.
    assert_eq!(
        approve(&review, &["SAFE_DELEGATECALL", "UNLIMITED_APPROVAL"]).err(),
        Some(KeyError::UnexpectedAcknowledgement)
    );
    // Duplicates are refused too.
    assert_eq!(
        approve(&review, &["SAFE_DELEGATECALL", "SAFE_DELEGATECALL"]).err(),
        Some(KeyError::UnexpectedAcknowledgement)
    );
    // Acknowledging something on a clean review is refused.
    let clean = clearsign::review_safe_transaction(&safe_vector_a(), DomainVersion::V1_3Plus);
    assert_eq!(
        approve(&clean, &["SAFE_DELEGATECALL"]).err(),
        Some(KeyError::UnexpectedAcknowledgement)
    );
    assert!(approve(&review, &["SAFE_DELEGATECALL"]).is_ok());
}

#[test]
fn blind_findings_also_require_acknowledgement() {
    let mut tx = safe_vector_a();
    tx.data = vec![0xde, 0xad, 0xbe, 0xef];
    let review = clearsign::review_safe_transaction(&tx, DomainVersion::V1_3Plus);
    assert_eq!(
        approve(&review, &[]).err(),
        Some(KeyError::UnacknowledgedFinding("UNKNOWN_SELECTOR"))
    );
}

#[test]
fn display_only_reviews_cannot_be_signed() {
    let bytes = hex::decode("0x02f86e0107843b9aca008506fc23ac00830186a094a0b86991c6218b36c1d19d4a2e9eb0ce3606eb4880b844095ea7b30000000000000000000000003fc91a3afd70395cd496c647d5a6cc9d4b2b7fadffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc0").unwrap();
    let parsed = clearsign::parse_unsigned_transaction(&bytes).unwrap();
    let display_only = clearsign::review_evm_transaction(&parsed);
    assert_eq!(
        approve(&display_only, &["UNLIMITED_APPROVAL"]).err(),
        Some(KeyError::NotSignable)
    );
}

#[test]
fn signatures_are_low_s_and_deterministic() {
    let account = Wallet::from_mnemonic(TEST_MNEMONIC, "")
        .unwrap()
        .ethereum_account(3)
        .unwrap();
    let half_n =
        hex::decode("7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a0").unwrap();
    for nonce in 0..200u64 {
        let mut tx = safe_vector_a();
        tx.nonce = U256::from_u64(nonce);
        let review = clearsign::review_safe_transaction(&tx, DomainVersion::V1_3Plus);
        let approval = approve(&review, &[]).unwrap();
        let a = account.sign(&approval).unwrap();
        let b = account.sign(&approval).unwrap();
        assert_eq!(a, b, "RFC 6979 signatures must be deterministic");
        assert!(
            a.s.as_slice() <= half_n.as_slice(),
            "high-S signature at nonce {nonce}"
        );
    }
}
