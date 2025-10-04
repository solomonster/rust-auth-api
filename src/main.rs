use tokio::net::TcpListener;
use axum::{Router, routing::get};
mod routes;

#[tokio::main]
async fn main() {

    let server_address: String = "127.0.0.1.8080".to_string();
    let listener = TcpListener::bind(server_address)
        .await
        .expect("Unable to connect to the server");

    let local_address = listener.local_addr().unwrap();

    let app = routes::app().await;
    axum::serve(listener, app)
        .await
        .expect("Error serving application");

    println!("Listening on {}", local_address );

}
