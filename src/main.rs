use tokio::sync::mpsc;
mod api;
mod can;

#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::channel::<can::CanMessage>(1024);

    tokio::spawn(can::can_perioic_task(rx));

    
    warp::serve(api::api_filter())
        .run(([127, 0, 0, 1], 3030))
        .await;
}
