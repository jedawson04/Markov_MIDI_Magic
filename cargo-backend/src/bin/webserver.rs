use tokio::fs::File;
// use warp::cors::Cors;
use warp::http::Response;
use warp::hyper::Body;
use warp::Filter;
// use std::env::current_dir;

#[tokio::main]
async fn main() {
    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
        .allow_headers(vec!["Content-Type"]);

    let midi_route = warp::path("midi")
        .and(warp::options().map(warp::reply).with(cors.clone()))
        .or(warp::path("midi").and_then(handle_midi).with(cors));

    warp::serve(midi_route).run(([127, 0, 0, 1], 3030)).await; // creates a server with midi_route fiter on 127.0.0.1:3030
}

async fn handle_midi() -> Result<impl warp::Reply, warp::Rejection> {
    let path = "./src/creations/[\"classical\", \"jazz\"]_order_3_creation.mid".to_string(); // path to creation
                                                                              // let cd = current_dir().unwrap();
                                                                              // println!("{cd:?}");

    let file = File::open(&path).await.unwrap();
    println!("{path}");
    let body = Body::wrap_stream(tokio_util::io::ReaderStream::new(file));
    Ok(Response::new(body))
}
