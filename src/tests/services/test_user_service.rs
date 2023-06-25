use std::sync::{Arc, Mutex};
use crate::services::user_service::UserService;
use crate::tests::mock_repositories::mock_profile_repository::MockProfileRepository;
use crate::tests::mock_repositories::mock_transaction::MockTransactionCreator;
use crate::tests::mock_repositories::mock_user_repository::MockUserRepository;

#[test]
pub fn test_create_user() {
    let users = Arc::new(Mutex::new(Vec::new()));
    let profiles = Arc::new(Mutex::new(Vec::new()));
    let user_repository = MockUserRepository::new(users.clone());
    let profile_repository = MockProfileRepository::new(profiles.clone());
    let mock_trans_creator = MockTransactionCreator::new();
    
    let user_service = UserService::new(mock_trans_creator, user_repository, profile_repository, )
}