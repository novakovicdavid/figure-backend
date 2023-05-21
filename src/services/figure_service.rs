use async_trait::async_trait;
use crate::entities::dtos::figure_dto::FigureDTO;
use crate::entities::types::IdType;
use crate::repositories::figure_repository::FigureRepositoryTrait;
use crate::server_errors::ServerError;

#[async_trait]
pub trait FigureServiceTrait: Send + Sync {
    async fn find_figure_by_id(&self, figure_id: IdType) -> Result<FigureDTO, ServerError<String>>;
    // async fn update_profile_by_id(&self, profile_id: IdType, display_name: Option<String>, bio: Option<String>, banner: Option<String>, profile_picture: Option<String>) -> Result<(), ServerError<String>>;
}

pub struct FigureService<F: FigureRepositoryTrait> {
    figure_repository: F,
}

impl<F: FigureRepositoryTrait> FigureService<F> {
    pub fn new(figure_repository: F) -> impl FigureServiceTrait {
        FigureService {
            figure_repository
        }
    }
}

#[async_trait]
impl<F: FigureRepositoryTrait> FigureServiceTrait for FigureService<F> {
    async fn find_figure_by_id(&self, figure_id: IdType) -> Result<FigureDTO, ServerError<String>> {
        self.figure_repository.find_by_id(None, figure_id).await
    }
}