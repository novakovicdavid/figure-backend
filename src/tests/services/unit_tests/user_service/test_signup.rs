use crate::entities::dtos::profile_dto::ProfileDTO;
use crate::entities::dtos::session_dtos::Session;
use crate::entities::user::User;
use crate::repositories::traits::UserRepositoryTrait;
use crate::server_errors::ServerError;
use crate::services::traits::UserServiceTrait;
use crate::services::user_service::UserService;
use crate::tests::mocks::repositories::mock_profile_repository::MockProfileRepository;
use crate::tests::mocks::repositories::mock_session_repository::MockSessionRepository;
use crate::tests::mocks::repositories::mock_transaction::MockTransactionCreator;
use crate::tests::mocks::repositories::mock_user_repository::MockUserRepository;

#[tokio::test]
pub async fn signup() {
    let user_repository = MockUserRepository::new();
    let profile_repository = MockProfileRepository::new();
    let session_repository = MockSessionRepository::new();
    let transaction_creator = MockTransactionCreator::new();

    let user_service = UserService::new(transaction_creator, user_repository.clone(), profile_repository, session_repository);

    let result = user_service.signup_user("test@test.test".to_string(), "test1234".to_string(), "test".to_string()).await;
    let (profile, session) = result.unwrap();
    let saved_user = user_repository.find_one_by_id(None, 0).await.unwrap();
    let expected_password = saved_user.password.clone();
    let expected_user = User {
        id: 0,
        email: "test@test.test".to_string(),
        password: expected_password, // Can't generate the same hash again due to salting
        role: "user".to_string(),
    };
    let expected_profile = ProfileDTO {
        id: 0,
        username: "test".to_string(),
        display_name: None,
    };
    let expected_session = Session::new(
        session.get_id(),
        0,
        0,
        session.get_time_until_expiration(),
    );
    assert_eq!((saved_user, profile, session), (expected_user, expected_profile, expected_session));
}

#[tokio::test]
pub async fn signup_password_too_short() {
    let user_repository = MockUserRepository::new();
    let profile_repository = MockProfileRepository::new();
    let session_repository = MockSessionRepository::new();
    let transaction_creator = MockTransactionCreator::new();

    let user_service = UserService::new(transaction_creator, user_repository.clone(), profile_repository, session_repository);

    let signup_result = user_service.signup_user("test@test.test".to_string(), "1234567".to_string(), "test".to_string()).await;
    let saved_user = user_repository.find_one_by_id(None, 0).await;

    assert_eq!(signup_result, Err(ServerError::PasswordTooShort));
    // User shouldn't exist as the password given was too short
    assert!(saved_user.is_err());
}

#[tokio::test]
pub async fn password_too_long() {
    let user_repository = MockUserRepository::new();
    let profile_repository = MockProfileRepository::new();
    let session_repository = MockSessionRepository::new();
    let transaction_creator = MockTransactionCreator::new();

    let user_service = UserService::new(transaction_creator, user_repository.clone(), profile_repository, session_repository);

    let signup_result = user_service.signup_user("test@test.test".to_string(), "1111111111111111111111111111111111111111111111111111111111111".to_string(), "test".to_string()).await;
    let saved_user = user_repository.find_one_by_id(None, 0).await;

    assert_eq!(signup_result, Err(ServerError::PasswordTooLong));
    assert!(saved_user.is_err());
}

#[tokio::test]
pub async fn signup_invalid_email() {
    let user_repository = MockUserRepository::new();
    let profile_repository = MockProfileRepository::new();
    let session_repository = MockSessionRepository::new();
    let transaction_creator = MockTransactionCreator::new();

    let user_service = UserService::new(transaction_creator, user_repository.clone(), profile_repository, session_repository);

    // Missing @
    let signup_result = user_service.signup_user("testtest.test".to_string(), "1234567".to_string(), "test".to_string()).await;
    let saved_user = user_repository.find_one_by_id(None, 0).await;

    assert_eq!(signup_result, Err(ServerError::InvalidEmail));
    assert!(saved_user.is_err());

    // Missing tld
    let signup_result = user_service.signup_user("test@test".to_string(), "1234567".to_string(), "test".to_string()).await;
    let saved_user = user_repository.find_one_by_id(None, 0).await;

    assert_eq!(signup_result, Err(ServerError::InvalidEmail));
    assert!(saved_user.is_err());

    // Missing email username
    let signup_result = user_service.signup_user("@test.test".to_string(), "1234567".to_string(), "test".to_string()).await;
    let saved_user = user_repository.find_one_by_id(None, 0).await;

    assert_eq!(signup_result, Err(ServerError::InvalidEmail));
    assert!(saved_user.is_err());

    // Only @
    let signup_result = user_service.signup_user("@".to_string(), "1234567".to_string(), "test".to_string()).await;
    let saved_user = user_repository.find_one_by_id(None, 0).await;

    assert_eq!(signup_result, Err(ServerError::InvalidEmail));
    assert!(saved_user.is_err());

    // Empty email
    let signup_result = user_service.signup_user("".to_string(), "1234567".to_string(), "test".to_string()).await;
    let saved_user = user_repository.find_one_by_id(None, 0).await;

    assert_eq!(signup_result, Err(ServerError::InvalidEmail));
    assert!(saved_user.is_err());

    // Too long
    let signup_result = user_service.signup_user("1234567890123456789012345678901234567890123456789012345678901".to_string(), "1234567".to_string(), "test".to_string()).await;
    let saved_user = user_repository.find_one_by_id(None, 0).await;

    assert_eq!(signup_result, Err(ServerError::InvalidEmail));
    assert!(saved_user.is_err());
}

#[tokio::test]
pub async fn signup_invalid_username() {
    let user_repository = MockUserRepository::new();
    let profile_repository = MockProfileRepository::new();
    let session_repository = MockSessionRepository::new();
    let transaction_creator = MockTransactionCreator::new();

    let user_service = UserService::new(transaction_creator, user_repository.clone(), profile_repository, session_repository);

    let signup_result = user_service.signup_user("test@test.test".to_string(), "1234567".to_string(), "".to_string()).await;
    let saved_user = user_repository.find_one_by_id(None, 0).await;

    assert_eq!(signup_result, Err(ServerError::InvalidUsername));
    assert!(saved_user.is_err());
}
