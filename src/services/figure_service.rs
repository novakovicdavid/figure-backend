use async_trait::async_trait;
use bytes::Bytes;
use uuid::Uuid;
use crate::content_store::ContentStore;
use crate::entities::dtos::figure_dto::FigureDTO;
use crate::entities::figure::Figure;
use crate::entities::types::IdType;
use crate::repositories::traits::{FigureRepositoryTrait, TransactionTrait};
use crate::server_errors::ServerError;
use crate::services::traits::FigureServiceTrait;

pub struct FigureService<T: TransactionTrait> {
    figure_repository: Box<dyn FigureRepositoryTrait<T>>,
    storage: Box<dyn ContentStore>,
}

impl<T: TransactionTrait> FigureService<T> {
    pub fn new(figure_repository: Box<dyn FigureRepositoryTrait<T>>, storage: Box<dyn ContentStore>) -> Self {
        Self {
            figure_repository,
            storage,
        }
    }
}

#[async_trait]
impl<T: TransactionTrait> FigureServiceTrait for FigureService<T> {
    async fn find_figure_by_id(&self, figure_id: IdType) -> Result<FigureDTO, ServerError<String>> {
        self.figure_repository.find_by_id(None, figure_id).await
    }

    async fn find_figures_starting_from_id_with_profile_id(&self, figure_id: Option<IdType>, profile_id: Option<IdType>, limit: i32) -> Result<Vec<FigureDTO>, ServerError<String>> {
        self.figure_repository.find_starting_from_id_with_profile_id(None, figure_id, profile_id, limit).await
    }

    async fn create(&self, title: String, description: Option<String>, image: Bytes, width: u32, height: u32, profile_id: IdType) -> Result<Figure, ServerError<String>> {
        if width > i32::MAX as u32 || height > i32::MAX as u32 {
            return Err(ServerError::ImageDimensionsTooLarge)
        }

        let uid = Uuid::new_v4();
        let uid = uid.to_string();
        let url = self.storage.upload_image(uid.as_str(), image).await?;
        self.figure_repository.create(None, Figure {
            id: 0,
            title,
            description,
            width: width as i32,
            height: height as i32,
            url,
            profile_id,
        }).await
    }

    async fn get_total_figures_by_profile(&self, profile_id: IdType) -> Result<IdType, ServerError<String>> {
        self.figure_repository.count_by_profile_id(None, profile_id).await
    }

    async fn get_total_figures_count(&self) -> Result<IdType, ServerError<String>> {
        self.figure_repository.get_total_figures_count(None).await
    }
}