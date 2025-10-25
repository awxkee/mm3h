/*
 * // Copyright (c) Radzivon Bartoshyk 10/2025. All rights reserved.
 * //
 * // Redistribution and use in source and binary forms, with or without modification,
 * // are permitted provided that the following conditions are met:
 * //
 * // 1.  Redistributions of source code must retain the above copyright notice, this
 * // list of conditions and the following disclaimer.
 * //
 * // 2.  Redistributions in binary form must reproduce the above copyright notice,
 * // this list of conditions and the following disclaimer in the documentation
 * // and/or other materials provided with the distribution.
 * //
 * // 3.  Neither the name of the copyright holder nor the names of its
 * // contributors may be used to endorse or promote products derived from
 * // this software without specific prior written permission.
 * //
 * // THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
 * // AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * // IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * // DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
 * // FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * // DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
 * // SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
 * // CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
 * // OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * // OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */
use crate::generic64::read_le64;

#[inline]
fn fmix64(k: u64) -> u64 {
    const C1: u64 = 0xff51_afd7_ed55_8ccd;
    const C2: u64 = 0xc4ce_b9fe_1a85_ec53;
    const R: u32 = 33;
    let mut tmp = k;
    tmp ^= tmp >> R;
    tmp = tmp.wrapping_mul(C1);
    tmp ^= tmp >> R;
    tmp = tmp.wrapping_mul(C2);
    tmp ^= tmp >> R;
    tmp
}

#[inline]
fn murmurhash3_128_with_seed_impl(bytes: &[u8], seed: u32) -> u128 {
    const C1: u64 = 0x87c3_7b91_1142_53d5;
    const C2: u64 = 0x4cf5_ad43_2745_937f;
    const C3: u64 = 0x52dc_e729;
    const C4: u64 = 0x3849_5ab5;
    const R1: u32 = 27;
    const R3: u32 = 33;
    const M: u64 = 5;
    let mut h1: u64 = seed as u64;
    let mut h2: u64 = seed as u64;

    for chunk in bytes.chunks_exact(16) {
        let k1 = read_le64(chunk);
        let k2 = read_le64(&chunk[8..16]);
        h1 ^= k1.wrapping_mul(C1).rotate_left(31).wrapping_mul(C2);
        h1 = h1
            .rotate_left(R1)
            .wrapping_add(h2)
            .wrapping_mul(M)
            .wrapping_add(C3);
        h2 ^= k2.wrapping_mul(C2).rotate_left(R3).wrapping_mul(C1);
        h2 = h2
            .rotate_left(31)
            .wrapping_add(h1)
            .wrapping_mul(M)
            .wrapping_add(C4);
    }

    let remainder = bytes.chunks_exact(16).remainder();

    if !remainder.is_empty() {
        let len = remainder.len();

        let mut k1 = 0;
        let mut k2 = 0;
        if len >= 15 {
            k2 ^= (remainder[14] as u64).wrapping_shl(48);
        }
        if len >= 14 {
            k2 ^= (remainder[13] as u64).wrapping_shl(40);
        }
        if len >= 13 {
            k2 ^= (remainder[12] as u64).wrapping_shl(32);
        }
        if len >= 12 {
            k2 ^= (remainder[11] as u64).wrapping_shl(24);
        }
        if len >= 11 {
            k2 ^= (remainder[10] as u64).wrapping_shl(16);
        }
        if len >= 10 {
            k2 ^= (remainder[9] as u64).wrapping_shl(8);
        }
        if len >= 9 {
            k2 ^= remainder[8] as u64;
            k2 = k2.wrapping_mul(C2).rotate_left(33).wrapping_mul(C1);
            h2 ^= k2;
        }
        if len >= 8 {
            k1 ^= (remainder[7] as u64).wrapping_shl(56);
        }
        if len >= 7 {
            k1 ^= (remainder[6] as u64).wrapping_shl(48);
        }
        if len >= 6 {
            k1 ^= (remainder[5] as u64).wrapping_shl(40);
        }
        if len >= 5 {
            k1 ^= (remainder[4] as u64).wrapping_shl(32);
        }
        if len >= 4 {
            k1 ^= (remainder[3] as u64).wrapping_shl(24);
        }
        if len >= 3 {
            k1 ^= (remainder[2] as u64).wrapping_shl(16);
        }
        if len >= 2 {
            k1 ^= (remainder[1] as u64).wrapping_shl(8);
        }
        if len >= 1 {
            k1 ^= remainder[0] as u64;
        }
        k1 = k1.wrapping_mul(C1);
        k1 = k1.rotate_left(31);
        k1 = k1.wrapping_mul(C2);
        h1 ^= k1;
    }

    h1 ^= bytes.len() as u64;
    h2 ^= bytes.len() as u64;
    h1 = h1.wrapping_add(h2);
    h2 = h2.wrapping_add(h1);
    h1 = fmix64(h1);
    h2 = fmix64(h2);
    h1 = h1.wrapping_add(h2);
    h2 = h2.wrapping_add(h1);
    ((h2 as u128) << 64) | (h1 as u128)
}

/// Computes the 128-bit MurmurHash3 of a byte slice with a seed.
///
/// This is a fast, non-cryptographic hash function suitable for hash tables,
/// checksums, and general-purpose hashing. It produces a 128-bit hash value.
///
/// # Parameters
///
/// - `bytes`: The input data to hash as a byte slice (`&[u8]`).
/// - `seed`: A 32-bit seed to initialize the hash, allowing different outputs
///   for the same input.
///
/// # Returns
///
/// A 128-bit hash (`u128`) of the input data.
pub fn murmurhash3_128_with_seed(bytes: &[u8], seed: u32) -> u128 {
    murmurhash3_128_with_seed_impl(bytes, seed)
}

/// Computes the 128-bit MurmurHash3 of a byte slice using a default seed of 0.
///
/// This is a fast, non-cryptographic hash function suitable for hash tables,
/// checksums, and general-purpose hashing. It produces a 128-bit hash value.
///
/// # Parameters
///
/// - `bytes`: The input data to hash as a byte slice (`&[u8]`).
///
/// # Returns
///
/// A 128-bit hash (`u128`) of the input data.
pub fn murmurhash3_128(bytes: &[u8]) -> u128 {
    murmurhash3_128_with_seed_impl(bytes, 0)
}

#[cfg(test)]
mod test {
    use super::murmurhash3_128_with_seed;
    use crate::murmurhash3_128;

    #[test]
    fn test_empty_string() {
        assert_eq!(murmurhash3_128_with_seed("".as_bytes(), 0), 0);
    }

    #[test]
    fn test_tail_lengths() {
        assert_eq!(
            murmurhash3_128_with_seed("1".as_bytes(), 0),
            196948598568201132170845188519190297713
        );
        assert_eq!(
            murmurhash3_128_with_seed("12".as_bytes(), 0),
            181809175275681735127740556873700736405
        );
        assert_eq!(
            murmurhash3_128_with_seed("123".as_bytes(), 0),
            88386560642490731012209186590589091690
        );
        assert_eq!(
            murmurhash3_128_with_seed("1234".as_bytes(), 0),
            69278461145226992474103256931126732724
        );
    }

    #[test]
    fn test_large_data() {
        assert_eq!(
            murmurhash3_128(
                "Rust high performace utilities for YUV format handling and conversion.".as_bytes(),
            ),
            54181677952575170241362728818992244552
        );
        assert_eq!(
            murmurhash3_128("432432 gfdsafgsd 32432 fds".as_bytes()),
            71745777201048077136648467711053708943
        );
        assert_eq!(murmurhash3_128("MurmurHash2 (32-bit, x86)—The original version; contains a flaw that weakens collision in some cases.[9]MurmurHash2A (32-bit, x86)—A fixed variant using Merkle–Damgård construction. Slightly slower.".as_bytes()), 285565187209700280420403743997782857281);
    }
}
