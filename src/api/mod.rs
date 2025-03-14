use crate::can;
use tokio::sync::mpsc::Sender;
use warp::Filter;
mod recv_api;
mod send_api;

pub fn api_filter(
    tx: Sender<can::CanMessage>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    recv_api::recv_filter().or(send_api::send_filter(tx))
}
