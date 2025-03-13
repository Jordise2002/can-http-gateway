mod recv_api;

#[tokio::main]
async fn main() {
    warp::serve(recv_api::recv_filter())
        .run(([127, 0, 0, 1], 3030))
        .await;
}
