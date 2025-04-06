mod kgs {
    tonic::include_proto!("kgs");
}

pub use kgs::{
    KeyGenerateRequest, KeyGenerateResponse,
    key_generator_server::KeyGeneratorServer,
    key_generator_server::KeyGenerator,
};
