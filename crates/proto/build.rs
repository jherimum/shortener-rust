use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    tonic_build::compile_protos("protos/key-generator-service/kgs.proto")
        .unwrap();
    Ok(())
}
