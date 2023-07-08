use async_trait::async_trait;
use std::sync::{Arc, Mutex};
use crate::entities::dtos::session_dtos::Session;
use crate::repositories::traits::SessionRepositoryTrait;
use crate::server_errors::ServerError;

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
    async fn create(&self, session: Session) -> Result<Session, ServerError<String>> {
        let mut db = self.connection.lock().unwrap();
        db.push(session.clone());
        Ok(session)
    }

    async fn find_by_id(&self, session_id: &str, _time_until_expiration: Option<usize>) -> Result<Session, ServerError<String>> {
        let db = self.connection.lock().unwrap();
        match db.iter().find(|session| session.get_id() == session_id) {
            Some(session) => Ok(session.clone()),
            None => Err(ServerError::ResourceNotFound),
        }
    }

    async fn remove_by_id(&self, session_id: &str) -> Result<(), ServerError<String>> {
        let mut db = self.connection.lock().unwrap();
        match db.iter().position(|session| session.get_id() == session_id) {
            Some(position) => {
                db.remove(position);
                Ok(())
            },
            None => Err(ServerError::ResourceNotFound),
        }
    }
}