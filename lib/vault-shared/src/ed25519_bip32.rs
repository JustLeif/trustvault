use bech32::Hrp;
use ed25519_bip32::{self, XPub};
use pallas_crypto::key::ed25519::{self};
use thiserror::Error;

pub const HARDENED: u32 = 0x8000_0000;

#[derive(Error, Debug)]
pub enum FromBech32Error {
    #[error("Unable to decode bech32: {0}")]
    InvalidBech32(bech32::DecodeError),
    #[error("Unexpected bech32 HRP prefix")]
    InvalidBech32Hrp,
    #[error("Decoded bech32 data of unexpected length")]
    UnexpectedBech32Length,
    #[error("Invalid private key bytes: {0}")]
    InvalidPrivateKey(ed25519_bip32::PrivateKeyError),
}

/// Ed25519-BIP32 HD Public Key
#[derive(Debug, PartialEq, Eq)]
pub struct Ed25519Bip32PublicKey(pub ed25519_bip32::XPub);

impl Ed25519Bip32PublicKey {
    const BECH32_HRP: bech32::Hrp = Hrp::parse_unchecked("xpub");

    pub fn from_bytes(bytes: [u8; 64]) -> Self {
        Self(XPub::from_bytes(bytes))
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.0.as_ref().to_vec()
    }

    pub fn derive(&self, index: u32) -> Result<Self, ed25519_bip32::DerivationError> {
        Ok(Self(
            self.0.derive(ed25519_bip32::DerivationScheme::V2, index)?,
        ))
    }

    pub fn to_ed25519_pubkey(&self) -> ed25519::PublicKey {
        self.0.public_key().into()
    }

    pub fn chain_code(&self) -> [u8; 32] {
        *self.0.chain_code()
    }

    pub fn to_bech32(&self) -> Result<String, bech32::EncodeError> {
        return bech32::encode::<bech32::Bech32>(Self::BECH32_HRP, self.0.as_ref());
    }

    pub fn from_bech32(bech32: String) -> Result<Self, FromBech32Error> {
        let (hrp, data) = bech32::decode(&bech32).map_err(FromBech32Error::InvalidBech32)?;
        if hrp != Self::BECH32_HRP {
            return Err(FromBech32Error::InvalidBech32Hrp);
        }

        let bytes = data
            .try_into()
            .map_err(|_| FromBech32Error::UnexpectedBech32Length)?;

        Ok(Self::from_bytes(bytes))
    }
}

#[cfg(test)]
mod test {
    use super::Ed25519Bip32PublicKey;
    #[test]
    fn bech32_roundtrip() {
        let xpub_bech32 = "xpub1m2d3a43r7fccff5tvtzrq2d4sktk0k00wcazpjp64zud7g9d3lmg7a53429ek0wx7xkwslh8mdq9hey07mwt8gl2lua7pmd4v80qhrqkvg3n6";
        let xpub = Ed25519Bip32PublicKey::from_bech32(xpub_bech32.to_string()).unwrap();
        let encoded_xpub_bech32 = xpub.to_bech32().unwrap();

        assert_eq!(xpub_bech32, encoded_xpub_bech32)
    }
}
