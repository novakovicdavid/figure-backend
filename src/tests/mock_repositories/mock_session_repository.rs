use async_trait::async_trait;
use std::sync::{Arc, Mutex};
use crate::entities::types::IdType;
use crate::repositories::session_repository::SessionRepositoryTrait;
use crate::server_errors::ServerError;
use crate::Session;

#[derive(Clone)]
pub struct MockSessionRepository {
    connection: Arc<Mutex<Vec<Session>>>,
}

impl MockSessionRepository {
    pub fn new(connection: Arc<Mutex<Vec<Session>>>) -> Self {
        MockSessionRepository {
            connection
        }
    }
}

#[async_trait]
impl SessionRepositoryTrait for MockSessionRepository {
    async fn create(&self, user_id: IdType, profile_id: IdType, _time_until_expiration: Option<usize>) -> Result<Session, ServerError<String>> {
        let mut db = self.connection.lock().unwrap();
        let session_id = db.len().to_string();
        let session = Session {
            id: session_id.clone(),
            _user_id: user_id,
            profile_id,
        };
        db.push(session);
        Ok(Session {
            id: session_id,
            _user_id: user_id,
            profile_id,
        })
    }

    async fn find_by_id(&self, session_id: &str, _time_until_expiration: Option<usize>) -> Result<Session, ServerError<String>> {
        let db = self.connection.lock().unwrap();
        match db.iter().find(|session| session.id == session_id) {
            Some(session) => Ok(session.clone()),
            None => Err(ServerError::ResourceNotFound),
        }
    }

    async fn remove_by_id(&self, session_id: &str) -> Result<(), ServerError<String>> {
        let mut db = self.connection.lock().unwrap();
        match db.iter().position(|session| session.id == session_id) {
            Some(position) => {
                db.remove(position);
                Ok(())
            },
            None => Err(ServerError::ResourceNotFound),
        }
    }
}