mod kgs {
    tonic::include_proto!("kgs");
}

mod links {
    tonic::include_proto!("linksservice");
}

pub use kgs::{
    KeyGenerateRequest, KeyGenerateResponse,
    key_generator_server::KeyGeneratorServer,
    key_generator_server::KeyGenerator,
    key_generator_client::KeyGeneratorClient,
};
