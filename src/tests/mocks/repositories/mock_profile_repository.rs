use async_trait::async_trait;
use std::sync::{Arc, Mutex};
use crate::domain::models::profile::Profile;
use crate::domain::models::types::IdType;
use crate::repositories::traits::ProfileRepositoryTrait;
use crate::server_errors::ServerError;
use crate::tests::mocks::repositories::mock_transaction::MockTransaction;

#[derive(Clone)]
pub struct MockProfileRepository {
    db: Arc<Mutex<Vec<Profile>>>
}

impl MockProfileRepository {
    pub fn new() -> Self {
        MockProfileRepository {
            db: Arc::new(Mutex::new(Vec::new()))
        }
    }
}

#[async_trait]
impl ProfileRepositoryTrait<MockTransaction> for MockProfileRepository {
    async fn create(&self, _transaction: Option<&mut MockTransaction>, mut profile: Profile) -> Result<Profile, ServerError> {
        let mut db = self.db.lock().unwrap();
        profile.set_id(db.len() as IdType);
        db.push(profile.clone());
        Ok(profile)
    }

    async fn find_by_id(&self, _transaction: Option<&mut MockTransaction>, profile_id: IdType) -> Result<Profile, ServerError> {
        let db = self.db.lock().unwrap();
        db.iter().find(|profile| profile.get_id() == profile_id)
            .cloned()
            .ok_or_else(|| ServerError::ResourceNotFound)
    }

    async fn find_by_user_id(&self, _transaction: Option<&mut MockTransaction>, user_id: IdType) -> Result<Profile, ServerError> {
        let db = self.db.lock().unwrap();
        db.iter().find(|profile| profile.get_user_id() == user_id)
            .cloned()
            .ok_or_else(|| ServerError::ResourceNotFound)
    }

    async fn update_profile_by_id(&self, _transaction: Option<&mut MockTransaction>, profile_id: IdType, display_name: Option<String>, bio: Option<String>, banner: Option<String>, profile_picture: Option<String>) -> Result<(), ServerError> {
        let mut db = self.db.lock().unwrap();
        let position = match db.iter().position(|profile| profile.get_id() == profile_id) {
            Some(position) => position,
            None => return Err(ServerError::ResourceNotFound)
        };
        db.get(position)
            .cloned()
            .ok_or_else(|| ServerError::ResourceNotFound)
            .map(|mut profile| {
                profile.set_display_name(display_name);
                profile.set_bio(bio);
                profile.set_banner(banner);
                profile.set_profile_picture(profile_picture);
                db[position] = profile;
            })
    }

    async fn get_total_profiles_count(&self, _transaction: Option<&mut MockTransaction>) -> Result<IdType, ServerError> {
        Ok(self.db.lock().unwrap().len() as IdType)
    }
}