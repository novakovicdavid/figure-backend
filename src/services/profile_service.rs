use std::marker::PhantomData;
use async_trait::async_trait;
use bytes::Bytes;
use uuid::Uuid;
use crate::content_store::ContentStore;
use crate::entities::profile::Profile;
use crate::entities::types::IdType;
use crate::repositories::profile_repository::ProfileRepositoryTrait;
use crate::repositories::transaction::TransactionTrait;
use crate::server_errors::ServerError;

#[async_trait]
pub trait ProfileServiceTrait: Send + Sync {
    async fn find_profile_by_id(&self, profile_id: IdType) -> Result<Profile, ServerError<String>>;
    async fn update_profile_by_id(&self, profile_id: IdType, display_name: Option<String>, bio: Option<String>, banner: Option<Bytes>, profile_picture: Option<Bytes>) -> Result<(), ServerError<String>>;
}

pub struct ProfileService<T: TransactionTrait, P: ProfileRepositoryTrait<T>, S: ContentStore> {
    profile_repository: P,
    storage: S,
    marker: PhantomData<T>,
}

impl<T: TransactionTrait, P: ProfileRepositoryTrait<T>, S: ContentStore> ProfileService<T, P, S> {
    pub fn new(profile_repository: P, storage: S) -> Self {
        Self {
            profile_repository,
            storage,
            marker: PhantomData::default(),
        }
    }
}

#[async_trait]
impl<T: TransactionTrait, P: ProfileRepositoryTrait<T>, S: ContentStore> ProfileServiceTrait for ProfileService<T, P, S> {
    async fn find_profile_by_id(&self, profile_id: IdType) -> Result<Profile, ServerError<String>> {
        self.profile_repository.find_by_id(None, profile_id).await
    }

    async fn update_profile_by_id(&self, profile_id: IdType, display_name: Option<String>, bio: Option<String>, banner: Option<Bytes>, profile_picture: Option<Bytes>) -> Result<(), ServerError<String>> {
        let mut banner_url = None;
        let mut profile_picture_url = None;

        if let Some(banner) = banner {
            let url = format!("banners/{}", Uuid::new_v4());
            banner_url = self.storage.upload_image(url.as_str(), banner)
                .await
                .map(Some)
                .map_err(|e| ServerError::InternalError(e.to_string()))?;
        }
        if let Some(profile_picture) = profile_picture {
            let url = format!("profile_pictures/{}", Uuid::new_v4());
            profile_picture_url = self.storage.upload_image(url.as_str(), profile_picture)
                .await
                .map(Some)
                .map_err(|e| ServerError::InternalError(e.to_string()))?;
        }
        self.profile_repository.update_profile_by_id(None, profile_id, display_name, bio, banner_url, profile_picture_url).await
    }
}