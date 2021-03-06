#![feature(cfg_target_feature)]
#![cfg_attr(feature = "strict", deny(warnings))]
#![cfg_attr(feature = "cargo-clippy",
            allow(option_unwrap_used, print_stdout, use_debug))]

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[macro_use]
extern crate coresimd;

#[test]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
fn x86_all() {
    println!("sse: {:?}", cfg_feature_enabled!("sse"));
    println!("sse2: {:?}", cfg_feature_enabled!("sse2"));
    println!("sse3: {:?}", cfg_feature_enabled!("sse3"));
    println!("ssse3: {:?}", cfg_feature_enabled!("ssse3"));
    println!("sse4.1: {:?}", cfg_feature_enabled!("sse4.1"));
    println!("sse4.2: {:?}", cfg_feature_enabled!("sse4.2"));
    println!("sse4a: {:?}", cfg_feature_enabled!("sse4a"));
    println!("avx: {:?}", cfg_feature_enabled!("avx"));
    println!("avx2: {:?}", cfg_feature_enabled!("avx2"));
    println!("avx512f {:?}", cfg_feature_enabled!("avx512f"));
    println!("avx512cd {:?}", cfg_feature_enabled!("avx512cd"));
    println!("avx512er {:?}", cfg_feature_enabled!("avx512er"));
    println!("avx512pf {:?}", cfg_feature_enabled!("avx512pf"));
    println!("avx512bw {:?}", cfg_feature_enabled!("avx512bw"));
    println!("avx512dq {:?}", cfg_feature_enabled!("avx512dq"));
    println!("avx512vl {:?}", cfg_feature_enabled!("avx512vl"));
    println!("avx512_ifma {:?}", cfg_feature_enabled!("avx512ifma"));
    println!("avx512_vbmi {:?}", cfg_feature_enabled!("avx512vbmi"));
    println!(
        "avx512_vpopcntdq {:?}",
        cfg_feature_enabled!("avx512vpopcntdq")
    );
    println!("fma: {:?}", cfg_feature_enabled!("fma"));
    println!("abm: {:?}", cfg_feature_enabled!("abm"));
    println!("bmi: {:?}", cfg_feature_enabled!("bmi"));
    println!("bmi2: {:?}", cfg_feature_enabled!("bmi2"));
    println!("tbm: {:?}", cfg_feature_enabled!("tbm"));
    println!("popcnt: {:?}", cfg_feature_enabled!("popcnt"));
    println!("lzcnt: {:?}", cfg_feature_enabled!("lzcnt"));
    println!("fxsr: {:?}", cfg_feature_enabled!("fxsr"));
    println!("xsave: {:?}", cfg_feature_enabled!("xsave"));
    println!("xsaveopt: {:?}", cfg_feature_enabled!("xsaveopt"));
    println!("xsaves: {:?}", cfg_feature_enabled!("xsaves"));
    println!("xsavec: {:?}", cfg_feature_enabled!("xsavec"));
}
