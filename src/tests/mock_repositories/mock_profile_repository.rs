use async_trait::async_trait;
use std::sync::{Arc, Mutex};
use crate::entities::profile::Profile;
use crate::entities::types::IdType;
use crate::repositories::traits::ProfileRepositoryTrait;
use crate::server_errors::ServerError;
use crate::tests::mock_repositories::mock_transaction::MockTransaction;

#[derive(Clone)]
pub struct MockProfileRepository {
    db: Arc<Mutex<Vec<Profile>>>
}

impl MockProfileRepository {
    pub fn new(users: Arc<Mutex<Vec<Profile>>>) -> Self {
        MockProfileRepository {
            db: users
        }
    }
}

#[async_trait]
impl ProfileRepositoryTrait<MockTransaction> for MockProfileRepository {
    async fn create(&self, _transaction: Option<&mut MockTransaction>, username: String, user_id: IdType) -> Result<Profile, ServerError<String>> {
        let mut db = self.db.lock().unwrap();
        let profile = Profile {
            id: db.len() as IdType,
            username,
            display_name: None,
            bio: None,
            banner: None,
            profile_picture: None,
            user_id,
        };
        db.push(profile.clone());
        Ok(profile)
    }

    async fn find_by_id(&self, _transaction: Option<&mut MockTransaction>, profile_id: IdType) -> Result<Profile, ServerError<String>> {
        let db = self.db.lock().unwrap();
        db.iter().find(|profile| profile.id == profile_id)
            .cloned()
            .ok_or_else(|| ServerError::InternalError(String::from("No profile found with email")))
    }

    async fn find_by_user_id(&self, _transaction: Option<&mut MockTransaction>, user_id: IdType) -> Result<Profile, ServerError<String>> {
        let db = self.db.lock().unwrap();
        db.iter().find(|profile| profile.user_id == user_id)
            .cloned()
            .ok_or_else(|| ServerError::InternalError(String::from("No profile found with email")))
    }

    async fn update_profile_by_id(&self, _transaction: Option<&mut MockTransaction>, profile_id: IdType, display_name: Option<String>, bio: Option<String>, banner: Option<String>, profile_picture: Option<String>) -> Result<(), ServerError<String>> {
        let mut db = self.db.lock().unwrap();
        let position = match db.iter().position(|profile| profile.id == profile_id) {
            Some(position) => position,
            None => return Err(ServerError::ResourceNotFound)
        };
        db.get(position)
            .cloned()
            .ok_or_else(|| ServerError::InternalError(String::from("No profile found with email")))
            .map(|mut profile| {
                profile.display_name = display_name;
                profile.bio = bio;
                profile.banner = banner;
                profile.profile_picture = profile_picture;
                db[position] = profile;
            })
    }

    async fn get_total_profiles_count(&self, _transaction: Option<&mut MockTransaction>) -> Result<IdType, ServerError<String>> {
        Ok(self.db.lock().unwrap().len() as IdType)
    }
}