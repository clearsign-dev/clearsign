//! Bytewords, minimal encoding (BCR-2020-012).
//!
//! Each byte is one of 256 four-letter English words; the minimal encoding
//! writes only each word's first and last letter, so a byte is two characters.
//! A Bytewords body carries a four-byte CRC-32 of the payload at the end.

use alloc::string::String;
use alloc::vec::Vec;

use crate::Error;
use crate::crc32::crc32;

/// The Bytewords list, in order, from BCR-2020-012. Index is the byte value.
pub const WORDS: [&str; 256] = [
    "able", "acid", "also", "apex", "aqua", "arch", "atom", "aunt", "away", "axis", "back", "bald",
    "barn", "belt", "beta", "bias", "blue", "body", "brag", "brew", "bulb", "buzz", "calm", "cash",
    "cats", "chef", "city", "claw", "code", "cola", "cook", "cost", "crux", "curl", "cusp", "cyan",
    "dark", "data", "days", "deli", "dice", "diet", "door", "down", "draw", "drop", "drum", "dull",
    "duty", "each", "easy", "echo", "edge", "epic", "even", "exam", "exit", "eyes", "fact", "fair",
    "fern", "figs", "film", "fish", "fizz", "flap", "flew", "flux", "foxy", "free", "frog", "fuel",
    "fund", "gala", "game", "gear", "gems", "gift", "girl", "glow", "good", "gray", "grim", "guru",
    "gush", "gyro", "half", "hang", "hard", "hawk", "heat", "help", "high", "hill", "holy", "hope",
    "horn", "huts", "iced", "idea", "idle", "inch", "inky", "into", "iris", "iron", "item", "jade",
    "jazz", "join", "jolt", "jowl", "judo", "jugs", "jump", "junk", "jury", "keep", "keno", "kept",
    "keys", "kick", "kiln", "king", "kite", "kiwi", "knob", "lamb", "lava", "lazy", "leaf", "legs",
    "liar", "limp", "lion", "list", "logo", "loud", "love", "luau", "luck", "lung", "main", "many",
    "math", "maze", "memo", "menu", "meow", "mild", "mint", "miss", "monk", "nail", "navy", "need",
    "news", "next", "noon", "note", "numb", "obey", "oboe", "omit", "onyx", "open", "oval", "owls",
    "paid", "part", "peck", "play", "plus", "poem", "pool", "pose", "puff", "puma", "purr", "quad",
    "quiz", "race", "ramp", "real", "redo", "rich", "road", "rock", "roof", "ruby", "ruin", "runs",
    "rust", "safe", "saga", "scar", "sets", "silk", "skew", "slot", "soap", "solo", "song", "stub",
    "surf", "swan", "taco", "task", "taxi", "tent", "tied", "time", "tiny", "toil", "tomb", "toys",
    "trip", "tuna", "twin", "ugly", "undo", "unit", "urge", "user", "vast", "very", "veto", "vial",
    "vibe", "view", "visa", "void", "vows", "wall", "wand", "warm", "wasp", "wave", "waxy", "webs",
    "what", "when", "whiz", "wolf", "work", "yank", "yawn", "yell", "yoga", "yurt", "zaps", "zero",
    "zest", "zinc", "zone", "zoom",
];

/// Encode `payload` as minimal Bytewords with its CRC-32 appended.
pub fn encode(payload: &[u8]) -> String {
    let checksum = crc32(payload);
    let mut out = String::with_capacity(payload.len().saturating_add(4).saturating_mul(2));
    for byte in payload.iter().chain(checksum.to_be_bytes().iter()) {
        push_minimal(&mut out, *byte);
    }
    out
}

/// Decode minimal Bytewords, checking and removing the trailing CRC-32.
pub fn decode(s: &str) -> Result<Vec<u8>, Error> {
    let bytes = s.as_bytes();
    if bytes.len() % 2 != 0 {
        return Err(Error::Bytewords("odd number of characters"));
    }
    let mut out = Vec::with_capacity(bytes.len() / 2);
    let mut chars = bytes.chunks_exact(2);
    for pair in &mut chars {
        let (first, last) = match pair {
            [a, b] => (*a, *b),
            _ => return Err(Error::Bytewords("truncated pair")),
        };
        out.push(byte_for(first, last).ok_or(Error::Bytewords("not a Byteword"))?);
    }
    // A body is the payload plus a four-byte checksum, so it cannot be shorter.
    let split = out
        .len()
        .checked_sub(4)
        .ok_or(Error::Bytewords("shorter than its checksum"))?;
    let (payload, checksum) = out.split_at(split);
    let found: [u8; 4] = checksum
        .try_into()
        .map_err(|_| Error::Bytewords("truncated checksum"))?;
    if crc32(payload).to_be_bytes() != found {
        return Err(Error::Bytewords("checksum does not match"));
    }
    out.truncate(split);
    Ok(out)
}

fn push_minimal(out: &mut String, byte: u8) {
    if let Some(word) = WORDS.get(usize::from(byte)) {
        let mut chars = word.chars();
        if let Some(first) = chars.next() {
            out.push(first);
        }
        if let Some(last) = word.chars().next_back() {
            out.push(last);
        }
    }
}

/// The byte whose word starts with `first` and ends with `last`.
fn byte_for(first: u8, last: u8) -> Option<u8> {
    let first = first.to_ascii_lowercase();
    let last = last.to_ascii_lowercase();
    WORDS
        .iter()
        .position(|w| {
            let b = w.as_bytes();
            match (b.first(), b.last()) {
                (Some(f), Some(l)) => *f == first && *l == last,
                _ => false,
            }
        })
        .and_then(|i| u8::try_from(i).ok())
}
