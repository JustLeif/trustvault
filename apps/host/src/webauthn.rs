use webauthn_rs::prelude::*;

pub fn start(
    rp_id: &str,
    rp_origin: &Url,
    uuid: Uuid,
    username: &str,
    display_name: &str,
) -> Result<(CreationChallengeResponse, PasskeyRegistration), WebauthnError> {
    let builder = WebauthnBuilder::new(rp_id, rp_origin)?;
    let webauthn = builder.build()?;
    Ok(webauthn.start_passkey_registration(uuid, username, display_name, None)?)
}
