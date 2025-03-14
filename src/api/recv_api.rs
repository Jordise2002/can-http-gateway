use warp::Filter;

pub fn recv_filter() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let recv_base = warp::path("recv");

    let packet_ammount = recv_base
        .and(warp::path("ammount"))
        .and(warp::get())
        .and(warp::path::end())
        .and_then(handle_packet_ammount);

    let packet_request = recv_base
        .and(warp::get())
        .and(warp::path::param())
        .and_then(handle_packet_request);

    packet_ammount.or(packet_request)
}

async fn handle_packet_ammount() -> Result<warp::reply::Json, warp::Rejection> {
    let packet_amount = serde_json::json!({"ammount": crate::can::size().await});

    let packet_amount = warp::reply::json(&packet_amount);

    Ok(packet_amount)
}

async fn handle_packet_request(ammount: u32) -> Result<warp::reply::Json, warp::Rejection> {
    let result = crate::can::dequeue(ammount).await;

    let result = warp::reply::json(&result);

    Ok(result)
}
