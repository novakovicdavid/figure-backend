// use crate::domain::models::user::User;
// use crate::repositories::traits::UserRepositoryTrait;
// use crate::services::traits::UserServiceTrait;
// use crate::services::user_service::hash_password;
// use crate::tests::services::unit_tests::user_service::helpers::create_user_service_with_mocks;
//
// #[tokio::test]
// pub async fn sign_in() {
//     let (user_service, mocks) = create_user_service_with_mocks();
//
//     let user = User::new(0, "test@test.test".to_string(), hash_password("password").unwrap(),
//                          "user".to_string()).unwrap();
//
//     mocks.user_repository
//         .create(None, user)
//         .await
//         .unwrap();
//
//     let sign_up_result = user_service.sign_in("test@test.test", "password").await.unwrap();
//
//
// }