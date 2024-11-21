fn main() {
    println!("Building protobufs..");
    tonic_build::configure()
        .build_server(true) // Set to false if you only need client
        .out_dir("src/generated")
        .compile(&["proto/galactus.proto"], &["proto"])
        .unwrap();
}