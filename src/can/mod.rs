mod can_message;
mod queue;

use futures_util::stream::StreamExt;
use tokio::sync::mpsc::Receiver;
use tokio_socketcan::{CANFrame, CANSocket};

pub use can_message::CanMessage;
pub use queue::{dequeue, enqueue, size};

pub async fn can_perioic_task(mut rx: Receiver<CanMessage>) {
    let mut socket_rx = CANSocket::open("vcan0").unwrap();
    let socket_tx = CANSocket::open("vcan0").unwrap();
    loop {
        tokio::select! {
            Some(message) = rx.recv() => {
                socket_tx.write_frame(CANFrame::new(message.can_id, &message.body, false, false).unwrap()).unwrap();
            }
            Some(container) = socket_rx.next() => {
                if let Ok(can_frame) = container {
                    enqueue(CanMessage::new(can_frame.id(), can_frame.data())).await;
                }
            }
        }
    }
}
