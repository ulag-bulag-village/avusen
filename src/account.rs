use std::ops::Deref;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Decoder {
    pub private_key: PrivateKey,
    #[serde(flatten)]
    pub encoder: Encoder,
}

impl Deref for Decoder {
    type Target = Encoder;

    fn deref(&self) -> &Self::Target {
        &self.encoder
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Encoder {
    pub public_key: PublicKey,
    pub signature: String,
}

pub type PublicKey = String;

pub type PrivateKey = String;
