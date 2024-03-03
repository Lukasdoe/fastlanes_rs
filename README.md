# FastLanes Rust Crate

This crate wraps the supplementary material for the [FastLanes Compression Format Paper](https://ir.cwi.nl/pub/32992/32992.pdf) (https://github.com/cwida/FastLanes).

## Valid Exported Functions

 * generated_rsum_fallback_scalar_rsum{,1,2,3}
 * generated_rsum_fallback_unit64_rsum{,1,2,3}
 * generated_rsum_x86_64_avx2_rsum{,1,2,3}
 * generated_rsum_x86_64_sse_rsum{,1,2,3}
 * generated_rsum_x86_64_avx512bw_rsum{,1,2,3}
 * generated_untranspose_fallback_scalar_untranspose_i{,1,2,3}
 * generated_untranspose_fallback_scalar_untranspose_o{,1,2,3}

## Known Problems

 * Requires clang++ compiler available on the system
 * Requires nightly build for the avx512bw target feature flag
 * Most exported signatures are missing a definition, only use functions listed in [here](#valid-exported-functions)
