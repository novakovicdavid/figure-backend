use crate::repositories::figure_repository::FigureRepositoryTrait;
use crate::repositories::profile_repository::ProfileRepositoryTrait;
use crate::repositories::session_repository::SessionRepositoryTrait;
use crate::repositories::user_repository::UserRepositoryTrait;
use crate::services::figure_service::FigureServiceTrait;
use crate::services::profile_service::ProfileServiceTrait;
use crate::services::user_service::UserServiceTrait;


pub struct Context {
    pub service_context: ServiceContext,
    pub repository_context: RepositoryContext,
}

impl Context {
    pub fn new(service_context: ServiceContext,
               repository_context: RepositoryContext)
               -> Context {
        Context {
            service_context,
            repository_context,
        }
    }
}

pub struct ServiceContext {
    pub user_service: Box<dyn UserServiceTrait>,
    pub profile_service: Box<dyn ProfileServiceTrait>,
    pub figure_service: Box<dyn FigureServiceTrait>,
}

impl ServiceContext {
    pub fn new(user_service: Box<dyn UserServiceTrait>,
               profile_service: Box<dyn ProfileServiceTrait>,
               figure_service: Box<dyn FigureServiceTrait>,)
               -> ServiceContext {
        ServiceContext {
            user_service,
            profile_service,
            figure_service,
        }
    }
}

pub struct RepositoryContext {
    user_repository: Box<dyn UserRepositoryTrait>,
    profile_repository: Box<dyn ProfileRepositoryTrait>,
    figure_repository: Box<dyn FigureRepositoryTrait>,
    pub session_repository: Box<dyn SessionRepositoryTrait>,
}

impl RepositoryContext {
    pub fn new(
        user_repository: Box<dyn UserRepositoryTrait>,
        profile_repository: Box<dyn ProfileRepositoryTrait>,
        figure_repository: Box<dyn FigureRepositoryTrait>,
        session_repository: Box<dyn SessionRepositoryTrait>) -> RepositoryContext {
        RepositoryContext {
            user_repository,
            profile_repository,
            figure_repository,
            session_repository,
        }
    }
}