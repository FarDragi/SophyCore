use tonic::{Request, Response, Status};

use crate::{
    cache::xp::XpCacheCommands, error::ServiceError, models::xp::Xp, services::pb::LevelInfo,
};

use self::pb::{xp_server::Xp as XpService, LevelUp, User};

use super::{pb, Service};

#[tonic::async_trait]
impl XpService for Service {
    async fn add_xp(&self, request: Request<User>) -> Result<Response<LevelUp>, Status> {
        let request = request.into_inner();
        let mut response = LevelUp::default();

        let user_id = request.user_id;
        let guild_id = request.guild_id;

        let mut guild_xp = Xp::from_user_id(self, user_id).await.map_service_error()?;

        let (guild_add, guild_up) = guild_xp.add();

        response.guild = Some(LevelInfo {
            add: guild_add,
            up: guild_up,
            level: guild_xp.level,
        });

        self.cache
            .set_user_guild_xp(user_id, guild_id, &guild_xp)
            .await
            .map_service_error()?;

        let mut global_xp = Xp::from_user_id_and_guild_id(self, user_id, guild_id)
            .await
            .map_service_error()?;

        let (global_add, global_up) = global_xp.add();

        response.global = Some(LevelInfo {
            add: global_add,
            up: global_up,
            level: global_xp.level,
        });

        self.cache
            .set_user_global_xp(user_id, &global_xp)
            .await
            .map_service_error()?;

        Ok(Response::new(response))
    }
}
