use std::sync::{Arc, Mutex};
use crate::entities::profile::Profile;
use crate::entities::user::User;
use crate::services::user_service::{hash_password, UserService, UserServiceTrait};
use crate::Session;
use crate::tests::mock_repositories::mock_profile_repository::MockProfileRepository;
use crate::tests::mock_repositories::mock_session_repository::MockSessionRepository;
use crate::tests::mock_repositories::mock_transaction::MockTransactionCreator;
use crate::tests::mock_repositories::mock_user_repository::MockUserRepository;

#[tokio::test]
pub async fn test_signup_user_happy_flow() {
    let users = Arc::new(Mutex::new(Vec::new()));
    let profiles = Arc::new(Mutex::new(Vec::new()));
    let sessions = Arc::new(Mutex::new(Vec::new()));
    let user_repository = MockUserRepository::new(users.clone());
    let profile_repository = MockProfileRepository::new(profiles.clone());
    let session_repository = MockSessionRepository::new(sessions);
    let mock_trans_creator = MockTransactionCreator::new();

    let user_service = UserService::new(mock_trans_creator, user_repository, profile_repository, session_repository);

    let result = user_service.signup_user("test@test.test".to_string(), "test1234".to_string(), "test".to_string()).await;
    let (user, profile, session) = result.unwrap();
    let expected_user = User {
        id: 0,
        email: "test@test.test".to_string(),
        password: user.clone().password, // Can't generate the same hash again due to salting
        role: "user".to_string(),
    };
    let expected_profile = Profile {
        id: 0,
        username: "test".to_string(),
        display_name: None,
        bio: None,
        banner: None,
        profile_picture: None,
        user_id: 0,
    };
    let expected_session = Session {
        id: "0".to_string(),
        _user_id: 0,
        profile_id: 0,
    };
    assert_eq!((expected_user, expected_profile, expected_session), (user, profile, session));
}