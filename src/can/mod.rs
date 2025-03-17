mod can_message;
mod queue;

pub use can_message::CanMessage;
use futures_util::stream::StreamExt;
pub use queue::{dequeue, enqueue, size};
use socketcan::{tokio::CanSocket, CanFrame, CanId, EmbeddedFrame, Frame, SocketOptions};
use tokio::sync::mpsc::Receiver;

pub async fn can_perioic_task(mut rx: Receiver<CanMessage>, interface: String) {
    let socket_rx = CanSocket::open(&interface);
    let socket_tx = CanSocket::open(&interface);

    let (mut socket_rx, socket_tx) = if socket_rx.is_err() || socket_tx.is_err() {
        eprintln!("Couldn't open socketcan interface: {}", interface);
        std::process::exit(-1);
    } else {
        (socket_rx.unwrap(), socket_tx.unwrap())
    };

    let loop_back_result = socket_tx.set_loopback(false);

    if loop_back_result.is_err()  {
        eprint!("Couldn't open socketcan interface: {}", interface);
        std::process::exit(-1);
    }

    loop {
        tokio::select! {
            Some(message) = rx.recv() => {
                socket_tx.write_frame(CanFrame::new(CanId::standard(message.can_id as u16).unwrap(), &message.body).unwrap()).await.unwrap();
            }
            Some(container) = socket_rx.next() => {
                if let Ok(can_frame) = container {
                    enqueue(CanMessage::new(can_frame.can_id().as_raw() as u16, can_frame.data())).await;
                }
            }
        }
    }
}
