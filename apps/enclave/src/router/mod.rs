use crate::{
    aes256gcm::decrypt_private_key_aes256gcm,
    aes256gcm::encrypt_private_key_aes256gcm,
    ed25519_bip32::Ed25519Bip32PrivateKey,
    encryption_key_cache,
    kmstool::{decrypt, genkey},
};
use vault_shared::{
    ed25519_bip32::HARDENED,
    vsock::{VsockEnclaveResponse, VsockEnclaveResult, VsockHostRequest},
};

/// TODO: inject a config object to mock external calls to perform E2E tests against this function.
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
            let account_index_0_xpub_bech32 = root_xprv
                .derive(1852 | HARDENED)
                .derive(1815 | HARDENED)
                .derive(0 | HARDENED)
                .to_public()
                .to_bech32()
                .map_err(|e| {
                    format!("failed to encode index 0 account xpub into bech32 with error: {e}")
                })?;

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

            encryption_key_cache::insert(
                encryption_key_ciphertext.clone(),
                std::sync::Arc::new(encryption_key_plaintext),
            )
            .await;

            return Ok(VsockEnclaveResponse::CardanoCreateWalletData {
                encrypted_secret_key: private_key_ciphertext,
                aes_gcm_nonce: nonce,
                kms_ciphertext: encryption_key_ciphertext,
                aws_region: aws_region,
                kms_key_id: kms_key_id,
                account_index_0_xpub_bech32: account_index_0_xpub_bech32,
            });
        }
        VsockHostRequest::CardanoSignTransaction {
            tx_hash,
            aws_region,
            aws_access_key_id,
            aws_secret_access_key,
            aws_session_token,
            kms_proxy_port,
            aes_gcm_nonce,
            encrypted_secret_key,
            kms_ciphertext,
            cip1852_account,
            cip1852_index,
            cip1852_role,
        } => {
            let plaintext_encryption_key = match encryption_key_cache::get(&kms_ciphertext).await {
                Some(plaintext_key) => plaintext_key,
                None => {
                    let [plaintext_key] = decrypt(
                        &aws_region,
                        &aws_access_key_id,
                        &aws_secret_access_key,
                        &aws_session_token,
                        &kms_proxy_port,
                        &kms_ciphertext,
                    )
                    .await
                    .map_err(|e| {
                        format!(
                            "failed to decrypt encryption key using aws kmstool with error: {e}"
                        )
                    })?;
                    let plaintext_key = std::sync::Arc::new(plaintext_key);

                    encryption_key_cache::insert(kms_ciphertext.clone(), plaintext_key.clone())
                        .await;

                    plaintext_key
                }
            };

            let raw_secret_key: [u8; 96] = decrypt_private_key_aes256gcm(
                &encrypted_secret_key,
                &plaintext_encryption_key,
                &aes_gcm_nonce,
            )
            .map_err(|e| format!("failed to decrypt raw secret key with error: {e}"))?
            .try_into()
            .map_err(|bytes: Vec<u8>| {
                format!(
                    "failed to convert private key: expected 96 private key bytes, got {}",
                    bytes.len()
                )
            })?;

            let root_xprv = Ed25519Bip32PrivateKey::from_bytes(raw_secret_key).map_err(|e| {
                format!("failed to build an Ed25519Bip32PrivateKey with error: {e}")
            })?;

            let child_xprv = root_xprv
                .derive(1852 | HARDENED)
                .derive(1815 | HARDENED)
                .derive(cip1852_account | HARDENED)
                .derive(cip1852_role)
                .derive(cip1852_index);
            let child_xpub = child_xprv.to_public();

            let signer = child_xprv.to_ed25519_secret_key_extended();
            let signature = signer.sign(tx_hash.as_ref());

            Ok(VsockEnclaveResponse::CardanoSignTransactionData {
                public_key: child_xpub.as_bytes().to_vec(),
                signature: signature.as_ref().to_vec(),
            })
        }
    }
}
