//! The multipart UR fountain scheme (BCR-2024-001).
//!
//! Sender and receiver must agree, without communicating, on which fragments
//! were mixed into each part. They do that by deriving everything from the
//! message checksum and the part's sequence number through a specified PRNG.
//! Every step here follows the reference implementation in the spec, and each
//! has the spec's own test vector in `tests/`.

use alloc::vec;
use alloc::vec::Vec;

use sha2::{Digest, Sha256};

/// Xoshiro256**, seeded from a SHA-256 digest, exactly as MUR specifies.
pub struct Xoshiro256 {
    state: [u64; 4],
}

impl Xoshiro256 {
    pub fn from_seed_bytes(seed: &[u8]) -> Self {
        let digest = Sha256::digest(seed);
        let mut state = [0u64; 4];
        for (i, slot) in state.iter_mut().enumerate() {
            let mut v = 0u64;
            for n in 0..8 {
                let byte = digest
                    .get(i.saturating_mul(8).saturating_add(n))
                    .copied()
                    .unwrap_or(0);
                v = (v << 8) | u64::from(byte);
            }
            *slot = v;
        }
        Xoshiro256 { state }
    }

    pub fn next_u64(&mut self) -> u64 {
        let [s0, s1, s2, s3] = self.state;
        let result = s1.wrapping_mul(5).rotate_left(7).wrapping_mul(9);
        let t = s1 << 17;
        let mut n2 = s2 ^ s0;
        let mut n3 = s3 ^ s1;
        let n1 = s1 ^ n2;
        let n0 = s0 ^ n3;
        n2 ^= t;
        n3 = n3.rotate_left(45);
        self.state = [n0, n1, n2, n3];
        result
    }

    /// Uniform in [0, 1), as the spec defines it: `next() / (u64::MAX + 1)`.
    pub fn next_double(&mut self) -> f64 {
        self.next_u64() as f64 / (u64::MAX as f64 + 1.0)
    }

    /// Uniform integer in `low..high` (exclusive), by the spec's formula.
    pub fn next_int(&mut self, low: usize, high: usize) -> usize {
        let count = high.saturating_sub(low) as f64;
        let scaled = self.next_double() * count;
        low.saturating_add(scaled as usize)
    }
}

/// Walker-Vose alias sampler, used to pick a part's degree with a bias to 1.
pub struct RandomSampler {
    probs: Vec<f64>,
    aliases: Vec<usize>,
}

impl RandomSampler {
    /// `probs` must be non-negative with a positive sum.
    pub fn new(probs: &[f64]) -> Option<Self> {
        let n = probs.len();
        if n == 0 || probs.iter().any(|p| *p < 0.0) {
            return None;
        }
        let sum: f64 = probs.iter().sum();
        if !sum.is_finite() || sum <= 0.0 {
            return None;
        }
        let mut p: Vec<f64> = probs.iter().map(|x| x * n as f64 / sum).collect();
        let mut small: Vec<usize> = Vec::new();
        let mut large: Vec<usize> = Vec::new();
        // The spec reverses Schwarz's index order; matching it matters, because
        // the resulting tables must be identical on both sides of the channel.
        for i in (0..n).rev() {
            match p.get(i) {
                Some(v) if *v < 1.0 => small.push(i),
                Some(_) => large.push(i),
                None => return None,
            }
        }
        let mut out_probs = vec![0.0f64; n];
        let mut out_aliases = vec![0usize; n];
        while let (Some(a), Some(g)) = (small.last().copied(), large.last().copied()) {
            small.pop();
            large.pop();
            let pa = *p.get(a)?;
            *out_probs.get_mut(a)? = pa;
            *out_aliases.get_mut(a)? = g;
            let pg = p.get_mut(g)?;
            *pg += pa - 1.0;
            if *pg < 1.0 {
                small.push(g);
            } else {
                large.push(g);
            }
        }
        for i in large.into_iter().chain(small) {
            *out_probs.get_mut(i)? = 1.0;
        }
        Some(RandomSampler {
            probs: out_probs,
            aliases: out_aliases,
        })
    }

    pub fn next(&self, rng: &mut Xoshiro256) -> usize {
        let r1 = rng.next_double();
        let r2 = rng.next_double();
        let n = self.probs.len();
        let i = (n as f64 * r1) as usize;
        match (self.probs.get(i), self.aliases.get(i)) {
            (Some(p), Some(alias)) => {
                if r2 < *p {
                    i
                } else {
                    *alias
                }
            }
            _ => 0,
        }
    }
}

/// Choose how many fragments a part mixes: a harmonic series, biased to 1.
pub fn choose_degree(seq_len: usize, rng: &mut Xoshiro256) -> Option<usize> {
    let probs: Vec<f64> = (1..=seq_len).map(|i| 1.0 / i as f64).collect();
    let sampler = RandomSampler::new(&probs)?;
    Some(sampler.next(rng).saturating_add(1))
}

/// Fisher-Yates, stopping once `count` items have been drawn.
pub fn shuffled(items: &[usize], rng: &mut Xoshiro256, count: usize) -> Vec<usize> {
    let mut remaining: Vec<usize> = items.to_vec();
    let mut result: Vec<usize> = Vec::with_capacity(count);
    while result.len() < count && !remaining.is_empty() {
        let index = rng.next_int(0, remaining.len());
        if index >= remaining.len() {
            break;
        }
        result.push(remaining.remove(index));
    }
    result
}

/// Which fragments part `seq_num` carries. Parts 1..=seq_len are the plain
/// fragments in order; beyond that, the mix is derived from the checksum.
pub fn choose_fragments(seq_num: u32, seq_len: usize, checksum: u32) -> Vec<usize> {
    if seq_len == 0 {
        return Vec::new();
    }
    if u64::from(seq_num) <= seq_len as u64 {
        let index = usize::try_from(seq_num).unwrap_or(0).saturating_sub(1);
        return vec![index];
    }
    let mut seed = [0u8; 8];
    if let Some(slot) = seed.get_mut(0..4) {
        slot.copy_from_slice(&seq_num.to_be_bytes());
    }
    if let Some(slot) = seed.get_mut(4..8) {
        slot.copy_from_slice(&checksum.to_be_bytes());
    }
    let mut rng = Xoshiro256::from_seed_bytes(&seed);
    let degree = choose_degree(seq_len, &mut rng).unwrap_or(1);
    let indexes: Vec<usize> = (0..seq_len).collect();
    let mut chosen = shuffled(&indexes, &mut rng, degree);
    chosen.sort_unstable();
    chosen.dedup();
    chosen
}

/// The fragment length the reference encoder picks: the fewest fragments whose
/// length still fits `max_fragment_len` (BCR-2024-001, "Determining Fragment
/// Length"). The receiver does not need this, but an encoder and its tests do.
pub fn find_nominal_fragment_length(
    message_len: usize,
    min_fragment_len: usize,
    max_fragment_len: usize,
) -> Option<usize> {
    if message_len == 0 || min_fragment_len == 0 || max_fragment_len < min_fragment_len {
        return None;
    }
    let max_fragment_count = message_len.checked_div(min_fragment_len)?;
    let mut fragment_len = None;
    for fragment_count in 1..=max_fragment_count.max(1) {
        let len = message_len.div_ceil(fragment_count);
        fragment_len = Some(len);
        if len <= max_fragment_len {
            break;
        }
    }
    fragment_len
}
