use std::marker::PhantomData;
use async_trait::async_trait;
use bytes::Bytes;
use uuid::Uuid;
use crate::content_store::ContentStore;
use crate::entities::profile::model::Profile;
use crate::entities::profile::traits::{ProfileRepositoryTrait, ProfileServiceTrait};
use crate::infrastructure::traits::TransactionTrait;
use crate::utilities::types::IdType;
use crate::server_errors::ServerError;

pub struct ProfileService<T, P, S> {
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
impl<T, P, S> ProfileServiceTrait for ProfileService<T, P, S>
    where T: TransactionTrait, P: ProfileRepositoryTrait<T>, S: ContentStore {
    async fn find_profile_by_id(&self, profile_id: IdType) -> Result<Profile, ServerError> {
        self.profile_repository.find_by_id(None, profile_id).await
    }

    async fn update_profile_by_id(&self, profile_id: IdType, display_name: Option<String>, bio: Option<String>, banner: Option<Bytes>, profile_picture: Option<Bytes>) -> Result<(), ServerError> {
        let mut banner_url = None;
        let mut profile_picture_url = None;

        if let Some(banner) = banner {
            let url = format!("banners/{}", Uuid::new_v4());
            banner_url = self.storage.upload_image(url.as_str(), banner)
                .await
                .map(Some)
                .map_err(|e| ServerError::InternalError(e.into()))?;
        }
        if let Some(profile_picture) = profile_picture {
            let url = format!("profile_pictures/{}", Uuid::new_v4());
            profile_picture_url = self.storage.upload_image(url.as_str(), profile_picture)
                .await
                .map(Some)?;
        }
        self.profile_repository.update_profile_by_id(None, profile_id, display_name, bio, banner_url, profile_picture_url).await
    }

    async fn get_total_profiles_count(&self) -> Result<IdType, ServerError> {
        self.profile_repository.get_total_profiles_count(None).await
    }
}