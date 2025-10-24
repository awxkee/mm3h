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
use crate::generic::scramble;
#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn _mm_rotate_left15(v: __m128i) -> __m128i {
    let left = _mm_slli_epi32::<15>(v);
    let right = _mm_srli_epi32::<17>(v);
    _mm_or_si128(left, right)
}

#[target_feature(enable = "sse2")]
pub(crate) unsafe fn sse_murmurhash3_32(bytes: &[u8], seed: u32) -> u32 {
    const C1: u32 = 0xcc9e2d51u32;
    const C2: u32 = 0x1b873593u32;

    let read_size = 4;
    let len = bytes.len() as u32;
    let block_count = len / read_size;

    let mut h1 = seed;

    let c1 = _mm_set1_epi32(C1 as i32);
    let c2 = _mm_set1_epi32(C2 as i32);

    for chunk in bytes.chunks_exact(64) {
        unsafe {
            let mut k1 = _mm_loadu_si128(chunk.as_ptr().cast());
            let mut k2 = _mm_loadu_si128(chunk.get_unchecked(16..).as_ptr().cast());
            let mut k3 = _mm_loadu_si128(chunk.get_unchecked(32..).as_ptr().cast());
            let mut k4 = _mm_loadu_si128(chunk.get_unchecked(48..).as_ptr().cast());

            k1 = _mm_mullo_epi32(k1, c1);
            k2 = _mm_mullo_epi32(k2, c1);
            k3 = _mm_mullo_epi32(k3, c1);
            k4 = _mm_mullo_epi32(k4, c1);

            k1 = _mm_rotate_left15(k1);
            k2 = _mm_rotate_left15(k2);
            k3 = _mm_rotate_left15(k3);
            k4 = _mm_rotate_left15(k4);

            k1 = _mm_mullo_epi32(k1, c2);
            k2 = _mm_mullo_epi32(k2, c2);
            k3 = _mm_mullo_epi32(k3, c2);
            k4 = _mm_mullo_epi32(k4, c2);

            h1 ^= _mm_extract_epi32::<0>(k1) as u32;
            h1 = h1.rotate_left(13);
            h1 = h1.wrapping_mul(5);
            h1 = h1.wrapping_add(0xe6546b64);

            h1 ^= _mm_extract_epi32::<1>(k1) as u32;
            h1 = h1.rotate_left(13);
            h1 = h1.wrapping_mul(5);
            h1 = h1.wrapping_add(0xe6546b64);

            h1 ^= _mm_extract_epi32::<2>(k1) as u32;
            h1 = h1.rotate_left(13);
            h1 = h1.wrapping_mul(5);
            h1 = h1.wrapping_add(0xe6546b64);

            h1 ^= _mm_extract_epi32::<3>(k1) as u32;
            h1 = h1.rotate_left(13);
            h1 = h1.wrapping_mul(5);
            h1 = h1.wrapping_add(0xe6546b64);

            h1 ^= _mm_extract_epi32::<0>(k2) as u32;
            h1 = h1.rotate_left(13);
            h1 = h1.wrapping_mul(5);
            h1 = h1.wrapping_add(0xe6546b64);

            h1 ^= _mm_extract_epi32::<1>(k2) as u32;
            h1 = h1.rotate_left(13);
            h1 = h1.wrapping_mul(5);
            h1 = h1.wrapping_add(0xe6546b64);

            h1 ^= _mm_extract_epi32::<2>(k2) as u32;
            h1 = h1.rotate_left(13);
            h1 = h1.wrapping_mul(5);
            h1 = h1.wrapping_add(0xe6546b64);

            h1 ^= _mm_extract_epi32::<3>(k2) as u32;
            h1 = h1.rotate_left(13);
            h1 = h1.wrapping_mul(5);
            h1 = h1.wrapping_add(0xe6546b64);

            h1 ^= _mm_extract_epi32::<0>(k3) as u32;
            h1 = h1.rotate_left(13);
            h1 = h1.wrapping_mul(5);
            h1 = h1.wrapping_add(0xe6546b64);

            h1 ^= _mm_extract_epi32::<1>(k3) as u32;
            h1 = h1.rotate_left(13);
            h1 = h1.wrapping_mul(5);
            h1 = h1.wrapping_add(0xe6546b64);

            h1 ^= _mm_extract_epi32::<2>(k3) as u32;
            h1 = h1.rotate_left(13);
            h1 = h1.wrapping_mul(5);
            h1 = h1.wrapping_add(0xe6546b64);

            h1 ^= _mm_extract_epi32::<3>(k3) as u32;
            h1 = h1.rotate_left(13);
            h1 = h1.wrapping_mul(5);
            h1 = h1.wrapping_add(0xe6546b64);

            h1 ^= _mm_extract_epi32::<0>(k4) as u32;
            h1 = h1.rotate_left(13);
            h1 = h1.wrapping_mul(5);
            h1 = h1.wrapping_add(0xe6546b64);

            h1 ^= _mm_extract_epi32::<1>(k4) as u32;
            h1 = h1.rotate_left(13);
            h1 = h1.wrapping_mul(5);
            h1 = h1.wrapping_add(0xe6546b64);

            h1 ^= _mm_extract_epi32::<2>(k4) as u32;
            h1 = h1.rotate_left(13);
            h1 = h1.wrapping_mul(5);
            h1 = h1.wrapping_add(0xe6546b64);

            h1 ^= _mm_extract_epi32::<3>(k4) as u32;
            h1 = h1.rotate_left(13);
            h1 = h1.wrapping_mul(5);
            h1 = h1.wrapping_add(0xe6546b64);
        }
    }

    let rem = bytes.chunks_exact(64).remainder();

    for chunk in rem.chunks_exact(32) {
        unsafe {
            let mut k1 = _mm_loadu_si128(chunk.as_ptr().cast());
            let mut k2 = _mm_loadu_si128(chunk.get_unchecked(16..).as_ptr().cast());

            k1 = _mm_mullo_epi32(k1, c1);
            k2 = _mm_mullo_epi32(k2, c1);

            k1 = _mm_rotate_left15(k1);
            k2 = _mm_rotate_left15(k2);

            k1 = _mm_mullo_epi32(k1, c2);
            k2 = _mm_mullo_epi32(k2, c2);

            h1 ^= _mm_extract_epi32::<0>(k1) as u32;
            h1 = h1.rotate_left(13);
            h1 = h1.wrapping_mul(5);
            h1 = h1.wrapping_add(0xe6546b64);

            h1 ^= _mm_extract_epi32::<1>(k1) as u32;
            h1 = h1.rotate_left(13);
            h1 = h1.wrapping_mul(5);
            h1 = h1.wrapping_add(0xe6546b64);

            h1 ^= _mm_extract_epi32::<2>(k1) as u32;
            h1 = h1.rotate_left(13);
            h1 = h1.wrapping_mul(5);
            h1 = h1.wrapping_add(0xe6546b64);

            h1 ^= _mm_extract_epi32::<3>(k1) as u32;
            h1 = h1.rotate_left(13);
            h1 = h1.wrapping_mul(5);
            h1 = h1.wrapping_add(0xe6546b64);

            h1 ^= _mm_extract_epi32::<0>(k2) as u32;
            h1 = h1.rotate_left(13);
            h1 = h1.wrapping_mul(5);
            h1 = h1.wrapping_add(0xe6546b64);

            h1 ^= _mm_extract_epi32::<1>(k2) as u32;
            h1 = h1.rotate_left(13);
            h1 = h1.wrapping_mul(5);
            h1 = h1.wrapping_add(0xe6546b64);

            h1 ^= _mm_extract_epi32::<2>(k2) as u32;
            h1 = h1.rotate_left(13);
            h1 = h1.wrapping_mul(5);
            h1 = h1.wrapping_add(0xe6546b64);

            h1 ^= _mm_extract_epi32::<3>(k2) as u32;
            h1 = h1.rotate_left(13);
            h1 = h1.wrapping_mul(5);
            h1 = h1.wrapping_add(0xe6546b64);
        }
    }

    let rem = rem.chunks_exact(32).remainder();

    for chunk in rem.chunks_exact(4) {
        let mut k1 = u32::from_ne_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]).to_le();

        k1 = k1.wrapping_mul(C1);
        k1 = k1.rotate_left(15);
        k1 = k1.wrapping_mul(C2);

        h1 ^= k1;
        h1 = h1.rotate_left(13);
        h1 = h1.wrapping_mul(5);
        h1 = h1.wrapping_add(0xe6546b64)
    }

    let mut k1 = 0u32;

    if len & 3 == 3 {
        k1 ^= (bytes[(block_count * read_size) as usize + 2] as u32) << 16;
    }
    if len & 3 >= 2 {
        k1 ^= (bytes[(block_count * read_size) as usize + 1] as u32) << 8;
    }
    if len & 3 >= 1 {
        k1 ^= bytes[(block_count * read_size) as usize] as u32;
        k1 = k1.wrapping_mul(C1);
        k1 = k1.rotate_left(15);
        k1 = k1.wrapping_mul(C2);
        h1 ^= k1;
    }

    h1 ^= bytes.len() as u32;
    h1 = scramble(h1);

    h1
}

#[cfg(test)]
mod test {
    use super::sse_murmurhash3_32;

    #[test]
    fn test_empty_string() {
        if std::arch::is_x86_feature_detected!("sse2") {
            unsafe {
                assert_eq!(sse_murmurhash3_32("".as_bytes(), 0), 0);
            }
        }
    }

    #[test]
    fn test_tail_lengths() {
        if std::arch::is_x86_feature_detected!("sse2") {
            unsafe {
                assert_eq!(sse_murmurhash3_32("1".as_bytes(), 0), 2484513939);
                assert_eq!(sse_murmurhash3_32("12".as_bytes(), 0), 4191350549);
                assert_eq!(sse_murmurhash3_32("123".as_bytes(), 0), 2662625771);
                assert_eq!(sse_murmurhash3_32("1234".as_bytes(), 0), 1914461635);
            }
        }
    }

    #[test]
    fn test_large_data() {
        if std::arch::is_x86_feature_detected!("sse2") {
            unsafe {
                assert_eq!(
                    sse_murmurhash3_32("The quick brown fox jumps over the lazy dog".as_bytes(), 0),
                    776992547
                );
                assert_eq!(
                    sse_murmurhash3_32(
                        "Rust high performace utilities for YUV format handling and conversion."
                            .as_bytes(),
                        0
                    ),
                    937425919
                );
                assert_eq!(
                    sse_murmurhash3_32("432432 gfdsafgsd 32432 fds".as_bytes(), 0),
                    948823384
                );
                assert_eq!(sse_murmurhash3_32("MurmurHash2 (32-bit, x86)—The original version; contains a flaw that weakens collision in some cases.[9]
MurmurHash2A (32-bit, x86)—A fixed variant using Merkle–Damgård construction. Slightly slower.
CMurmurHash2A (32-bit, x86)—MurmurHash2A, but works incrementally.
MurmurHashNeutral2 (32-bit, x86)—Slower, but endian- and alignment-neutral.
MurmurHashAligned2 (32-bit, x86)—Slower, but does aligned reads (safer on some platforms).
MurmurHash64A (64-bit, x64)—The original 64-bit version. Optimized for 64-bit arithmetic.
MurmurHash64B (64-bit, x86)—A 64-bit version optimized for 32-bit platforms. It is not a true 64-bit hash due to insufficient mixing of the stripes.[10]
".as_bytes(), 0), 1959355408);
            }
        }
    }
}
