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
    let port = std::env::var("PORT")
        .expect("PORT is not set")
        .parse::<u32>()
        .expect("PORT must be a u32");

    let listener = {
        #[cfg(feature = "vsock")]
        {
            VsockListener::bind(VsockAddr::new(VMADDR_CID_ANY, port))?
        }
        #[cfg(not(feature = "vsock"))]
        {
            let tcp_port = u16::try_from(port).expect("TCP port must fit in u16");
            TcpListener::bind(("127.0.0.1", tcp_port)).await?
        }
    };
    println!("enclave server listening on port {port}");

    loop {
        let (stream, peer_addr) = listener.accept().await?;

        tokio::spawn(async move {
            let mut transport = CborTransport::new(stream);

            match transport.receive::<HostRequest>().await {
                Ok(request) => {
                    println!("accepted request from {peer_addr:?}: {request:?}");
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
