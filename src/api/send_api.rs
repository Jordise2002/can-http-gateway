use crate::can;
use tokio::sync::mpsc::Sender;
use warp::Filter;
pub fn send_filter(
    tx: Sender<can::CanMessage>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let base_route = warp::path("send");

    let send = base_route
        .and(warp::post())
        .and(warp::body::json())
        .map(move |value: serde_json::Value| (tx.clone(), value))
        .and_then(|(tx, value)| handle_send(value, tx));

    send
}

async fn handle_send(
    value: serde_json::Value,
    tx: Sender<can::CanMessage>,
) -> Result<impl warp::Reply, warp::Rejection> {
    if let Some(messages) = value.as_array() {
        for message in messages {
            if let Ok(real_message) = serde_json::from_value::<can::CanMessage>(message.clone()) {
                tx.send(real_message).await.unwrap();
            } else {
                return Ok(warp::reply::with_status(
                    "Bad Request",
                    warp::http::StatusCode::BAD_REQUEST,
                ));
            }
        }
    }
    Ok(warp::reply::with_status("OK", warp::http::StatusCode::OK))
}
