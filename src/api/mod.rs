use warp::Filter;
mod recv_api;

pub fn api_filter() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    recv_api::recv_filter()
}
