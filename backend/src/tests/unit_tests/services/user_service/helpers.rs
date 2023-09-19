use crate::services::user_service::UserService;
use crate::tests::unit_tests::mocks::repositories::mock_profile_repository::MockProfileRepository;
use crate::tests::unit_tests::mocks::repositories::mock_session_repository::MockSessionRepository;
use crate::tests::unit_tests::mocks::repositories::mock_transaction::{MockTransaction, MockTransactionManager};
use crate::tests::unit_tests::mocks::repositories::mock_user_repository::MockUserRepository;
use crate::tests::unit_tests::mocks::utilities::secure_rand_generator::FakeRandomGenerator;

pub struct UserServiceMocks {
    pub user_repository: MockUserRepository,
    pub profile_repository: MockProfileRepository,
    pub session_repository: MockSessionRepository,
}

pub fn create_user_service_with_mocks() -> (UserService<MockTransactionManager, MockTransaction, MockUserRepository, MockProfileRepository, MockSessionRepository, FakeRandomGenerator>, UserServiceMocks) {
    let user_repository = MockUserRepository::new();
    let profile_repository = MockProfileRepository::new();
    let session_repository = MockSessionRepository::new();
    let transaction_manager = MockTransactionManager::new();
    let random_number_generator = FakeRandomGenerator::new();

    let user_service = UserService::new(
        transaction_manager,
        user_repository.clone(),
        profile_repository.clone(),
        session_repository.clone(),
        random_number_generator);

    (
        user_service,
        UserServiceMocks {
            user_repository,
            profile_repository,
            session_repository,
        }
    )
}