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

use std::hash::Hasher;

pub struct Murmur3Hasher {
    seed: u32,
    bytes: Vec<u8>,
}

impl Hasher for Murmur3Hasher {
    fn finish(&self) -> u64 {
        #[cfg(all(target_arch = "aarch64", feature = "neon"))]
        {
            use crate::neon::neon_murmurhash3_32;
            neon_murmurhash3_32(&self.bytes, self.seed) as u64
        }
        #[cfg(not(all(target_arch = "aarch64", feature = "neon")))]
        {
            use std::sync::OnceLock;
            type HashFn = unsafe fn(&[u8], u32) -> u32;
            static EXECUTOR: OnceLock<HashFn> = OnceLock::new();

            let func = EXECUTOR.get_or_init(|| {
                #[cfg(all(target_arch = "x86_64", feature = "avx"))]
                {
                    if std::arch::is_x86_feature_detected!("avx2") {
                        use crate::avx::avx_murmurhash3_32;
                        return avx_murmurhash3_32;
                    }
                }
                #[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), feature = "sse"))]
                {
                    if std::arch::is_x86_feature_detected!("sse2") {
                        use crate::sse::sse_murmurhash3_32;
                        return sse_murmurhash3_32;
                    }
                }

                crate::generic::murmurhash3_32
            });
            unsafe { func(&self.bytes, self.seed) as u64 }
        }
    }

    fn write(&mut self, bytes: &[u8]) {
        let mut copy = bytes.to_vec();
        self.bytes.append(&mut copy);
    }
}

impl Default for Murmur3Hasher {
    fn default() -> Self {
        Self {
            seed: 0,
            bytes: vec![0; 0],
        }
    }
}

impl Murmur3Hasher {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn new_with_seed(seed: u32) -> Self {
        Self {
            seed,
            bytes: vec![0; 0],
        }
    }
}

#[cfg(test)]
mod test {
    use super::Murmur3Hasher;
    use std::hash::Hasher;

    #[test]
    fn use_in_hashmap() {
        let mut hasher = Murmur3Hasher::default();
        hasher.write_i16(0x1234);
        hasher.write_i16(0x1234);
        hasher.write_i16(0x1234);
        let op = hasher.finish();
        assert_eq!(op, 0x3c09ef02u64);
    }
}
