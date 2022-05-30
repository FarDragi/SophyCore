mod xp;

use std::sync::Arc;

use tonic::transport::Server;

use crate::{config::Config, database::Database, services::xp::XpServer};

pub struct Service {
    pub config: Arc<Config>,
    pub database: Database,
}

impl Service {
    pub async fn start(self) {
        let addr = format!("0.0.0.0:{}", self.config.port.unwrap_or(50051))
            .parse()
            .expect("Failed to parse address");

        info!("Listening on {}", addr);

        Server::builder()
            .add_service(XpServer::new(self))
            .serve(addr)
            .await
            .expect("Fail start server");
    }
}
