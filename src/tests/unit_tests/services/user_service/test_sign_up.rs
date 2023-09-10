use crate::domain::models::profile::Profile;
use crate::entities::dtos::profile_dto::ProfileDTO;
use crate::entities::dtos::session_dtos::Session;
use crate::domain::models::user::User;
use crate::repositories::traits::{ProfileRepositoryTrait, UserRepositoryTrait};
use crate::server_errors::ServerError;
use crate::services::traits::UserServiceTrait;
use crate::tests::unit_tests::services::user_service::helpers::create_user_service_with_mocks;

#[tokio::test]
pub async fn sign_up() {
    let (user_service, mocks) = create_user_service_with_mocks();

    let result = user_service.sign_up("test@test.test", "test1234", "test").await;

    let (profile_dto, session) = result.unwrap();

    let saved_user = mocks.user_repository.find_by_id(None, 0).await.unwrap();

    let saved_profile = mocks.profile_repository.find_by_id(None, 0).await.unwrap();

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
pub async fn password_too_short() {
    let (user_service, mocks) = create_user_service_with_mocks();

    let signup_result = user_service.sign_up("test@test.test", "1234567", "test").await;

    let saved_user = mocks.user_repository.find_by_id(None, 0).await;

    assert_eq!(signup_result, Err(ServerError::PasswordTooShort));
    assert_eq!(saved_user, Err(ServerError::ResourceNotFound));
}

#[tokio::test]
pub async fn password_too_long() {
    let (user_service, mocks) = create_user_service_with_mocks();

    let signup_result = user_service.sign_up("test@test.test", "111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111", "test").await;

    let saved_user = mocks.user_repository.find_by_id(None, 0).await;

    assert_eq!(signup_result, Err(ServerError::PasswordTooLong));
    assert_eq!(saved_user, Err(ServerError::ResourceNotFound));
}

#[tokio::test]
pub async fn missing_at_symbol_in_email() {
    let (user_service, mocks) = create_user_service_with_mocks();

    let signup_result = user_service.sign_up("testtest.test", "12345678", "test").await;

    let saved_user = mocks.user_repository.find_by_id(None, 0).await;

    assert_eq!(signup_result, Err(ServerError::InvalidEmail));
    assert_eq!(saved_user, Err(ServerError::ResourceNotFound));
}

#[tokio::test]
pub async fn missing_tld_in_email() {
    let (user_service, mocks) = create_user_service_with_mocks();

    let signup_result = user_service.sign_up("test@test", "12345678", "test").await;

    let saved_user = mocks.user_repository.find_by_id(None, 0).await;

    assert_eq!(signup_result, Err(ServerError::InvalidEmail));
    assert_eq!(saved_user, Err(ServerError::ResourceNotFound));
}

#[tokio::test]
pub async fn missing_username_in_email() {
    let (user_service, mocks) = create_user_service_with_mocks();

    let signup_result = user_service.sign_up("@test.test", "12345678", "test").await;

    let saved_user = mocks.user_repository.find_by_id(None, 0).await;

    assert_eq!(signup_result, Err(ServerError::InvalidEmail));
    assert_eq!(saved_user, Err(ServerError::ResourceNotFound));
}

#[tokio::test]
pub async fn missing_username_and_domain_in_email() {
    let (user_service, mocks) = create_user_service_with_mocks();

    let signup_result = user_service.sign_up("@", "12345678", "test").await;

    let saved_user = mocks.user_repository.find_by_id(None, 0).await;

    assert_eq!(signup_result, Err(ServerError::InvalidEmail));
    assert_eq!(saved_user, Err(ServerError::ResourceNotFound));
}

#[tokio::test]
pub async fn empty_email() {
    let (user_service, mocks) = create_user_service_with_mocks();

    let signup_result = user_service.sign_up("", "12345678", "test").await;

    let saved_user = mocks.user_repository.find_by_id(None, 0).await;

    assert_eq!(signup_result, Err(ServerError::InvalidEmail));
    assert_eq!(saved_user, Err(ServerError::ResourceNotFound));
}

#[tokio::test]
pub async fn email_too_long() {
    let (user_service, mocks) = create_user_service_with_mocks();

    let signup_result = user_service.sign_up("1234567890123456789012345678901234567890123456789012345678901", "12345678", "test").await;

    let saved_user = mocks.user_repository.find_by_id(None, 0).await;

    assert_eq!(signup_result, Err(ServerError::InvalidEmail));
    assert_eq!(saved_user, Err(ServerError::ResourceNotFound));
}

#[tokio::test]
pub async fn invalid_username() {
    let (user_service, mocks) = create_user_service_with_mocks();

    let signup_result = user_service.sign_up("test@test.test", "12345678", "").await;

    let saved_user = mocks.user_repository.find_by_id(None, 0).await;

    assert_eq!(signup_result, Err(ServerError::InvalidUsername));
    assert_eq!(saved_user, Err(ServerError::ResourceNotFound));
}
