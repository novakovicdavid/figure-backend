use crate::repositories::traits::{FigureRepositoryTrait, ProfileRepositoryTrait, SessionRepositoryTrait, TransactionCreator, TransactionTrait, UserRepositoryTrait};
use crate::services::traits::{FigureServiceTrait, ProfileServiceTrait, UserServiceTrait};

pub struct Context<T: TransactionTrait> {
    pub service_context: ServiceContext,
    pub repository_context: RepositoryContext<T>,
}

impl<T: TransactionTrait> Context<T> {
    pub fn new(service_context: ServiceContext, repository_context: RepositoryContext<T>) -> Context<T> {
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
    pub fn new(user_service: Box<dyn UserServiceTrait>, profile_service: Box<dyn ProfileServiceTrait>, figure_service: Box<dyn FigureServiceTrait>)
               -> ServiceContext {
        ServiceContext {
            user_service,
            profile_service,
            figure_service,
        }
    }
}

pub struct RepositoryContext<T: TransactionTrait> {
    user_repository: Box<dyn UserRepositoryTrait<T>>,
    profile_repository: Box<dyn ProfileRepositoryTrait<T>>,
    figure_repository: Box<dyn FigureRepositoryTrait<T>>,
    pub session_repository: Box<dyn SessionRepositoryTrait>,
    transaction_starter: Box<dyn TransactionCreator<T>>,
}

impl<T: TransactionTrait> RepositoryContext<T> {
    pub fn new(user_repository: Box<dyn UserRepositoryTrait<T>>,
               profile_repository: Box<dyn ProfileRepositoryTrait<T>>,
               figure_repository: Box<dyn FigureRepositoryTrait<T>>,
               session_repository: Box<dyn SessionRepositoryTrait>,
               transaction_starter: Box<dyn TransactionCreator<T>>)
               -> RepositoryContext<T>
    {
        RepositoryContext {
            user_repository,
            profile_repository,
            figure_repository,
            session_repository,
            transaction_starter,
        }
    }
}