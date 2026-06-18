use std::error::Error;

use tokio_vsock::{VMADDR_CID_ANY, VsockAddr, VsockListener};
use vault_shared::vsock::{VsockEnclaveResult, VsockHostRequest, VsockTransport};

pub mod aes256gcm;
pub mod ed25519_bip32;
pub mod kmstool;
pub mod router;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let port = std::env::var("ENCLAVE_VSOCK_PORT")
        .expect("ENCLAVE_VSOCK_PORT is not set")
        .parse::<u32>()
        .expect("ENCLAVE_VSOCK_PORT must be a u32");

    let listener = VsockListener::bind(VsockAddr::new(VMADDR_CID_ANY, port))?;
    println!("enclave vsock server listening on port {port}");

    loop {
        let (stream, peer_addr) = listener.accept().await?;

        tokio::spawn(async move {
            let mut transport = VsockTransport::new(stream);

            match transport.receive::<VsockHostRequest>().await {
                Ok(request) => {
                    println!("accepted request from {peer_addr:?}: {request:?}");
                }
                Err(err) => {
                    let result = transport
                        .send::<VsockEnclaveResult>(&Err(err.to_string()))
                        .await;
                    eprintln!(
                        "failed to receive request from {peer_addr:?}: {err}. attempted to transport back to host with result {result:?}"
                    );
                }
            }
        });
    }
}
