#[cfg(not(feature = "vsock"))]
use tokio::net::TcpStream;
#[cfg(feature = "vsock")]
use tokio_vsock::{VsockAddr, VsockStream};
use vault_shared::transport::{CborTransport, EnclaveResult, HostRequest};

#[cfg(feature = "vsock")]
const ENCLAVE_VSOCK_CID: u32 = 16;

#[cfg(feature = "vsock")]
async fn connect_enclave(enclave_port: u16) -> std::io::Result<VsockStream> {
    VsockStream::connect(VsockAddr::new(ENCLAVE_VSOCK_CID, enclave_port.into())).await
}

#[cfg(not(feature = "vsock"))]
async fn connect_enclave(enclave_port: u16) -> std::io::Result<TcpStream> {
    TcpStream::connect(("127.0.0.1", enclave_port)).await
}

pub async fn send_and_receive_message(enclave_port: u16, request: &HostRequest) -> EnclaveResult {
    let stream = connect_enclave(enclave_port)
        .await
        .map_err(|e| format!("failed to connect to enclave with error: {e}"))?;
    let mut transport = CborTransport::new(stream);
    transport
        .send(request)
        .await
        .map_err(|e| format!("failed to send message to enclave with error: {e}"))?;
    return transport
        .receive::<EnclaveResult>()
        .await
        .map_err(|e| format!("failed to receive message from enclave with error: {e}"))?;
}
