use bdk_esplora::esplora_client::{BlockingClient, Builder};

pub struct EsploraClient(BlockingClient);

impl EsploraClient {
    pub fn new(url: String) -> Self {
        let client = Builder::new(url.as_str()).build_blocking().unwrap();
        Self(client)
    }

    // pub fn scan();

    // pub fn sync();

    // pub fn broadcast();

    // pub fn estimate_fee();
}
