use std::marker::PhantomData;
use async_trait::async_trait;
use bytes::Bytes;
use crate::entities::dtos::figure_dto::FigureDTO;
use crate::entities::dtos::profile_dto::ProfileDTO;
use crate::entities::figure::Figure;
use crate::entities::profile::Profile;
use crate::entities::types::IdType;
use crate::server_errors::ServerError;
use crate::Session;

#[async_trait]
pub trait UserServiceTrait: Send + Sync {
    async fn signup_user(&self, email: String, password: String, username: String) -> Result<(ProfileDTO, Session), ServerError<String>>;
    async fn authenticate_user(&self, email: String, password: String) -> Result<(ProfileDTO, Session), ServerError<String>>;
}

#[async_trait]
pub trait ProfileServiceTrait: Send + Sync {
    async fn find_profile_by_id(&self, profile_id: IdType) -> Result<Profile, ServerError<String>>;
    async fn update_profile_by_id(&self, profile_id: IdType, display_name: Option<String>, bio: Option<String>, banner: Option<Bytes>, profile_picture: Option<Bytes>) -> Result<(), ServerError<String>>;
    async fn get_total_profiles_count(&self) -> Result<IdType, ServerError<String>>;
}

#[async_trait]
pub trait FigureServiceTrait: Send + Sync {
    async fn find_figure_by_id(&self, figure_id: IdType) -> Result<FigureDTO, ServerError<String>>;
    async fn find_figures_starting_from_id_with_profile_id(&self, figure_id: Option<IdType>, profile_id: Option<IdType>, limit: i32) -> Result<Vec<FigureDTO>, ServerError<String>>;
    async fn create(&self, title: String, description: Option<String>, image: Bytes, width: u32, height: u32, profile_id: IdType) -> Result<Figure, ServerError<String>>;
    async fn get_total_figures_by_profile(&self, figure_id: IdType) -> Result<IdType, ServerError<String>>;
    async fn get_total_figures_count(&self) -> Result<IdType, ServerError<String>>;
}