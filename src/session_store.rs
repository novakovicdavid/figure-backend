use async_trait::async_trait;
use log::error;
use redis::{Expiry, RedisResult};
use redis::aio::ConnectionManager;
use uuid::Uuid;
use redis::AsyncCommands;
use crate::Session;
use serde::{Serialize, Deserialize};

#[async_trait]
pub trait SessionStoreFns: Sync + Send {
    async fn create_session(&self, user_id: i64, profile_id: i64) -> Result<Session, ()>;
    async fn get_data_of_session(&self, session_id: String) -> Result<SessionValueInStore, ()>;
    async fn get_sessions_of_user(&self);
}

pub type SessionStore = Box<dyn SessionStoreFns>;

pub struct SessionStoreConnection {
    connection: ConnectionManager,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SessionValueInStore {
    pub user_id: i64,
    pub profile_id: i64,
}

impl SessionStoreConnection {
    pub async fn new(store_url: String) -> SessionStore {
        let client = redis::Client::open(store_url).unwrap();
        let connection = ConnectionManager::new(client).await.unwrap();
        Box::new(
            SessionStoreConnection {
                connection
            }
        )
    }
}

#[async_trait]
impl SessionStoreFns for SessionStoreConnection {
    async fn create_session(&self, user_id: i64, profile_id: i64) -> Result<Session, ()> {
        let session_id = Uuid::new_v4().to_string();
        let session_value_json =
            match serde_json::to_string(&SessionValueInStore {
                user_id,
                profile_id: profile_id,
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
                user_id: user_id,
                profile_id: profile_id,
            }),
            Err(_) => Err(())
        }
    }

    async fn get_data_of_session(&self, session_id: String) -> Result<SessionValueInStore, ()> {
        match self.connection.clone().get_ex(session_id, Expiry::EX(86400)).await as RedisResult<String> {
            Ok(session_value) => {
                let serialize_result = serde_json::from_str(&session_value).or_else(|_| Err(()));
                if serialize_result.is_ok() {
                    Ok(serialize_result.unwrap() as SessionValueInStore)
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
}