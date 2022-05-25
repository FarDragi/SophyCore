mod service;

use service::{core::core_server::CoreServer, Service};
use tonic::transport::Server;

#[tokio::main]
async fn main() {
    let addr = "0.0.0.0:50051".parse().unwrap();
    let service = Service {};

    println!("Listening on {}", addr);

    Server::builder()
        .add_service(CoreServer::new(service))
        .serve(addr)
        .await
        .expect("Fail start server");
}
