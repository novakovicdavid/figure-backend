use std::any::Any;
use async_trait::async_trait;
use crate::entities::dtos::session_dtos::Session;
use crate::domain::models::figure::Figure;
use crate::domain::models::profile::Profile;
use crate::domain::models::types::IdType;
use crate::domain::models::user::User;
use crate::server_errors::ServerError;

#[async_trait]
pub trait TransactionManagerTrait<T: TransactionTrait>: Send + Sync {
    async fn create(&self) -> Result<T, ServerError>;
}

#[async_trait]
pub trait TransactionTrait: Send + Sync {
    type Inner;
    async fn commit(self) -> Result<(), ServerError>;
    fn inner(&mut self) -> &mut Self::Inner;
}

#[async_trait]
pub trait UserRepositoryTrait<T: TransactionTrait>: Send + Sync + Clone {
    async fn create(&self, transaction: Option<&mut T>, user: User) -> Result<User, ServerError>;
    async fn find_one_by_email(&self, transaction: Option<&mut T>, email: String) -> Result<User, ServerError>;
    async fn find_by_id(&self, transaction: Option<&mut T>, id: IdType) -> Result<User, ServerError>;
}

#[async_trait]
pub trait ProfileRepositoryTrait<T: TransactionTrait>: Send + Sync + Clone {
    async fn create(&self, transaction: Option<&mut T>, profile: Profile) -> Result<Profile, ServerError>;
    async fn find_by_id(&self, transaction: Option<&mut T>, profile_id: IdType) -> Result<Profile, ServerError>;
    async fn find_by_user_id(&self, transaction: Option<&mut T>, user_id: IdType) -> Result<Profile, ServerError>;
    async fn update_profile_by_id(&self, transaction: Option<&mut T>, profile_id: IdType, display_name: Option<String>, bio: Option<String>, banner: Option<String>, profile_picture: Option<String>) -> Result<(), ServerError>;
    async fn get_total_profiles_count(&self, transaction: Option<&mut T>) -> Result<IdType, ServerError>;
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

#[async_trait]
pub trait SessionRepositoryTrait: Send + Sync + Clone {
    async fn create(&self, session: Session) -> Result<Session, ServerError>;
    async fn find_by_id(&self, session_id: &str, time_until_expiration: Option<usize>) -> Result<Session, ServerError>;
    async fn remove_by_id(&self, session_id: &str) -> Result<(), ServerError>;
}