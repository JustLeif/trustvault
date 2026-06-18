#[derive(Debug, thiserror::Error)]
pub enum KmsToolError {
    #[error("failed to Command::new().output()")]
    Io(#[from] std::io::Error),
    #[error(
        "failed to parse command output, status: {status}. stdout: {stdout}. stderr: {stderr}."
    )]
    StdoutParse {
        stdout: String,
        status: String,
        stderr: String,
    },
    #[error("failed to decode stdout from base64")]
    DecodeError(#[from] base64::DecodeError),
}
