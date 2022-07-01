use tonic::{Request, Response, Status};

use crate::{error::ServiceError, models::xp::Xp, services::pb::LevelInfo};

use self::pb::{xp_server::Xp as XpService, LevelUp, User};

use super::{pb, Service};

#[tonic::async_trait]
impl XpService for Service {
    async fn add_xp(&self, request: Request<User>) -> Result<Response<LevelUp>, Status> {
        let request = request.into_inner();
        let mut response = LevelUp::default();

        let user_id = request.user_id;
        let guild_id = request.guild_id;

        {
            let guild_xp = Xp::from_user_id(self, user_id).await.map_service_error()?;

            let guild_update = guild_xp.add();

            response.guild = Some(LevelInfo {
                add: guild_update.1.add,
                up: guild_update.1.up,
                level: guild_xp.level,
            });

            guild_xp.save(self).await.map_service_error()?;
        }

        {
            let global_xp = Xp::from_user_id_and_guild_id(self, user_id, guild_id)
                .await
                .map_service_error()?;

            let global_update = global_xp.add();

            response.global = Some(LevelInfo {
                add: global_update.1.add,
                up: global_update.1.up,
                level: global_xp.level,
            });

            global_xp.save(self).await.map_service_error()?;
        }

        Ok(Response::new(response))
    }
}
