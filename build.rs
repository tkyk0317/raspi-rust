extern crate cc;

// アセンブラやCファイルをコンパイル
fn main() {
    cc::Build::new()
        .file("src/boot.s")
        .file("src/vector.s")
        .flag("-mcpu=cortex-a53")
        .compile("asm");
}
