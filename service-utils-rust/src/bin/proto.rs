fn main() {
    println!("Compiling protos...");
    tonic_build::configure()
        .out_dir("src/")
        .compile(&["../proto/main.proto"], &["../proto"])
        .expect("Failed to compile protos");
}
