mod core;

use std::sync::Arc;

use tonic::transport::Server;

use crate::{config::Config, database::Database, services::core::CoreServer};

pub struct Service {
    pub config: Arc<Config>,
    pub database: Database,
}

impl Service {
    pub async fn start(self) {
        let addr = "0.0.0.0:50051".parse().unwrap();

        info!("Listening on {}", addr);

        Server::builder()
            .add_service(CoreServer::new(self))
            .serve(addr)
            .await
            .expect("Fail start server");
    }
}
