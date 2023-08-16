use crate::domain::models::profile::Profile;
use crate::entities::dtos::profile_dto::ProfileDTO;
use crate::entities::dtos::session_dtos::Session;
use crate::domain::models::user::User;
use crate::repositories::traits::{ProfileRepositoryTrait, UserRepositoryTrait};
use crate::server_errors::ServerError;
use crate::services::traits::UserServiceTrait;
use crate::services::user_service::UserService;
use crate::tests::mocks::repositories::mock_profile_repository::MockProfileRepository;
use crate::tests::mocks::repositories::mock_session_repository::MockSessionRepository;
use crate::tests::mocks::repositories::mock_transaction::MockTransactionManager;
use crate::tests::mocks::repositories::mock_user_repository::MockUserRepository;
use crate::tests::mocks::utilities::secure_rand_generator::FakeRandomGenerator;

#[tokio::test]
pub async fn signup() {
    let user_repository = MockUserRepository::new();
    let profile_repository = MockProfileRepository::new();
    let session_repository = MockSessionRepository::new();
    let transaction_manager = MockTransactionManager::new();
    let random_number_generator = FakeRandomGenerator::new();

    let user_service = UserService::new(transaction_manager, user_repository.clone(), profile_repository.clone(), session_repository, random_number_generator);

    let result = user_service.signup_user("test@test.test".to_string(), "test1234".to_string(), "test".to_string()).await;
    let (profile_dto, session) = result.unwrap();
    let saved_user = user_repository.find_by_id(None, 0).await.unwrap();
    let saved_profile = profile_repository.find_by_id(None, 0).await.unwrap();
    let expected_password = saved_user.get_password();
    let expected_user = User::new(0, "test@test.test".to_string(), expected_password.to_string(), "user".to_string()).unwrap();
    let expected_profile = Profile::new(0, "test".to_string(), None, None, None, None, 0).unwrap();
    let expected_profile_dto = ProfileDTO {
        id: 0,
        username: "test".to_string(),
        display_name: None,
    };
    let expected_session = Session::new(
        0.to_string(),
        0,
        0,
        session.get_time_until_expiration(),
    );
    assert_eq!((saved_user, saved_profile, profile_dto, session), (expected_user, expected_profile, expected_profile_dto, expected_session));
}

#[tokio::test]
pub async fn signup_password_too_short() {
    let user_repository = MockUserRepository::new();
    let profile_repository = MockProfileRepository::new();
    let session_repository = MockSessionRepository::new();
    let transaction_manager = MockTransactionManager::new();
    let random_number_generator = FakeRandomGenerator::new();

    let user_service = UserService::new(transaction_manager, user_repository.clone(), profile_repository, session_repository, random_number_generator);

    let signup_result = user_service.signup_user("test@test.test".to_string(), "1234567".to_string(), "test".to_string()).await;
    let saved_user = user_repository.find_by_id(None, 0).await;

    assert_eq!(signup_result, Err(ServerError::PasswordTooShort));
    // User shouldn't exist as the password given was too short
    assert!(saved_user.is_err());
}

#[tokio::test]
pub async fn password_too_long() {
    let user_repository = MockUserRepository::new();
    let profile_repository = MockProfileRepository::new();
    let session_repository = MockSessionRepository::new();
    let transaction_manager = MockTransactionManager::new();
    let random_number_generator = FakeRandomGenerator::new();

    let user_service = UserService::new(transaction_manager, user_repository.clone(), profile_repository, session_repository, random_number_generator);

    let signup_result = user_service.signup_user("test@test.test".to_string(), "1111111111111111111111111111111111111111111111111111111111111".to_string(), "test".to_string()).await;
    let saved_user = user_repository.find_by_id(None, 0).await;

    assert_eq!(signup_result, Err(ServerError::PasswordTooLong));
    assert!(saved_user.is_err());
}

#[tokio::test]
pub async fn signup_invalid_email() {
    let user_repository = MockUserRepository::new();
    let profile_repository = MockProfileRepository::new();
    let session_repository = MockSessionRepository::new();
    let transaction_manager = MockTransactionManager::new();
    let random_number_generator = FakeRandomGenerator::new();

    let user_service = UserService::new(transaction_manager, user_repository.clone(), profile_repository, session_repository, random_number_generator);

    // Missing @
    let signup_result = user_service.signup_user("testtest.test".to_string(), "12345678".to_string(), "test".to_string()).await;
    let saved_user = user_repository.find_by_id(None, 0).await;

    assert_eq!(signup_result, Err(ServerError::InvalidEmail));
    assert_eq!(saved_user, Err(ServerError::ResourceNotFound));

    // Missing tld
    let signup_result = user_service.signup_user("test@test".to_string(), "12345678".to_string(), "test".to_string()).await;
    let saved_user = user_repository.find_by_id(None, 0).await;

    assert_eq!(signup_result, Err(ServerError::InvalidEmail));
    assert_eq!(saved_user, Err(ServerError::ResourceNotFound));

    // Missing email username
    let signup_result = user_service.signup_user("@test.test".to_string(), "12345678".to_string(), "test".to_string()).await;
    let saved_user = user_repository.find_by_id(None, 0).await;

    assert_eq!(signup_result, Err(ServerError::InvalidEmail));
    assert_eq!(saved_user, Err(ServerError::ResourceNotFound));

    // Only @
    let signup_result = user_service.signup_user("@".to_string(), "12345678".to_string(), "test".to_string()).await;
    let saved_user = user_repository.find_by_id(None, 0).await;

    assert_eq!(signup_result, Err(ServerError::InvalidEmail));
    assert_eq!(saved_user, Err(ServerError::ResourceNotFound));

    // Empty email
    let signup_result = user_service.signup_user("".to_string(), "12345678".to_string(), "test".to_string()).await;
    let saved_user = user_repository.find_by_id(None, 0).await;

    assert_eq!(signup_result, Err(ServerError::InvalidEmail));
    assert_eq!(saved_user, Err(ServerError::ResourceNotFound));

    // Too long
    let signup_result = user_service.signup_user("1234567890123456789012345678901234567890123456789012345678901".to_string(), "12345678".to_string(), "test".to_string()).await;
    let saved_user = user_repository.find_by_id(None, 0).await;

    assert_eq!(signup_result, Err(ServerError::InvalidEmail));
    assert_eq!(saved_user, Err(ServerError::ResourceNotFound));
}

#[tokio::test]
pub async fn signup_invalid_username() {
    let user_repository = MockUserRepository::new();
    let profile_repository = MockProfileRepository::new();
    let session_repository = MockSessionRepository::new();
    let transaction_manager = MockTransactionManager::new();
    let random_number_generator = FakeRandomGenerator::new();

    let user_service = UserService::new(transaction_manager, user_repository.clone(), profile_repository, session_repository, random_number_generator);

    let signup_result = user_service.signup_user("test@test.test".to_string(), "12345678".to_string(), "".to_string()).await;

    assert_eq!(signup_result, Err(ServerError::InvalidUsername));
}
