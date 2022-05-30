use tonic::{Request, Response, Status};

use self::pb::{xp_server::Xp, LevelUp, User};

use super::Service;

pub mod pb {
    tonic::include_proto!("core");
}

pub use pb::xp_server::XpServer;

#[tonic::async_trait]
impl Xp for Service {
    async fn add_xp(&self, request: Request<User>) -> Result<Response<LevelUp>, Status> {
        let request = request.into_inner();

        println!("{:?}", request);

        unimplemented!()
    }
}
