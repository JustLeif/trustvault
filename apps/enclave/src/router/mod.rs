use crate::{
    aes256gcm::encrypt_private_key_aes256gcm, ed25519_bip32::Ed25519Bip32PrivateKey,
    kmstool::genkey,
};
use vault_shared::vsock::{VsockEnclaveResponse, VsockEnclaveResult, VsockHostRequest};

pub async fn handle_host_request(request: VsockHostRequest) -> VsockEnclaveResult {
    match request {
        VsockHostRequest::CardanoCreateWallet {
            aws_region,
            aws_access_key_id,
            aws_secret_access_key,
            aws_session_token,
            kms_proxy_port,
            kms_key_id,
        } => {
            let root_xprv = Ed25519Bip32PrivateKey::generate()
                .map_err(|e| format!("failed to generate private key with error: {e}"))?;
            let mut nonce = [0u8; 12];
            getrandom::fill(&mut nonce)
                .map_err(|e| format!("failed to generate nonce with error: {e}"))?;

            let [encryption_key_ciphertext, encryption_key_plaintext] = genkey(
                &aws_region,
                &aws_access_key_id,
                &aws_secret_access_key,
                &aws_session_token,
                &kms_proxy_port,
                &kms_key_id,
                "AES-256",
            )
            .await
            .map_err(|e| format!("failed to generate a kms secured key with error: {e}"))?;

            let private_key_ciphertext = encrypt_private_key_aes256gcm(
                &root_xprv.as_bytes(),
                &encryption_key_plaintext,
                &nonce,
            ).map_err(|e| format!("failed to encrypt root private key using kms provided encryption key with error: {e}"))?;

            return Ok(VsockEnclaveResponse::VsockEnclaveCardanoCreateWalletData {
                encrypted_secret_key: private_key_ciphertext,
                aes_gcm_nonce: nonce,
                kms_ciphertext: encryption_key_ciphertext,
                aws_region: aws_region,
                kms_key_id: kms_key_id,
            });
        }
        VsockHostRequest::CardanoSignTransaction {
            tx_cbor,
            partial_sign,
            aws_region,
            aws_access_key_id,
            aws_secret_access_key,
            aws_session_token,
            kms_proxy_port,
            aes_gcm_nonce,
            encrypted_secret_key,
            kms_ciphertext,
        } => {
            todo!("still need to implement signing...");
        }
    }
}
