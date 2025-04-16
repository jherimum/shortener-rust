use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    tonic_build::compile_protos("protos/key-generator-service/kgs.proto")?;
    tonic_build::compile_protos("protos/link-service/links.proto")?;
    Ok(())
}
