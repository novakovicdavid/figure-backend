use std::marker::PhantomData;
use async_trait::async_trait;
use bytes::Bytes;
use uuid::Uuid;
use tracing::instrument;
use crate::content_store::ContentStore;
use crate::entities::figure::dtos::FigureWithProfileDTO;
use crate::domain::figure::Figure;
use crate::entities::figure::traits::{FigureRepositoryTrait, FigureServiceTrait};
use crate::infrastructure::traits::TransactionTrait;
use crate::utilities::types::IdType;
use crate::server_errors::ServerError;

pub struct FigureService<T, F, S> {
    figure_repository: F,
    storage: S,
    marker: PhantomData<T>,
}

impl<T: TransactionTrait, F: FigureRepositoryTrait<T>, S: ContentStore> FigureService<T, F, S> {
    pub fn new(figure_repository: F, storage: S) -> Self {
        Self {
            figure_repository,
            storage,
            marker: PhantomData::default(),
        }
    }
}

#[async_trait]
impl<T, F, S> FigureServiceTrait for FigureService<T, F, S>
    where T: TransactionTrait, F: FigureRepositoryTrait<T>, S: ContentStore {
    async fn find_figure_by_id(&self, figure_id: IdType) -> Result<FigureWithProfileDTO, ServerError> {
        self.figure_repository.find_by_id(None, figure_id)
            .await
            .map(|(figure, profile)| FigureWithProfileDTO::from(figure, profile))
    }

    #[instrument(level = "trace", skip(self))]
    async fn find_figures_starting_from_id_with_profile_id(&self, figure_id: Option<IdType>, profile_id: Option<IdType>, limit: i32) -> Result<Vec<FigureWithProfileDTO>, ServerError> {
        self.figure_repository.find_starting_from_id_with_profile_id(None, figure_id, profile_id, limit)
            .await
            .map(|figures_and_profiles| figures_and_profiles.into_iter()
                .map(|figure_and_profile| FigureWithProfileDTO::from(figure_and_profile.0, figure_and_profile.1)).collect())
    }

    async fn create(&self, title: String, description: Option<String>, image: Bytes, width: u32, height: u32, profile_id: IdType) -> Result<Figure, ServerError> {
        Figure::check_size(width, height)?;
        let uid = Uuid::new_v4();
        let uid = uid.to_string();
        let url = self.storage.upload_image(uid.as_str(), image).await?;
        self.figure_repository.create(None, Figure::new(
            0,
            title,
            description,
            width as i32,
            height as i32,
            url,
            profile_id,
        )?)
            .await
    }

    async fn get_total_figures_by_profile(&self, profile_id: IdType) -> Result<IdType, ServerError> {
        self.figure_repository.count_by_profile_id(None, profile_id)
            .await
    }

    async fn get_total_figures_count(&self) -> Result<IdType, ServerError> {
        self.figure_repository.get_total_figures_count(None)
            .await
    }
}