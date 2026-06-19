use serde::{Deserialize, Serialize};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_vsock::VsockStream;

pub const VSOCK_MAX_MESSAGE_LEN: usize = 1024 * 1024;

pub struct VsockTransport {
    stream: VsockStream,
}

impl VsockTransport {
    pub fn new(stream: VsockStream) -> Self {
        return Self { stream };
    }

    pub async fn receive<T: for<'de> Deserialize<'de>>(&mut self) -> Result<T, VsockReceiveError> {
        let mut len_bytes = [0u8; 4];
        self.stream.read_exact(&mut len_bytes).await?;
        let len = u32::from_be_bytes(len_bytes) as usize;
        if len > VSOCK_MAX_MESSAGE_LEN {
            return Err(VsockReceiveError::MessageTooLarge {
                len,
                max: VSOCK_MAX_MESSAGE_LEN,
            });
        }
        let mut buf = vec![0u8; len];
        self.stream.read_exact(&mut buf).await?;
        let message: T = serde_cbor::from_slice(&buf)?;
        return Ok(message);
    }

    pub async fn send<T: Serialize>(&mut self, message: &T) -> Result<(), VsockSendError> {
        let cbor_bytes = serde_cbor::to_vec(message)?;
        let len = cbor_bytes.len();
        if len > VSOCK_MAX_MESSAGE_LEN {
            return Err(VsockSendError::MessageTooLarge {
                len,
                max: VSOCK_MAX_MESSAGE_LEN,
            });
        }
        let len = len as u32;
        self.stream.write_all(&len.to_be_bytes()).await?;
        self.stream.write_all(&cbor_bytes).await?;
        self.stream.flush().await?;
        return Ok(());
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum VsockHostRequest {
    CardanoCreateWallet {
        aws_region: String,
        aws_access_key_id: String,
        aws_secret_access_key: String,
        aws_session_token: String,
        kms_proxy_port: String,
        kms_key_id: String,
    },
    CardanoSignTransaction {
        tx_cbor: Vec<u8>,
        partial_sign: bool,
        aws_region: String,
        aws_access_key_id: String,
        aws_secret_access_key: String,
        aws_session_token: String,
        kms_proxy_port: String,
        aes_gcm_nonce: [u8; 12],
        encrypted_secret_key: Vec<u8>,
        kms_ciphertext: Vec<u8>,
    },
}

pub type VsockEnclaveResult = Result<VsockEnclaveResponse, String>;

#[derive(Serialize, Deserialize, Debug)]
pub enum VsockEnclaveResponse {
    VsockEnclaveCardanoCreateWalletData {
        encrypted_secret_key: Vec<u8>,
        aes_gcm_nonce: [u8; 12],
        kms_ciphertext: Vec<u8>,
        aws_region: String,
        kms_key_id: String,
        account_index_0_xpub_bech32: String,
    },
}

#[derive(Debug, thiserror::Error)]
pub enum VsockReceiveError {
    #[error("failed to stream.read_exact()")]
    Io(#[from] std::io::Error),
    #[error("vsock message too large: {len} bytes exceeds max {max} bytes")]
    MessageTooLarge { len: usize, max: usize },
    #[error("failed to deserialize cbor")]
    Deserialization(#[from] serde_cbor::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum VsockSendError {
    #[error("failed to stream.write_all()")]
    Io(#[from] std::io::Error),
    #[error("vsock message too large: {len} bytes exceeds max {max} bytes")]
    MessageTooLarge { len: usize, max: usize },
    #[error("failed to serialize cbor")]
    Serialization(#[from] serde_cbor::Error),
}
