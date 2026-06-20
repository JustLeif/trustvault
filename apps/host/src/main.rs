#[cfg(not(feature = "vsock"))]
use tokio::net::TcpStream;
#[cfg(feature = "vsock")]
use tokio_vsock::{VsockAddr, VsockStream};
use tonic::{Request, Response, Status, transport::Server};
use vault_shared::transport::{CborTransport, EnclaveResponse, EnclaveResult, HostRequest};

pub mod trustvault_proto {
    tonic::include_proto!("trustvault");
}
pub mod enclave;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let enclave_port = std::env::var("ENCLAVE_PORT")
        .expect("ENCLAVE_PORT is not set")
        .parse::<u16>()
        .expect("ENCLAVE_PORT must be a u16");
    let host_port = std::env::var("HOST_PORT")
        .expect("HOST_PORT is not set")
        .parse::<u16>()
        .expect("HOST_PORT must be a u16");

    let result = enclave::send_and_receive_message(
        enclave_port,
        &HostRequest::CardanoCreateWallet {
            aws_region: String::from("fake"),
            aws_access_key_id: String::from("fake"),
            aws_secret_access_key: String::from("fake"),
            aws_session_token: String::from("fake"),
            kms_proxy_port: String::from("fake"),
            kms_key_id: String::from("fake"),
        },
    )
    .await;

    println!("{result:?}");
    Ok(())
}
