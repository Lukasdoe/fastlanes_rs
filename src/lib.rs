#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![feature(avx512_target_feature)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rsum_scalar_u8() {
        let input = vec![0 as u8; 1024];
        let mut out = vec![0 as u8; 1024];
        let base = vec![0 as u8; 1024];
        unsafe {
            generated_rsum_fallback_scalar_rsum(input.as_ptr(), out.as_mut_ptr(), base.as_ptr())
        };
    }

    #[test]
    fn test_rsum_unit64_u8() {
        let input = vec![0 as u8; 1024];
        let mut out = vec![0 as u8; 1024];
        let base = vec![0 as u8; 1024];
        unsafe {
            generated_rsum_fallback_unit64_rsum(input.as_ptr(), out.as_mut_ptr(), base.as_ptr())
        };
    }

    #[test]
    #[cfg(all(
        any(target_arch = "x86", target_arch = "x86_64"),
        target_feature = "sse"
    ))]
    fn test_rsum_sse_u8() {
        let input = vec![0 as u8; 1024];
        let mut out = vec![0 as u8; 1024];
        let base = vec![0 as u8; 1024];
        unsafe { generated_rsum_x86_64_sse_rsum(input.as_ptr(), out.as_mut_ptr(), base.as_ptr()) };
    }

    #[test]
    #[cfg(all(
        any(target_arch = "x86", target_arch = "x86_64"),
        target_feature = "avx2"
    ))]
    fn test_rsum_avx2_u8() {
        let input = vec![0 as u8; 1024];
        let mut out = vec![0 as u8; 1024];
        let base = vec![0 as u8; 1024];
        unsafe { generated_rsum_x86_64_avx2_rsum(input.as_ptr(), out.as_mut_ptr(), base.as_ptr()) };
    }

    #[test]
    #[cfg(all(
        any(target_arch = "x86", target_arch = "x86_64"),
        target_feature = "avx2"
    ))]
    fn test_rsum_avx512bw_u8() {
        let input = vec![0 as u8; 1024];
        let mut out = vec![0 as u8; 1024];
        let base = vec![0 as u8; 1024];
        unsafe {
            generated_rsum_x86_64_avx512bw_rsum(input.as_ptr(), out.as_mut_ptr(), base.as_ptr())
        };
    }
}
