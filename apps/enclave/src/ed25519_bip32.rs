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

    use super::*;
    use pallas_addresses::Network;
    use vault_shared::{
        addresses::{base_address, reward_address},
        ed25519_bip32::HARDENED,
    };

    // Using this bech32 value, test against yoroi wallet derivations.
    const KNOWN_XPRV_BECH32: &'static str = "xprv18zmg0nvgtcmzw5rrt432v25zle3uz9p5j7g58xa8revmrc6vjay5806rwm6yfm9k7pjpmxk4qzf6wk5p5m02u7s0m7u7fqaa06njhp3hpycq2243077u8vhca6ndkcyqady6rre9kzpnwz53kswyywnfdyhvjyte";

    #[test]
    fn bech32_roundtrip() {
        let xprv = Ed25519Bip32PrivateKey::generate().unwrap();
        let xprv_bech32 = xprv.to_bech32().unwrap();
        let decoded_xprv = Ed25519Bip32PrivateKey::from_bech32(xprv_bech32).unwrap();

        assert_eq!(xprv, decoded_xprv);
    }

    #[test]
    fn test_derivation() {
        let root = Ed25519Bip32PrivateKey::from_bech32(KNOWN_XPRV_BECH32.to_string()).unwrap();

        const EXTERNAL_MAINNET_ADDRESS_INDEX_0: &'static str = "addr1qxg6d6226c85ldttenpps5wnl8pdk3m4x0h05kpn66znxs8v2eqdycpdc6kdyhfg7rncgj5wakasqjwsu6qghle2cx0qcrvtvu";
        const EXTERNAL_MAINNET_ADDRESS_INDEX_1: &'static str = "addr1qyrefsup6w6adj9t26k4wnzee4dmfsk55ta5zfctun2akf0v2eqdycpdc6kdyhfg7rncgj5wakasqjwsu6qghle2cx0qunanxx";
        const INTERNAL_MAINNET_ADDRESS_INDEX_0: &'static str = "addr1q9gax5u3wv7v7cjdnrxrd3eqp7mplj9yc06lk9dam6ln3w0v2eqdycpdc6kdyhfg7rncgj5wakasqjwsu6qghle2cx0qtgzzwv";
        const REWARD_MAINNET_ADDRESS_INDEX_0: &'static str =
            "stake1u8k9vsxjvqkudtxjt550peuyf28wmwcqf8gwdqytlu4vr8sxsw76h";

        let external_xpub_index_0 = root
            .derive(1852 | HARDENED)
            .derive(1815 | HARDENED)
            .derive(0 | HARDENED)
            .derive(0)
            .derive(0)
            .to_public();
        let external_xpub_index_1 = root
            .derive(1852 | HARDENED)
            .derive(1815 | HARDENED)
            .derive(0 | HARDENED)
            .derive(0)
            .derive(1)
            .to_public();
        let internal_xpub_index_0 = root
            .derive(1852 | HARDENED)
            .derive(1815 | HARDENED)
            .derive(0 | HARDENED)
            .derive(1)
            .derive(0)
            .to_public();
        let reward_xpub_index_0 = root
            .derive(1852 | HARDENED)
            .derive(1815 | HARDENED)
            .derive(0 | HARDENED)
            .derive(2)
            .derive(0)
            .to_public();

        assert_eq!(
            EXTERNAL_MAINNET_ADDRESS_INDEX_0,
            base_address(
                Network::Mainnet,
                &external_xpub_index_0,
                Some(&reward_xpub_index_0)
            )
            .unwrap()
        );
        assert_eq!(
            EXTERNAL_MAINNET_ADDRESS_INDEX_1,
            base_address(
                Network::Mainnet,
                &external_xpub_index_1,
                Some(&reward_xpub_index_0)
            )
            .unwrap()
        );
        assert_eq!(
            INTERNAL_MAINNET_ADDRESS_INDEX_0,
            base_address(
                Network::Mainnet,
                &internal_xpub_index_0,
                Some(&reward_xpub_index_0)
            )
            .unwrap()
        );
        assert_eq!(
            REWARD_MAINNET_ADDRESS_INDEX_0,
            reward_address(Network::Mainnet, &reward_xpub_index_0).unwrap()
        );
    }
}
