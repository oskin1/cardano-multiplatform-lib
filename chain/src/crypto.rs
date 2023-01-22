use super::*;

/// Crypto-related on-chain structures. See the crypto crate for actually using these.

use cml_crypto;

use cml_crypto::chain::ChainCrypto;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct KesSignature {
    pub inner: Vec<u8>,
    #[serde(skip)]
    pub encodings: Option<KesSignatureEncoding>,
}

impl KesSignature {
    pub fn get(&self) -> &Vec<u8> {
        &self.inner
    }

    pub fn new(inner: Vec<u8>) -> Result<Self, DeserializeError> {
        if inner.len() != 32 {
            return Err(DeserializeError::new("KesSignature", DeserializeFailure::RangeCheck{ found: inner.len(), min: Some(32), max: Some(32) }));
        }
        Ok(Self {
            inner,
            encodings: None,
        })
    }
}

impl TryFrom<Vec<u8>> for KesSignature {
  type Error = DeserializeError;

  fn try_from(inner: Vec<u8>) -> Result<Self, Self::Error> {
      KesSignature::new(inner)
  }
}

impl From<KesSignature> for Vec<u8> {
  fn from(wrapper: KesSignature) -> Self {
      wrapper.inner
  }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct Nonce1 {
    pub bytes: Vec<u8>,
    #[serde(skip)]
    pub encodings: Option<Nonce1Encoding>,
}

impl Nonce1 {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self {
            bytes,
            encodings: None,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub enum Nonce {
    I0 {
        #[serde(skip)]
        i0_encoding: Option<cbor_event::Sz>,
        #[serde(skip)]
        outer_len_encoding: LenEncoding,
    },
    Nonce1(Nonce1),
}

impl Nonce {
    pub fn new_i0() -> Self {
        Self::I0 {
            i0_encoding: None,
            outer_len_encoding: LenEncoding::default(),
        }
    }

    pub fn new_nonce1(bytes: Vec<u8>) -> Self {
        Self::Nonce1(Nonce1::new(bytes))
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct SignkeyKES {
    pub inner: Vec<u8>,
    #[serde(skip)]
    pub encodings: Option<SignkeyKESEncoding>,
}

impl SignkeyKES {
    pub fn get(&self) -> &Vec<u8> {
        &self.inner
    }

    pub fn new(inner: Vec<u8>) -> Result<Self, DeserializeError> {
        if inner.len() != 16 {
            return Err(DeserializeError::new("SignkeyKES", DeserializeFailure::RangeCheck{ found: inner.len(), min: Some(16), max: Some(16) }));
        }
        Ok(Self {
            inner,
            encodings: None,
        })
    }
}

impl TryFrom<Vec<u8>> for SignkeyKES {
    type Error = DeserializeError;

    fn try_from(inner: Vec<u8>) -> Result<Self, Self::Error> {
        SignkeyKES::new(inner)
    }
}

impl From<SignkeyKES> for Vec<u8> {
    fn from(wrapper: SignkeyKES) -> Self {
        wrapper.inner
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct VrfCert {
    pub index_0: Vec<u8>,
    pub bytes: Vec<u8>,
    #[serde(skip)]
    pub encodings: Option<VrfCertEncoding>,
}

impl VrfCert {
    pub fn new(index_0: Vec<u8>, bytes: Vec<u8>) -> Self {
        Self {
            index_0,
            bytes,
            encodings: None,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct Vkeywitness {
    pub vkey: Vkey,
    pub signature: Ed25519Signature,
    #[serde(skip)]
    pub encodings: Option<VkeywitnessEncoding>,
}

impl Vkeywitness {
    pub fn new(vkey: Vkey, signature: Ed25519Signature) -> Self {
        Self {
            vkey,
            signature,
            encodings: None,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct BootstrapWitness {
    pub vkey: Vkey,
    pub signature: Ed25519Signature,
    pub chain_code: Vec<u8>,
    // TODO: this should be replaced by AddrAttributes when Byron is brought over
    pub attributes: Vec<u8>,
    #[serde(skip)]
    pub encodings: Option<BootstrapWitnessEncoding>,
}

impl BootstrapWitness {
    pub fn new(vkey: Vkey, signature: Ed25519Signature, chain_code: Vec<u8>, attributes: Vec<u8>) -> Self {
        Self {
            vkey,
            signature,
            chain_code,
            attributes,
            encodings: None,
        }
    }

    // pub fn to_public_key(&self) -> Result<crypto::Bip32PublicKey, crypto::CryptoError> {
    //     crypto::chain_crypto::PublicKey::<chain_crypto::ed25519_derive::Ed25519Bip32>::try_from(self.clone())
    //         .map(crypto::Bip32PublicKey)
    //         .map_err(Into::into)
    // }

    // pub fn to_address(&self) -> Result<AddressContent, CryptoError> {
    //     AddressContent::try_from(self.clone())
    //         .map_err(Into::into)
    // }
}

// impl TryFrom<BootstrapWitness> for chain_crypto::PublicKey<chain_crypto::ed25519_derive::Ed25519Bip32> {
//     type Error = ed25519_bip32::PublicKeyError;

//     fn try_from(wit: BootstrapWitness) -> Result<Self, Self::Error> {
//         combine_pk_and_chaincode(wit.vkey.pubkey.0, &wit.chain_code)
//     }
// }

// impl TryFrom<BootstrapWitness> for AddressContent {
//     type Error = ed25519_bip32::PublicKeyError;

//     fn try_from(wit: BootstrapWitness) -> Result<Self, Self::Error> {
//         let protocol_magic = wit.attributes.protocol_magic;
//         let key = chain_crypto::PublicKey::<chain_crypto::ed25519_derive::Ed25519Bip32>::try_from(wit)?;
//         let address_content = AddressContent::new_simple(&Bip32PublicKey(key), protocol_magic);
//         Ok(address_content)
//     }
// }

pub type Ed25519Signature = ChainCrypto<cml_crypto::Ed25519Signature>;

pub type Vkey = ChainCrypto<cml_crypto::PublicKey>;

pub type Ed25519KeyHash = ChainCrypto<cml_crypto::Ed25519KeyHash>;
pub type ScriptHash = ChainCrypto<cml_crypto::ScriptHash>;
// TransactionHash is either a hash of the tx CBOR or a hash of a redeem address (genesis)
pub type TransactionHash = ChainCrypto<cml_crypto::TransactionHash>;
pub type GenesisDelegateHash = ChainCrypto<cml_crypto::GenesisDelegateHash>;
pub type GenesisHash = ChainCrypto<cml_crypto::GenesisHash>;
pub type AuxiliaryDataHash = ChainCrypto<cml_crypto::AuxiliaryDataHash>;
pub type PoolMetadataHash = ChainCrypto<cml_crypto::PoolMetadataHash>;
pub type VRFKeyHash = ChainCrypto<cml_crypto::VRFKeyHash>;
pub type BlockBodyHash = ChainCrypto<cml_crypto::BlockBodyHash>;
pub type BlockHeaderHash = ChainCrypto<cml_crypto::BlockHeaderHash>;
pub type DataHash = ChainCrypto<cml_crypto::DataHash>;
pub type ScriptDataHash = ChainCrypto<cml_crypto::ScriptDataHash>;
pub type VRFVKey = ChainCrypto<cml_crypto::VRFVKey>;
pub type KESVKey = ChainCrypto<cml_crypto::KESVKey>;