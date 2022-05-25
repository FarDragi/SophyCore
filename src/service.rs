use tonic::{Request, Response, Status};

use self::core::{core_server::Core, AddXpRequest, LevelUpResponse};

mod core {
    tonic::include_proto!("core");
}

pub struct Service {}

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
