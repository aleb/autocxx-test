use autocxx_engine::BuilderError;

fn main() -> Result<(), BuilderError> {
    println!("cargo:rerun-if-changed=build.rs");

    let paths = ["include"];
    let build = autocxx_build::Builder::new("src/main.rs", paths)
        .extra_clang_args(&["-std=c++20"])
        .build()?;
    build.compile("my-precious-lib");
    println!("cargo:rerun-if-changed=src/main.rs");

    Ok(())
}
