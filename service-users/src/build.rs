fn main() {
    println!("cargo:warning=Compiling protos...");
    tonic_build::configure()
        .build_client(false)
        .out_dir("src/")
        .compile(&["../../proto/main.proto"], &["../../proto"])
        .expect("failed to compile protos");
}
