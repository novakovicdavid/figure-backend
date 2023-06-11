use async_trait::async_trait;
use bytes::Bytes;
use uuid::Uuid;
use crate::content_store::ContentStore;
use crate::entities::dtos::figure_dto::FigureDTO;
use crate::entities::figure::Figure;
use crate::entities::types::IdType;
use crate::repositories::figure_repository::FigureRepositoryTrait;
use crate::server_errors::ServerError;

#[async_trait]
pub trait FigureServiceTrait: Send + Sync {
    async fn find_figure_by_id(&self, figure_id: IdType) -> Result<FigureDTO, ServerError<String>>;
    async fn find_figures_starting_from_id_with_profile_id(&self, figure_id: Option<IdType>, profile_id: Option<IdType>, limit: i32) -> Result<Vec<FigureDTO>, ServerError<String>>;
    async fn create(&self, title: String, description: Option<String>, image: Bytes, width: u32, height: u32, profile_id: IdType) -> Result<Figure, ServerError<String>>;
    // async fn update_profile_by_id(&self, profile_id: IdType, display_name: Option<String>, bio: Option<String>, banner: Option<String>, profile_picture: Option<String>) -> Result<(), ServerError<String>>;
}

pub struct FigureService<F: FigureRepositoryTrait, S: ContentStore> {
    figure_repository: F,
    storage: S,
}

impl<F: FigureRepositoryTrait, S: ContentStore> FigureService<F, S> {
    pub fn new(figure_repository: F, storage: S) -> Self {
        Self {
            figure_repository,
            storage
        }
    }
}

#[async_trait]
impl<F: FigureRepositoryTrait, S: ContentStore> FigureServiceTrait for FigureService<F, S> {
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
}