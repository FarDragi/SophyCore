use tonic::{Request, Response, Status};

use self::pb::{core_server::Core, AddXpRequest, LevelUpResponse};

use super::Service;

pub mod pb {
    tonic::include_proto!("core");
}

pub use pb::core_server::CoreServer;

#[tonic::async_trait]
impl Core for Service {
    async fn add_xp(
        &self,
        request: Request<AddXpRequest>,
    ) -> Result<Response<LevelUpResponse>, Status> {
        let request = request.into_inner();

        println!("{:?}", request);

        unimplemented!()
    }
}
