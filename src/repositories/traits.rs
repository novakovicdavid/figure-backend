use async_trait::async_trait;
use crate::entities::dtos::figure_dto::FigureDTO;
use crate::entities::figure::Figure;
use crate::entities::profile::Profile;
use crate::entities::types::IdType;
use crate::entities::user::User;
use crate::server_errors::ServerError;
use crate::Session;

#[async_trait]
pub trait TransactionCreator<T: TransactionTrait>: Send + Sync {
    async fn create(&self) -> Result<T, ServerError<String>>;
}

#[async_trait]
pub trait TransactionTrait: Send + Sync {
    type Inner;
    async fn commit(self) -> Result<(), ServerError<String>>;
    fn inner(&mut self) -> &mut Self::Inner;
}

#[async_trait]
pub trait UserRepositoryTrait<T: TransactionTrait>: Send + Sync + Clone {
    async fn create(&self, transaction: Option<&mut T>, email: String, password_hash: String) -> Result<User, ServerError<String>>;
    async fn get_user_by_email(&self, transaction: Option<&mut T>, email: String) -> Result<User, ServerError<String>>;
}

#[async_trait]
pub trait ProfileRepositoryTrait<T: TransactionTrait>: Send + Sync + Clone {
    async fn create(&self, transaction: Option<&mut T>, username: String, user_id: IdType) -> Result<Profile, ServerError<String>>;
    async fn find_by_id(&self, transaction: Option<&mut T>, profile_id: IdType) -> Result<Profile, ServerError<String>>;
    async fn find_by_user_id(&self, transaction: Option<&mut T>, user_id: IdType) -> Result<Profile, ServerError<String>>;
    async fn update_profile_by_id(&self, transaction: Option<&mut T>, profile_id: IdType, display_name: Option<String>, bio: Option<String>, banner: Option<String>, profile_picture: Option<String>) -> Result<(), ServerError<String>>;
    async fn get_total_profiles_count(&self, transaction: Option<&mut T>) -> Result<IdType, ServerError<String>>;
}

#[async_trait]
pub trait FigureRepositoryTrait<T: TransactionTrait>: Send + Sync + Clone {
    async fn create(&self, transaction: Option<&mut T>, figure: Figure) -> Result<Figure, ServerError<String>>;
    async fn find_by_id(&self, transaction: Option<&mut T>, figure_id: IdType) -> Result<FigureDTO, ServerError<String>>;
    async fn find_starting_from_id_with_profile_id(&self, transaction: Option<&mut T>, figure_id: Option<IdType>, profile_id: Option<IdType>, limit: i32) -> Result<Vec<FigureDTO>, ServerError<String>>;
    async fn update_figure(&self, transaction: Option<&mut T>, figure: Figure) -> Result<(), ServerError<String>>;
    async fn delete_figure_by_id(&self, transaction: Option<&mut T>, figure_id: IdType) -> Result<(), ServerError<String>>;
    async fn count_by_profile_id(&self, transaction: Option<&mut T>, profile_id: IdType) -> Result<IdType, ServerError<String>>;
    async fn get_total_figures_count(&self, transaction: Option<&mut T>) -> Result<IdType, ServerError<String>>;
}

#[async_trait]
pub trait SessionRepositoryTrait: Send + Sync + Clone {
    async fn create(&self, user_id: IdType, profile_id: IdType, time_until_expiration: Option<usize>) -> Result<Session, ServerError<String>>;
    async fn find_by_id(&self, session_id: &str, time_until_expiration: Option<usize>) -> Result<Session, ServerError<String>>;
    async fn remove_by_id(&self, session_id: &str) -> Result<(), ServerError<String>>;
}