use fastlanes::{
    generated_rsum_fallback_scalar_rsum2, generated_untranspose_fallback_scalar_untranspose_i1,
};

mod data;
use data::DELTA_ARR;

use crate::data::{BASE_ARR, TRANSPOSED, UNTRANSPONSED};

#[test]
fn test_untranspose_u32() {
    let mut gen_untransposed = vec![0; 1024];
    unsafe {
        generated_untranspose_fallback_scalar_untranspose_i1(
            TRANSPOSED.as_ptr(),
            gen_untransposed.as_mut_ptr(),
        )
    };
    assert_eq!(gen_untransposed, UNTRANSPONSED);
}

#[test]
fn test_rsum_decode_scalar_u32() {
    let mut gen_rsum_decoded = vec![0; 1024];
    unsafe {
        generated_rsum_fallback_scalar_rsum2(
            DELTA_ARR.as_ptr(),
            gen_rsum_decoded.as_mut_ptr(),
            BASE_ARR.as_ptr(),
        )
    };
    assert_eq!(gen_rsum_decoded, TRANSPOSED);
}
