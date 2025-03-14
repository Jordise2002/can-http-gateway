#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CanMessage {
    pub can_id: u32,
    pub body: Vec<u8>,
}

impl CanMessage {
    pub fn new(can_id: u32, body: &[u8]) -> Self {
        CanMessage {
            can_id,
            body: Vec::from(body),
        }
    }
}
