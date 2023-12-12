use cmake::Config;

fn main() {
    let cfg = Config::new("third_party/protobuf")
        .define("protobuf_BUILD_TESTS", "OFF")
        .define("protobuf_WITH_ZLIB", "OFF")
        .define("protobuf_BUILD_CONFORMANCE", "ON")
        .define("CMAKE_CXX_STANDARD", "17")
        .define("CMAKE_BUILD_TYPE", "Release")
        .define("BUILD_SHARED_LIBS", "OFF")
        .build();

    let out_dir = std::env::var_os("OUT_DIR").expect("OUT_DIR");
    let out_dir = out_dir.to_str().expect("msg");
    let mut b = autocxx_build::Builder::new(
        "src/ffi.rs",
        &["include", &format!("{}/include", cfg.display())],
    )
    .build()
    .unwrap();

    b.cpp(true)
        .std("c++17")
        .file("include/runner.h")
        .file("include/runner.cc")
        .file("include/gen.h")
        .file("include/gen.cc")
        .include(format!("{}/include", cfg.display()))
        .include("third_party/protobuf")
        .static_flag(true)
        .out_dir(&out_dir)
        .compile("conformance");

    // Rebuild
    println!("cargo:rerun-if-changed=src/ffi.rs");
    println!("cargo:rerun-if-changed=include/gen.h");
    println!("cargo:rerun-if-changed=include/gen.cc");
    println!("cargo:rerun-if-changed=include/runner.h");
    println!("cargo:rerun-if-changed=include/runner.cc");

    // Linking
    for f in cfg.join("lib").read_dir().unwrap().into_iter() {
        let f = f.unwrap().file_name().to_str().unwrap().to_string();
        if f.starts_with("libabsl") && f.ends_with(".a") {
            println!(
                "cargo:rustc-link-lib=static={}",
                f.strip_prefix("lib").unwrap().strip_suffix(".a").unwrap()
            );
        }
    }

    println!("cargo:rustc-link-search=native={}/build/lib", cfg.display());
    println!("cargo:rustc-link-lib=static=conformance_common");
    println!("cargo:rustc-link-search=native={}/lib", cfg.display());
    println!("cargo:rustc-link-lib=static=protobuf");
    println!("cargo:rustc-link-lib=static=jsoncpp");
    println!("cargo:rustc-link-lib=static=utf8_range");
    println!("cargo:rustc-link-lib=static=utf8_validity");
    println!("cargo:rustc-link-search=native={}", &out_dir.to_string());
    println!("cargo:rustc-link-lib=conformance");

    // Proto
    protobuf_codegen::Codegen::new()
        .protoc()
        .include("third_party/protobuf/conformance")
        .include("third_party/protobuf/src/")
        .input("third_party/protobuf/conformance/conformance.proto")
        .input("third_party/protobuf/src/google/protobuf/test_messages_proto3.proto")
        .input("third_party/protobuf/src/google/protobuf/test_messages_proto2.proto")
        .cargo_out_dir("conformance")
        .run_from_script();
}
