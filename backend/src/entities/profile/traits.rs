use bytes::Bytes;
use crate::entities::profile::model::Profile;
use crate::infrastructure::traits::TransactionTrait;
use crate::server_errors::ServerError;
use crate::utilities::types::IdType;
use async_trait::async_trait;

#[async_trait]
pub trait ProfileServiceTrait: Send + Sync {
    async fn find_profile_by_id(&self, profile_id: IdType) -> Result<Profile, ServerError>;
    async fn update_profile_by_id(&self, profile_id: IdType, display_name: Option<String>, bio: Option<String>, banner: Option<Bytes>, profile_picture: Option<Bytes>) -> Result<(), ServerError>;
    async fn get_total_profiles_count(&self) -> Result<IdType, ServerError>;
}

#[async_trait]
pub trait ProfileRepositoryTrait<T: TransactionTrait>: Send + Sync + Clone {
    async fn create(&self, transaction: Option<&mut T>, profile: Profile) -> Result<Profile, ServerError>;
    async fn find_by_id(&self, transaction: Option<&mut T>, profile_id: IdType) -> Result<Profile, ServerError>;
    async fn find_by_user_id(&self, transaction: Option<&mut T>, user_id: IdType) -> Result<Profile, ServerError>;
    async fn update_profile_by_id(&self, transaction: Option<&mut T>, profile_id: IdType, display_name: Option<String>, bio: Option<String>, banner: Option<String>, profile_picture: Option<String>) -> Result<(), ServerError>;
    async fn get_total_profiles_count(&self, transaction: Option<&mut T>) -> Result<IdType, ServerError>;
}