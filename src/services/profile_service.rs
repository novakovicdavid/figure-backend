use async_trait::async_trait;
use crate::entities::profile::Profile;
use crate::entities::types::IdType;
use crate::repositories::profile_repository::ProfileRepositoryTrait;
use crate::server_errors::ServerError;

#[async_trait]
pub trait ProfileServiceTrait: Send + Sync {
    async fn find_profile_by_id(&self, profile_id: IdType) -> Result<Profile, ServerError<String>>;
    async fn update_profile_by_id(&self, profile_id: IdType, display_name: Option<String>, bio: Option<String>, banner: Option<String>, profile_picture: Option<String>) -> Result<(), ServerError<String>>;
}

pub struct ProfileService<P: ProfileRepositoryTrait> {
    profile_repository: P,
}

impl<P: ProfileRepositoryTrait> ProfileService<P> {
    pub fn new(profile_repository: P) -> impl ProfileServiceTrait {
        ProfileService {
            profile_repository
        }
    }
}

#[async_trait]
impl<P: ProfileRepositoryTrait> ProfileServiceTrait for ProfileService<P> {
    async fn find_profile_by_id(&self, profile_id: IdType) -> Result<Profile, ServerError<String>> {
        self.profile_repository.find_by_id(None, profile_id).await
    }

    async fn update_profile_by_id(&self, profile_id: IdType, display_name: Option<String>, bio: Option<String>, banner: Option<String>, profile_picture: Option<String>) -> Result<(), ServerError<String>> {
        self.profile_repository.update_profile_by_id(None, profile_id, display_name, bio, banner, profile_picture).await
    }
}