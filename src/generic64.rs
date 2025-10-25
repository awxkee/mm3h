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

#[inline(always)]
pub(crate) fn read_le64(bytes: &[u8]) -> u64 {
    u64::from_ne_bytes([
        bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
    ])
}

#[inline]
fn murmurhash2_64_with_seed_impl(bytes: &[u8], seed: u64) -> u64 {
    const M: u64 = 0xc6a4a7935bd1e995;

    let mut h = seed ^ ((bytes.len() as u64).wrapping_mul(M));

    for chunk in bytes.chunks_exact(32) {
        let mut k0 = read_le64(chunk);
        let mut k1 = read_le64(&chunk[8..16]);
        let mut k2 = read_le64(&chunk[16..24]);
        let mut k3 = read_le64(&chunk[24..32]);

        k0 = k0.wrapping_mul(M);
        k0 ^= k0 >> 47;
        k0 = k0.wrapping_mul(M);

        k1 = k1.wrapping_mul(M);
        k1 ^= k1 >> 47;
        k1 = k1.wrapping_mul(M);

        k2 = k2.wrapping_mul(M);
        k2 ^= k2 >> 47;
        k2 = k2.wrapping_mul(M);

        k3 = k3.wrapping_mul(M);
        k3 ^= k3 >> 47;
        k3 = k3.wrapping_mul(M);

        h ^= k0;
        h = h.wrapping_mul(M);

        h ^= k1;
        h = h.wrapping_mul(M);

        h ^= k2;
        h = h.wrapping_mul(M);

        h ^= k3;
        h = h.wrapping_mul(M);
    }

    let rem = bytes.chunks_exact(32).remainder();

    for chunk in rem.chunks_exact(8) {
        let mut k = read_le64(chunk);

        k = k.wrapping_mul(M);
        k ^= k >> 47;
        k = k.wrapping_mul(M);

        h ^= k;
        h = h.wrapping_mul(M);
    }

    let remainder = rem.chunks_exact(8).remainder();

    let quot = bytes.len() & 7;
    if quot > 0 {
        if quot == 7 {
            h ^= (remainder[6] as u64).wrapping_shl(48);
        }
        if quot >= 6 {
            h ^= (remainder[5] as u64).wrapping_shl(40);
        }
        if quot >= 5 {
            h ^= (remainder[4] as u64).wrapping_shl(32);
        }
        if quot >= 4 {
            h ^= (remainder[3] as u64).wrapping_shl(24);
        }
        if quot >= 3 {
            h ^= (remainder[2] as u64).wrapping_shl(16);
        }
        if quot >= 2 {
            h ^= (remainder[1] as u64).wrapping_shl(8);
        }
        if quot >= 1 {
            h ^= remainder[0] as u64;
        }
        h = h.wrapping_mul(M);
    }

    h ^= h >> 47;
    h = h.wrapping_mul(M);
    h ^= h >> 47;

    h
}

/// Computes a 64-bit Murmur2 hash of the given byte slice with a seed.
///
/// Murmur2 is a non-cryptographic hash function known for good
/// distribution and performance. This variant returns a 64-bit hash.
///
/// # Parameters
///
/// - `bytes`: The input byte slice to hash.
/// - `seed`: A 64-bit seed value to influence the hash output.
///
/// # Returns
///
/// A `u64` representing the 64-bit Murmur2 hash of the input bytes.
pub fn murmurhash2_64_with_seed(bytes: &[u8], seed: u64) -> u64 {
    murmurhash2_64_with_seed_impl(bytes, seed)
}

/// Computes the 64-bit MurmurHash2 of a byte slice.
///
/// This is a fast, non-cryptographic hash function suitable for hash tables
/// and checksums. It processes the input bytes in 8-byte blocks, with a
/// final mix for any remaining bytes.
///
/// # Parameters
///
/// - `bytes`: The input data to hash as a byte slice (`&[u8]`).
///
/// # Returns
///
/// A 64-bit hash (`u64`) of the input data.
pub fn murmurhash2_64(bytes: &[u8]) -> u64 {
    murmurhash2_64_with_seed_impl(bytes, 0)
}

#[cfg(test)]
mod test {
    use super::murmurhash2_64_with_seed;

    #[test]
    fn test_empty_string() {
        assert_eq!(murmurhash2_64_with_seed("".as_bytes(), 0), 0);
    }

    #[test]
    fn test_tail_lengths() {
        assert_eq!(
            murmurhash2_64_with_seed("1".as_bytes(), 0),
            746762829127501960
        );
        assert_eq!(
            murmurhash2_64_with_seed("12".as_bytes(), 0),
            17086341747085514672
        );
        assert_eq!(
            murmurhash2_64_with_seed("123".as_bytes(), 0),
            12856370151437683476
        );
        assert_eq!(
            murmurhash2_64_with_seed("1234".as_bytes(), 0),
            10572085188814244945
        );
    }

    #[test]
    fn test_large_data() {
        assert_eq!(
            murmurhash2_64_with_seed(
                "Rust high performace utilities for YUV format handling and conversion.".as_bytes(),
                0
            ),
            8586007052130510318
        );
        assert_eq!(
            murmurhash2_64_with_seed("432432 gfdsafgsd 32432 fds".as_bytes(), 0),
            12905733955589511660
        );
        assert_eq!(murmurhash2_64_with_seed("MurmurHash2 (32-bit, x86)—The original version; contains a flaw that weakens collision in some cases.[9]MurmurHash2A (32-bit, x86)—A fixed variant using Merkle–Damgård construction. Slightly slower.".as_bytes(), 0), 12740723412636160583);
    }
}
