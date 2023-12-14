fn main() {
    println!("Compiling protos...");
    tonic_build::configure()
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .out_dir("../service-users/src/")
        .compile(&["./main.proto"], &["./"])
        .expect("Failed to compile users protos");

    tonic_build::configure()
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .out_dir("../service-notes/src/")
        .compile(&["./main.proto"], &["./"])
        .expect("Failed to compile notes protos");

    tonic_build::configure()
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .out_dir("../service-utils/src/")
        .compile(&["./main.proto"], &["./"])
        .expect("Failed to compile utils protos");

    tonic_build::configure()
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .out_dir("../service-oauth/src/")
        .compile(&["./main.proto"], &["./"])
        .expect("Failed to compile utils protos");
}
