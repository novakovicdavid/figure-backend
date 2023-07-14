use std::marker::PhantomData;
use crate::repositories::traits::{FigureRepositoryTrait, ProfileRepositoryTrait, SessionRepositoryTrait, TransactionCreatorTrait, TransactionTrait, UserRepositoryTrait};
use crate::services::traits::{FigureServiceTrait, ProfileServiceTrait, UserServiceTrait};

pub trait ContextTrait: Send + Sync {
    type ServiceContext: ServiceContextTrait;
    type RepositoryContext: RepositoryContextTrait;
    fn new(service_context: Self::ServiceContext, repository_context: Self::RepositoryContext) -> Self;
    fn service_context(&self) -> &Self::ServiceContext;
    fn repository_context(&self) -> &Self::RepositoryContext;
}

pub struct Context<SC, RC> {
    service_context: SC,
    repository_context: RC,
}

impl<SC: ServiceContextTrait, RC: RepositoryContextTrait> ContextTrait for Context<SC, RC> {
    type ServiceContext = SC;
    type RepositoryContext = RC;

    fn new(service_context: Self::ServiceContext, repository_context: Self::RepositoryContext) -> Self {
        Context {
            service_context,
            repository_context,
        }
    }

    fn service_context(&self) -> &Self::ServiceContext {
        &self.service_context
    }
    fn repository_context(&self) -> &Self::RepositoryContext {
        &self.repository_context
    }
}

pub trait ServiceContextTrait: Send + Sync {
    type UserService: UserServiceTrait;
    type ProfileService: ProfileServiceTrait;
    type FigureService: FigureServiceTrait;
    fn user_service(&self) -> &Self::UserService;
    fn profile_service(&self) -> &Self::ProfileService;
    fn figure_service(&self) -> &Self::FigureService;
}

pub struct ServiceContext<US, PS, FS> {
    user_service: US,
    profile_service: PS,
    figure_service: FS,
}

impl<US, PS, FS> ServiceContext<US, PS, FS> {
    pub fn new(user_service: US, profile_service: PS, figure_service: FS)
               -> ServiceContext<US, PS, FS> {
        ServiceContext {
            user_service,
            profile_service,
            figure_service,
        }
    }
}

impl<US, PS, FS> ServiceContextTrait for ServiceContext<US, PS, FS>
    where US: UserServiceTrait, PS: ProfileServiceTrait, FS: FigureServiceTrait {
    type UserService = US;
    type ProfileService = PS;
    type FigureService = FS;

    fn user_service(&self) -> &Self::UserService {
        &self.user_service
    }

    fn profile_service(&self) -> &Self::ProfileService {
        &self.profile_service
    }

    fn figure_service(&self) -> &Self::FigureService {
        &self.figure_service
    }
}

pub trait RepositoryContextTrait: Send + Sync {
    type Transaction: TransactionTrait;
    type TransactionCreator: TransactionCreatorTrait<Self::Transaction>;
    type UserRepository: UserRepositoryTrait<Self::Transaction>;
    type ProfileRepository: ProfileRepositoryTrait<Self::Transaction>;
    type FigureRepository: FigureRepositoryTrait<Self::Transaction>;
    type SessionRepository: SessionRepositoryTrait;

    fn user_repository(&self) -> &Self::UserRepository;
    fn profile_repository(&self) -> &Self::ProfileRepository;
    fn figure_repository(&self) -> &Self::FigureRepository;
    fn session_repository(&self) -> &Self::SessionRepository;
}

pub struct RepositoryContext<T, UR, PR, FR, SR, TS> {
    marker: PhantomData<T>,
    user_repository: UR,
    profile_repository: PR,
    figure_repository: FR,
    session_repository: SR,
    transaction_starter: TS,
}

impl<T, UR, PR, FR, SR, TS> RepositoryContext<T, UR, PR, FR, SR, TS> {
    pub fn new(
        user_repository: UR,
        profile_repository: PR,
        figure_repository: FR,
        session_repository: SR,
        transaction_starter: TS, ) -> RepositoryContext<T, UR, PR, FR, SR, TS> {
        RepositoryContext {
            marker: PhantomData::default(),
            user_repository,
            profile_repository,
            figure_repository,
            session_repository,
            transaction_starter,
        }
    }
}

impl<T, UR, PR, FR, SR, TS> RepositoryContextTrait for RepositoryContext<T, UR, PR, FR, SR, TS>
    where T: TransactionTrait, UR: UserRepositoryTrait<T>, PR: ProfileRepositoryTrait<T>,
          FR: FigureRepositoryTrait<T>, SR: SessionRepositoryTrait, TS: TransactionCreatorTrait<T> {
    type Transaction = T;
    type TransactionCreator = TS;
    type UserRepository = UR;
    type ProfileRepository = PR;
    type FigureRepository = FR;
    type SessionRepository = SR;

    fn user_repository(&self) -> &Self::UserRepository {
        &self.user_repository
    }

    fn profile_repository(&self) -> &Self::ProfileRepository {
        &self.profile_repository
    }

    fn figure_repository(&self) -> &Self::FigureRepository {
        &self.figure_repository
    }

    fn session_repository(&self) -> &Self::SessionRepository {
        &self.session_repository
    }
}