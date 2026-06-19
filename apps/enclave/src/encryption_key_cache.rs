use std::{
    collections::HashMap,
    sync::{Arc, LazyLock},
};
use tokio::sync::RwLock;

pub type EncryptionKey = Arc<Vec<u8>>;

static ENCRYPTION_KEY_CACHE: LazyLock<RwLock<HashMap<Vec<u8>, EncryptionKey>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

pub async fn get(encrypted_key: &[u8]) -> Option<EncryptionKey> {
    ENCRYPTION_KEY_CACHE
        .read()
        .await
        .get(encrypted_key)
        .cloned()
}

pub async fn insert(encrypted_key: Vec<u8>, decrypted_key: EncryptionKey) {
    ENCRYPTION_KEY_CACHE
        .write()
        .await
        .insert(encrypted_key, decrypted_key);
}
