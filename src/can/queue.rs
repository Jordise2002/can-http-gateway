use crate::can::CanMessage;
use circular_buffer::CircularBuffer;
use lazy_static::lazy_static;
use std::sync::Arc;
use tokio::sync::Mutex;

type ProtectedQueue = Arc<Mutex<CircularBuffer<4096, CanMessage>>>;

lazy_static! {
    static ref QUEUE: ProtectedQueue = Arc::new(Mutex::new(CircularBuffer::new()));
}

pub async fn enqueue(value: CanMessage) {
    let mut queue = QUEUE.lock().await;
    queue.push_front(value);
}

pub async fn dequeue(ammount: u32) -> Vec<CanMessage> {
    let mut queue = QUEUE.lock().await;
    let mut result = Vec::new();

    for _i in 0..ammount {
        let value = queue.pop_back();
        if let Some(value) = value {
            result.push(value);
        } else {
            break;
        }
    }

    result
}

pub async fn size() -> u32 {
    let queue = QUEUE.lock().await;
    queue.len() as u32
}
