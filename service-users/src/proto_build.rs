fn main() {
    println!("Compiling protos...");
    tonic_build::configure()
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .out_dir("./src/")
        .compile(&["../proto/main.proto"], &["../proto"])
        .expect("Failed to compile protos");
}
