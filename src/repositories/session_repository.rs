use async_trait::async_trait;
use redis::aio::ConnectionManager;
use redis::{AsyncCommands, Expiry, RedisResult};
use crate::entities::types::IdType;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::Session;
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
pub trait SessionRepositoryTrait: Send + Sync + Clone {
    async fn create(&self, user_id: IdType, profile_id: IdType, time_until_expiration: Option<usize>) -> Result<Session, ServerError<String>>;
    async fn find_by_id(&self, session_id: &str, time_until_expiration: Option<usize>) -> Result<Session, ServerError<String>>;
    async fn remove_by_id(&self, session_id: &str) -> Result<(), ServerError<String>>;
}

#[async_trait]
impl SessionRepositoryTrait for SessionRepository {
    async fn create(&self, user_id: IdType, profile_id: IdType, time_until_expiration: Option<usize>) -> Result<Session, ServerError<String>> {
        let session_id = Uuid::new_v4().to_string();
        let session = SessionValueInStore {
            user_id,
            profile_id,
        };
        let session_value_json = match serde_json::to_string(&session) {
            Ok(json) => json,
            Err(e) => {
                return Err(ServerError::InternalError(e.to_string()));
            }
        };

        let mut connection = self.connection.clone();
        let result = match time_until_expiration {
            Some(time) => connection.set_ex(
                session_id.clone(),
                session_value_json,
                time),
            None => {
                connection.set(
                    session_id.clone(),
                    session_value_json,
                )
            }
        }.await;

        match result {
            Ok(()) => Ok(Session {
                id: session_id.to_string(),
                _user_id: user_id,
                profile_id,
            }),
            Err(e) => Err(ServerError::InternalError(e.to_string()))
        }
    }

    async fn find_by_id(&self, session_id: &str, time_until_expiration: Option<usize>) -> Result<Session, ServerError<String>> {
        let mut connection = self.connection.clone();
        let result: RedisResult<String> = match time_until_expiration {
            Some(time) => connection.get_ex(session_id, Expiry::EX(time)),
            None => connection.get(session_id)
        }.await;

        match result {
            Ok(session_string) => {
                serde_json::from_str::<SessionValueInStore>(&session_string)
                    .map(|value| Session {
                        id: session_id.to_string(),
                        _user_id: value.user_id,
                        profile_id: value.profile_id,
                    })
                    .map_err(|e| ServerError::InternalError(e.to_string()))
            }
            Err(e) => Err(ServerError::InternalError(e.to_string()))
        }
    }

    async fn remove_by_id(&self, session_id: &str) -> Result<(), ServerError<String>> {
        self.connection
            .clone()
            .del(session_id)
            .await
            .map_err(|e| ServerError::InternalError(e.to_string()))
    }
}