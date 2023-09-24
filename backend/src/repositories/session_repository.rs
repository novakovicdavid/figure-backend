use async_trait::async_trait;
use redis::aio::ConnectionManager;
use redis::{AsyncCommands, Expiry, RedisResult};
use crate::domain::models::types::IdType;
use serde::{Serialize, Deserialize};
use crate::entities::dtos::session_dtos::Session;
use crate::repositories::traits::SessionRepositoryTrait;
use crate::server_errors::ServerError;

#[derive(Clone)]
pub struct SessionRepository {
    connection: ConnectionManager,
}

impl SessionRepository {
    pub fn new(connection: ConnectionManager) -> Self {
        SessionRepository {
            connection
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SessionValueInStore {
    pub user_id: IdType,
    pub profile_id: IdType,
}

#[async_trait]
impl SessionRepositoryTrait for SessionRepository {
    async fn create(&self, session: Session) -> Result<Session, ServerError> {
        let session_in_store = SessionValueInStore {
            user_id: session.get_user_id(),
            profile_id: session.get_profile_id(),
        };
        let session_value_json = match serde_json::to_string(&session_in_store) {
            Ok(json) => json,
            Err(e) => {
                return Err(ServerError::InternalError(e.into()));
            }
        };

        let mut connection = self.connection.clone();
        let result = match session.get_time_until_expiration() {
            Some(time) => connection.set_ex(
                session.get_id(),
                session_value_json,
                time),
            None => {
                connection.set(
                    session.get_id(),
                    session_value_json,
                )
            }
        }.await;

        match result {
            Ok(()) => Ok(session),
            Err(e) => Err(ServerError::InternalError(e.into()))
        }
    }

    async fn find_by_id(&self, session_id: &str, time_until_expiration: Option<usize>) -> Result<Session, ServerError> {
        let mut connection = self.connection.clone();
        let result: RedisResult<String> = match time_until_expiration {
            Some(time) => connection.get_ex(session_id, Expiry::EX(time)),
            None => connection.get(session_id)
        }.await;

        match result {
            Ok(session_string) => {
                serde_json::from_str::<SessionValueInStore>(&session_string)
                    .map(|value| Session::new(
                        session_id.to_string(),
                        value.user_id,
                        value.profile_id,
                        None,
                    ))
                    .map_err(|e| ServerError::InternalError(e.into()))
            }
            Err(_) => Err(ServerError::ResourceNotFound)
        }
    }

    async fn remove_by_id(&self, session_id: &str) -> Result<(), ServerError> {
        self.connection
            .clone()
            .del(session_id)
            .await
            .map_err(|e| ServerError::InternalError(e.into()))
    }
}