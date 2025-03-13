mod api;

#[tokio::main]
async fn main() {


    
    warp::serve(api::api_filter())
        .run(([127, 0, 0, 1], 3030))
        .await;
}
