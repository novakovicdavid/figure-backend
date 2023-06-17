pub trait ContextTrait<SC, RC> {
    type Context;
    fn get_context(&self) -> &Self::Context;
}

impl<SC, RC> ContextTrait<SC, RC> for Context<SC, RC> {
    type Context = Context<SC, RC>;

    fn get_context(&self) -> &Self::Context {
        self
    }
}

pub struct Context<SC, RC> {
    pub service_context: SC,
    pub repository_context: RC,
}

impl<SC, RC> Context<SC, RC> {
    pub fn new(service_context: SC, repository_context: RC) -> Context<SC, RC> {
        Context {
            service_context,
            repository_context,
        }
    }
}

pub struct ServiceContext<US, PS, FS> {
    pub user_service: US,
    pub profile_service: PS,
    pub figure_service: FS,
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

pub trait ServiceContextTrait<US, PS, FS> {
    type ServiceContext;
    fn get_service_context(&self) -> &Self::ServiceContext;
}

impl<US, PS, FS> ServiceContextTrait<US, PS, FS> for ServiceContext<US, PS, FS> {
    type ServiceContext = ServiceContext<US, PS, FS>;

    fn get_service_context(&self) -> &Self::ServiceContext {
        self
    }
}

pub struct RepositoryContext<UR, PR, FR, SR> {
    user_repository: UR,
    profile_repository: PR,
    figure_repository: FR,
    pub session_repository: SR,
}

impl<UR, PR, FR, SR> RepositoryContext<UR, PR, FR, SR> {
    pub fn new(
        user_repository: UR,
        profile_repository: PR,
        figure_repository: FR,
        session_repository: SR) -> RepositoryContext<UR, PR, FR, SR> {
        RepositoryContext {
            user_repository,
            profile_repository,
            figure_repository,
            session_repository,
        }
    }
}

pub trait RepositoryContextTrait<UR, PR, FR, SR> {
    type RepositoryContext;
    fn get_repository_context(&self) -> &Self::RepositoryContext;
}

impl<UR, PR, FR, SR> RepositoryContextTrait<UR, PR, FR, SR> for RepositoryContext<UR, PR, FR, SR> {
    type RepositoryContext = RepositoryContext<UR, PR, FR, SR>;

    fn get_repository_context(&self) -> &Self::RepositoryContext {
        self
    }
}