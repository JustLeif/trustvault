use bech32::Hrp;
use ed25519_bip32::{self, XPRV_SIZE, XPrv};
use pallas_crypto::key::ed25519::SecretKeyExtended;
use vault_shared::ed25519_bip32::{Ed25519Bip32PublicKey, FromBech32Error};

/// Ed25519-BIP32 HD Private Key
#[derive(Debug, PartialEq, Eq)]
pub struct Ed25519Bip32PrivateKey(ed25519_bip32::XPrv);

impl Ed25519Bip32PrivateKey {
    const BECH32_HRP: bech32::Hrp = Hrp::parse_unchecked("xprv");

    pub fn generate() -> Result<Self, getrandom::Error> {
        let mut buf = [0u8; XPRV_SIZE];
        getrandom::fill(&mut buf)?;
        let xprv = XPrv::normalize_bytes_force3rd(buf);
        Ok(Self(xprv))
    }

    pub fn from_bytes(bytes: [u8; 96]) -> Result<Self, ed25519_bip32::PrivateKeyError> {
        Ok(Self(XPrv::from_bytes_verified(bytes)?))
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.0.as_ref().to_vec()
    }

    pub fn derive(&self, index: u32) -> Self {
        Self(self.0.derive(ed25519_bip32::DerivationScheme::V2, index))
    }

    pub fn to_ed25519_secret_key_extended(&self) -> SecretKeyExtended {
        unsafe { SecretKeyExtended::from_bytes_unchecked(self.0.extended_secret_key()) }
    }

    pub fn to_public(&self) -> Ed25519Bip32PublicKey {
        Ed25519Bip32PublicKey(self.0.public())
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

        Self::from_bytes(bytes).map_err(FromBech32Error::InvalidPrivateKey)
    }
}
#[cfg(test)]
mod test {

    use super::Ed25519Bip32PrivateKey;

    #[test]
    fn bech32_roundtrip() {
        let xprv = Ed25519Bip32PrivateKey::generate().unwrap();
        let xprv_bech32 = xprv.to_bech32().unwrap();
        let decoded_xprv = Ed25519Bip32PrivateKey::from_bech32(xprv_bech32).unwrap();

        assert_eq!(xprv, decoded_xprv);
    }
}
