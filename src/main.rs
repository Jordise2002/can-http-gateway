use clap::Parser;
use tokio::sync::mpsc;

mod api;
mod can;

#[derive(Parser)]
struct Args {
    //Socketcan Interface's name
    interface: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let (tx, rx) = mpsc::channel::<can::CanMessage>(1024);

    tokio::spawn(can::can_perioic_task(rx, args.interface));

    warp::serve(api::api_filter(tx))
        .run(([127, 0, 0, 1], 3030))
        .await;
}
