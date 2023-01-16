use async_trait::async_trait;
use log::error;
use redis::{Expiry, RedisResult};
use redis::aio::ConnectionManager;
use uuid::Uuid;
use redis::AsyncCommands;
use crate::Session;
use serde::{Serialize, Deserialize};
use crate::entities::types::IdType;
use crate::server_errors::ServerError;

#[async_trait]
pub trait SessionStoreFns: Sync + Send {
    async fn create_session(&self, user_id: IdType, profile_id: IdType) -> Result<Session, ()>;
    async fn get_data_of_session(&self, session_id: &str) -> Result<SessionValueInStore, ()>;
    async fn get_sessions_of_user(&self);
    async fn invalidate_session(&self, session_id: &str) -> Result<(), ServerError<String>>;
}

pub type SessionStore = Box<dyn SessionStoreFns>;

pub struct SessionStoreConnection {
    connection: ConnectionManager,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SessionValueInStore {
    pub user_id: IdType,
    pub profile_id: IdType,
}

impl SessionStoreConnection {
    pub async fn new(store_url: &str) -> Box<Self> {
        let client = redis::Client::open(store_url).unwrap();
        let connection = ConnectionManager::new(client).await.unwrap();
        Box::new(
            Self {
                connection
            }
        )
    }
}

#[async_trait]
impl SessionStoreFns for SessionStoreConnection {
    async fn create_session(&self, user_id: IdType, profile_id: IdType) -> Result<Session, ()> {
        let session_id = Uuid::new_v4().to_string();
        let session_value_json =
            match serde_json::to_string(&SessionValueInStore {
                user_id,
                profile_id,
            }) {
                Ok(json) => json,
                Err(error) => {
                    error!("{}", error);
                    return Err(())
                }
            };
        match self.connection.clone()
            .set_ex(
                session_id.clone(),
                session_value_json,
                86400).await as RedisResult<()>
        {
            Ok(_) => Ok(Session {
                id: session_id,
                _user_id: user_id,
                profile_id,
            }),
            Err(_) => Err(())
        }
    }

    async fn get_data_of_session(&self, session_id: &str) -> Result<SessionValueInStore, ()> {
        match self.connection.clone().get_ex(session_id, Expiry::EX(86400)).await as RedisResult<String> {
            Ok(session_value) => {
                let serialize_result = serde_json::from_str(&session_value).map_err(|_| ());
                if let Ok(session_value) = serialize_result {
                    Ok(session_value)
                }
                else {
                    serialize_result
                }
            },
            Err(_) => Err(())
        }
    }

    async fn get_sessions_of_user(&self) {
        todo!()
    }

    async fn invalidate_session(&self, session_id: &str) -> Result<(), ServerError<String>> {
        let result: RedisResult<i32> = self.connection.clone().del(session_id).await;
        match result {
            Ok(_) => Ok(()),
            Err(e) => {
                Err(ServerError::InternalError(e.to_string()))
            }
        }
    }
}