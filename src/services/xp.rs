use tonic::{Request, Response, Status};

use crate::{error::ServiceError, models::xp::Xp};

use self::pb::{xp_server::Xp as XpService, LevelUp, User};

use super::{pb, Service};

#[tonic::async_trait]
impl XpService for Service {
    async fn add_xp(&self, request: Request<User>) -> Result<Response<LevelUp>, Status> {
        let request = request.into_inner();
        let mut response = LevelUp::default();

        println!("{:?}", request);

        let local_xp = Xp::from_user_id(self, request.user_id)
            .await
            .map_service_error()?;

        let global_xp = Xp::from_user_id(self, request.user_id)
            .await
            .map_service_error()?;

        todo!()
    }
}
