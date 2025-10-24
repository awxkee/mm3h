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
use std::arch::aarch64::*;

#[inline(always)]
unsafe fn vrotate_left15(v: uint32x4_t) -> uint32x4_t {
    unsafe { vsliq_n_u32::<15>(vshrq_n_u32::<17>(v), v) }
}

pub(crate) fn neon_murmurhash3_32(bytes: &[u8], seed: u32) -> u32 {
    const C1: u32 = 0xcc9e2d51u32;
    const C2: u32 = 0x1b873593u32;

    let read_size = 4;
    let len = bytes.len() as u32;
    let block_count = len / read_size;

    let mut h1 = seed;

    for chunk in bytes.chunks_exact(64) {
        unsafe {
            let mut k1 = vld1q_u32(chunk.as_ptr() as *const u32);
            let mut k2 = vld1q_u32(chunk.get_unchecked(16..).as_ptr() as *const u32);
            let mut k3 = vld1q_u32(chunk.get_unchecked(32..).as_ptr() as *const u32);
            let mut k4 = vld1q_u32(chunk.get_unchecked(48..).as_ptr() as *const u32);

            k1 = vmulq_n_u32(k1, C1);
            k2 = vmulq_n_u32(k2, C1);
            k3 = vmulq_n_u32(k3, C1);
            k4 = vmulq_n_u32(k4, C1);

            k1 = vrotate_left15(k1);
            k2 = vrotate_left15(k2);
            k3 = vrotate_left15(k3);
            k4 = vrotate_left15(k4);

            k1 = vmulq_n_u32(k1, C2);
            k2 = vmulq_n_u32(k2, C2);
            k3 = vmulq_n_u32(k3, C2);
            k4 = vmulq_n_u32(k4, C2);

            h1 ^= vgetq_lane_u32::<0>(k1);
            h1 = h1.rotate_left(13);
            h1 = h1.wrapping_mul(5);
            h1 = h1.wrapping_add(0xe6546b64);

            h1 ^= vgetq_lane_u32::<1>(k1);
            h1 = h1.rotate_left(13);
            h1 = h1.wrapping_mul(5);
            h1 = h1.wrapping_add(0xe6546b64);

            h1 ^= vgetq_lane_u32::<2>(k1);
            h1 = h1.rotate_left(13);
            h1 = h1.wrapping_mul(5);
            h1 = h1.wrapping_add(0xe6546b64);

            h1 ^= vgetq_lane_u32::<3>(k1);
            h1 = h1.rotate_left(13);
            h1 = h1.wrapping_mul(5);
            h1 = h1.wrapping_add(0xe6546b64);

            h1 ^= vgetq_lane_u32::<0>(k2);
            h1 = h1.rotate_left(13);
            h1 = h1.wrapping_mul(5);
            h1 = h1.wrapping_add(0xe6546b64);

            h1 ^= vgetq_lane_u32::<1>(k2);
            h1 = h1.rotate_left(13);
            h1 = h1.wrapping_mul(5);
            h1 = h1.wrapping_add(0xe6546b64);

            h1 ^= vgetq_lane_u32::<2>(k2);
            h1 = h1.rotate_left(13);
            h1 = h1.wrapping_mul(5);
            h1 = h1.wrapping_add(0xe6546b64);

            h1 ^= vgetq_lane_u32::<3>(k2);
            h1 = h1.rotate_left(13);
            h1 = h1.wrapping_mul(5);
            h1 = h1.wrapping_add(0xe6546b64);

            h1 ^= vgetq_lane_u32::<0>(k3);
            h1 = h1.rotate_left(13);
            h1 = h1.wrapping_mul(5);
            h1 = h1.wrapping_add(0xe6546b64);

            h1 ^= vgetq_lane_u32::<1>(k3);
            h1 = h1.rotate_left(13);
            h1 = h1.wrapping_mul(5);
            h1 = h1.wrapping_add(0xe6546b64);

            h1 ^= vgetq_lane_u32::<2>(k3);
            h1 = h1.rotate_left(13);
            h1 = h1.wrapping_mul(5);
            h1 = h1.wrapping_add(0xe6546b64);

            h1 ^= vgetq_lane_u32::<3>(k3);
            h1 = h1.rotate_left(13);
            h1 = h1.wrapping_mul(5);
            h1 = h1.wrapping_add(0xe6546b64);

            h1 ^= vgetq_lane_u32::<0>(k4);
            h1 = h1.rotate_left(13);
            h1 = h1.wrapping_mul(5);
            h1 = h1.wrapping_add(0xe6546b64);

            h1 ^= vgetq_lane_u32::<1>(k4);
            h1 = h1.rotate_left(13);
            h1 = h1.wrapping_mul(5);
            h1 = h1.wrapping_add(0xe6546b64);

            h1 ^= vgetq_lane_u32::<2>(k4);
            h1 = h1.rotate_left(13);
            h1 = h1.wrapping_mul(5);
            h1 = h1.wrapping_add(0xe6546b64);

            h1 ^= vgetq_lane_u32::<3>(k4);
            h1 = h1.rotate_left(13);
            h1 = h1.wrapping_mul(5);
            h1 = h1.wrapping_add(0xe6546b64);
        }
    }

    let rem = bytes.chunks_exact(64).remainder();

    for chunk in rem.chunks_exact(32) {
        unsafe {
            let mut k1 = vld1q_u32(chunk.as_ptr() as *const u32);
            let mut k2 = vld1q_u32(chunk.get_unchecked(16..).as_ptr() as *const u32);

            k1 = vmulq_n_u32(k1, C1);
            k2 = vmulq_n_u32(k2, C1);

            k1 = vrotate_left15(k1);
            k2 = vrotate_left15(k2);

            k1 = vmulq_n_u32(k1, C2);
            k2 = vmulq_n_u32(k2, C2);

            h1 ^= vgetq_lane_u32::<0>(k1);
            h1 = h1.rotate_left(13);
            h1 = h1.wrapping_mul(5);
            h1 = h1.wrapping_add(0xe6546b64);

            h1 ^= vgetq_lane_u32::<1>(k1);
            h1 = h1.rotate_left(13);
            h1 = h1.wrapping_mul(5);
            h1 = h1.wrapping_add(0xe6546b64);

            h1 ^= vgetq_lane_u32::<2>(k1);
            h1 = h1.rotate_left(13);
            h1 = h1.wrapping_mul(5);
            h1 = h1.wrapping_add(0xe6546b64);

            h1 ^= vgetq_lane_u32::<3>(k1);
            h1 = h1.rotate_left(13);
            h1 = h1.wrapping_mul(5);
            h1 = h1.wrapping_add(0xe6546b64);

            h1 ^= vgetq_lane_u32::<0>(k2);
            h1 = h1.rotate_left(13);
            h1 = h1.wrapping_mul(5);
            h1 = h1.wrapping_add(0xe6546b64);

            h1 ^= vgetq_lane_u32::<1>(k2);
            h1 = h1.rotate_left(13);
            h1 = h1.wrapping_mul(5);
            h1 = h1.wrapping_add(0xe6546b64);

            h1 ^= vgetq_lane_u32::<2>(k2);
            h1 = h1.rotate_left(13);
            h1 = h1.wrapping_mul(5);
            h1 = h1.wrapping_add(0xe6546b64);

            h1 ^= vgetq_lane_u32::<3>(k2);
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
    use super::neon_murmurhash3_32;

    #[test]
    fn test_empty_string() {
        assert_eq!(neon_murmurhash3_32("".as_bytes(), 0), 0);
    }

    #[test]
    fn test_tail_lengths() {
        assert_eq!(neon_murmurhash3_32("1".as_bytes(), 0), 2484513939);
        assert_eq!(neon_murmurhash3_32("12".as_bytes(), 0), 4191350549);
        assert_eq!(neon_murmurhash3_32("123".as_bytes(), 0), 2662625771);
        assert_eq!(neon_murmurhash3_32("1234".as_bytes(), 0), 1914461635);
    }

    #[test]
    fn test_large_data() {
        assert_eq!(
            neon_murmurhash3_32("The quick brown fox jumps over the lazy dog".as_bytes(), 0),
            776992547
        );
        assert_eq!(
            neon_murmurhash3_32(
                "Rust high performace utilities for YUV format handling and conversion.".as_bytes(),
                0
            ),
            937425919
        );
        assert_eq!(
            neon_murmurhash3_32("432432 gfdsafgsd 32432 fds".as_bytes(), 0),
            948823384
        );
        assert_eq!(neon_murmurhash3_32("MurmurHash2 (32-bit, x86)—The original version; contains a flaw that weakens collision in some cases.[9]
MurmurHash2A (32-bit, x86)—A fixed variant using Merkle–Damgård construction. Slightly slower.
CMurmurHash2A (32-bit, x86)—MurmurHash2A, but works incrementally.
MurmurHashNeutral2 (32-bit, x86)—Slower, but endian- and alignment-neutral.
MurmurHashAligned2 (32-bit, x86)—Slower, but does aligned reads (safer on some platforms).
MurmurHash64A (64-bit, x64)—The original 64-bit version. Optimized for 64-bit arithmetic.
MurmurHash64B (64-bit, x86)—A 64-bit version optimized for 32-bit platforms. It is not a true 64-bit hash due to insufficient mixing of the stripes.[10]
".as_bytes(), 0), 1959355408);
    }
}
