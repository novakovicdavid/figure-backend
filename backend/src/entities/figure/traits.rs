use bytes::Bytes;
use crate::domain::figure::Figure;
use crate::entities::figure::dtos::FigureWithProfileDTO;
use crate::domain::profile::Profile;
use crate::infrastructure::traits::TransactionTrait;
use crate::server_errors::ServerError;
use crate::utilities::types::IdType;
use async_trait::async_trait;

#[async_trait]
pub trait FigureServiceTrait: Send + Sync {
    async fn find_figure_by_id(&self, figure_id: IdType) -> Result<FigureWithProfileDTO, ServerError>;
    async fn find_figures_starting_from_id_with_profile_id(&self, figure_id: Option<IdType>, profile_id: Option<IdType>, limit: i32) -> Result<Vec<FigureWithProfileDTO>, ServerError>;
    async fn create(&self, title: String, description: Option<String>, image: Bytes, width: u32, height: u32, profile_id: IdType) -> Result<Figure, ServerError>;
    async fn get_total_figures_by_profile(&self, figure_id: IdType) -> Result<IdType, ServerError>;
    async fn get_total_figures_count(&self) -> Result<IdType, ServerError>;
}

#[async_trait]
pub trait FigureRepositoryTrait<T: TransactionTrait>: Send + Sync + Clone {
    async fn create(&self, transaction: Option<&mut T>, figure: Figure) -> Result<Figure, ServerError>;
    async fn find_by_id(&self, transaction: Option<&mut T>, figure_id: IdType) -> Result<(Figure, Profile), ServerError>;
    async fn find_starting_from_id_with_profile_id(&self, transaction: Option<&mut T>, figure_id: Option<IdType>, profile_id: Option<IdType>, limit: i32) -> Result<Vec<(Figure, Profile)>, ServerError>;
    async fn update_figure(&self, transaction: Option<&mut T>, figure: Figure) -> Result<(), ServerError>;
    async fn delete_figure_by_id(&self, transaction: Option<&mut T>, figure_id: IdType) -> Result<(), ServerError>;
    async fn count_by_profile_id(&self, transaction: Option<&mut T>, profile_id: IdType) -> Result<IdType, ServerError>;
    async fn get_total_figures_count(&self, transaction: Option<&mut T>) -> Result<IdType, ServerError>;
}