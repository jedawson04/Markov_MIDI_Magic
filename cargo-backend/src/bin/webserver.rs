use warp::Filter;
use warp::http::Response;
use warp::hyper::Body;
use tokio::fs::File;
use warp::cors::Cors;

#[tokio::main]
async fn main() {
    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
        .allow_headers(vec!["Content-Type"]);

    let midi_route = warp::path("midi")
        .and(warp::options().map(warp::reply).with(cors.clone()))
        .or(warp::path("midi").and_then(handle_midi).with(cors));


warp::serve(midi_route)
    .run(([127, 0, 0, 1], 3030))
    .await;
}

async fn handle_midi() -> Result<impl warp::Reply, warp::Rejection> {
    let file = File::open("src\\midi-files-by-genre\\test\\jazz_creation.mid").await.unwrap();
    let body = Body::wrap_stream(tokio_util::io::ReaderStream::new(file));
    Ok(Response::new(body))
}
