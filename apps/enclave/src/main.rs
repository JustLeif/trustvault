#[cfg(not(feature = "vsock"))]
use tokio::net::TcpListener;
#[cfg(feature = "vsock")]
use tokio_vsock::{VMADDR_CID_ANY, VsockAddr, VsockListener};
use vault_shared::transport::{CborTransport, EnclaveResult, HostRequest};

pub mod aes256gcm;
pub mod ed25519_bip32;
pub mod encryption_key_cache;
pub mod kmstool;
pub mod router;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let enclave_port = std::env::var("ENCLAVE_PORT")
        .expect("ENCLAVE_PORT is not set")
        .parse::<u16>()
        .expect("ENCLAVE_PORT must be a u16");

    let listener = {
        #[cfg(feature = "vsock")]
        {
            VsockListener::bind(VsockAddr::new(VMADDR_CID_ANY, enclave_port.into()))?
        }
        #[cfg(not(feature = "vsock"))]
        {
            TcpListener::bind(("127.0.0.1", enclave_port)).await?
        }
    };
    println!("enclave server listening on port {enclave_port}");

    loop {
        let (stream, peer_addr) = listener.accept().await?;

        tokio::spawn(async move {
            let mut transport = CborTransport::new(stream);

            match transport.receive::<HostRequest>().await {
                Ok(request) => {
                    println!("accepted request from {peer_addr:?}: {request:?}");
                    let result = crate::router::handle_host_request(request).await;
                    if let Err(err) = transport.send::<EnclaveResult>(&result).await {
                        eprintln!("failed to send enclave result: {err}");
                    }
                }
                Err(err) => {
                    let result = transport.send::<EnclaveResult>(&Err(err.to_string())).await;
                    eprintln!(
                        "failed to receive request from {peer_addr:?}: {err}. attempted to transport back to host with result {result:?}"
                    );
                }
            }
        });
    }
}
