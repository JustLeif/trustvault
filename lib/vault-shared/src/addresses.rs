use pallas_addresses::{
    Network, ShelleyAddress, ShelleyDelegationPart, ShelleyPaymentPart, StakeAddress, StakePayload,
};

use crate::ed25519_bip32::Ed25519Bip32PublicKey;
use pallas_crypto::hash::Hash;
use pallas_crypto::hash::Hasher;

fn key_hash(xpub: &Ed25519Bip32PublicKey) -> Hash<28> {
    let public_key = xpub.to_ed25519_pubkey();
    Hasher::<224>::hash(public_key.as_ref())
}

pub fn base_address(
    network: Network,
    payment_xpub: &Ed25519Bip32PublicKey,
    stake_xpub: Option<&Ed25519Bip32PublicKey>,
) -> Result<String, pallas_addresses::Error> {
    let payment_hash = key_hash(payment_xpub);
    let delegation = match stake_xpub {
        Some(stake) => ShelleyDelegationPart::key_hash(key_hash(stake)),
        None => ShelleyDelegationPart::Null,
    };

    let addr = ShelleyAddress::new(
        network,
        ShelleyPaymentPart::key_hash(payment_hash),
        delegation,
    );

    addr.to_bech32()
}

pub fn reward_address(
    network: Network,
    stake_xpub: &Ed25519Bip32PublicKey,
) -> Result<String, pallas_addresses::Error> {
    let stake_hash = key_hash(stake_xpub);

    let addr = StakeAddress::new(network, StakePayload::Stake(stake_hash));

    addr.to_bech32()
}
