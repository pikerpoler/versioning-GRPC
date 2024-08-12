use std::{env, fs};
use std::path::{Path, PathBuf};

const PROTO_DIR: &str = "src/vector_service";
const THIRD_PARTY_DIR: &str = "src"; // Update as needed

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let versions = [
        "V1",
        "V2",
    ];
    for version in versions {
        // directory the main .proto file resides in
        let file_name = format!("{version}.proto");
        let proto_path = Path::new(PROTO_DIR).join(file_name);
        println!("path : {proto_path:?}");

        let include_dirs = [Path::new(PROTO_DIR),
            Path::new(THIRD_PARTY_DIR),
        ];

        println!("include: {include_dirs:?}");

        let original_out_dir = PathBuf::from(env::var("OUT_DIR")?);
        let out_dir = "src"; // Use the correct out_dir

        tonic_build::configure()
            .protoc_arg("--experimental_allow_proto3_optional")
            .out_dir(out_dir)
            .file_descriptor_set_path(
                original_out_dir.join(format!("vector_service.{version}.bin")),
            )
            .compile(&[proto_path], &include_dirs)?;
    }
    // remove unneeded google.api.rs file post-compile
    let google_api_path: &Path = "./src/engine/grpc/google.api.rs".as_ref();
    if fs::metadata(google_api_path).is_ok() {
        fs::remove_file(google_api_path).unwrap()
    }
    Ok(())
}
