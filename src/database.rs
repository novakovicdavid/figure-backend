use std::fmt::{Debug};
use argon2::{PasswordHash, PasswordVerifier};
use async_trait::async_trait;
use sea_query::{Alias, Expr, InsertStatement, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use crate::entities::figure::{Figure, FigureDef};
use crate::entities::user::{User, UserAndProfile, UserDef};
use crate::entities::profile::ProfileDef;
use serde::{Deserialize};
use sqlx::{Column, Error, PgPool, Pool, Postgres, Row};
use zeroize::Zeroize;
use crate::auth_layer::{hash_password, is_email_valid, is_username_valid};
use crate::entities::profile::{ProfileDTO};
use crate::entities::user::{UserDTO};
use crate::server_errors::ServerError;
use futures::future::ready;
use futures::{TryFutureExt};
use crate::entities::types::Id;

#[derive(Deserialize)]
pub struct SignUpForm {
    pub email: String,
    pub password: String,
    pub username: String,
}

#[derive(Deserialize)]
pub struct SignInForm {
    pub email: String,
    pub password: String,
}

#[async_trait]
pub trait DatabaseFns: Sync + Send + Debug {
    async fn get_figure(&self, id: &i32) -> Result<Figure, ServerError<String>>;
    async fn signup_user(&self, signup: SignUpForm) -> Result<(UserDTO, ProfileDTO), ServerError<String>>;
    async fn authenticate_user_by_email(&self, email: String, password: String) -> Result<(UserDTO, ProfileDTO), ServerError<String>>;
}

pub type Database = Box<dyn DatabaseFns>;

#[derive(Debug)]
struct DatabaseImpl {
    db: Pool<Postgres>,
}

#[async_trait]
impl DatabaseFns for DatabaseImpl {
    async fn get_figure(&self, id: &i32) -> Result<Figure, ServerError<String>> {
        let (sql, values) = Query::select()
            .columns([FigureDef::Id, FigureDef::Title, FigureDef::Width, FigureDef::Height, FigureDef::ProfileId])
            .from(FigureDef::Table)
            .and_where(Expr::col(FigureDef::Id).eq(*id))
            .limit(1)
            .build_sqlx(PostgresQueryBuilder);
        match sqlx::query_as_with::<_, Figure, _>(&sql, values).fetch_one(&self.db).await {
            Ok(figure) => Ok(figure),

            Err(Error::RowNotFound) => Err(ServerError::ResourceNotFound),
            Err(e) => Err(ServerError::InternalError(e.to_string()))
        }
    }

    async fn signup_user(&self, mut signup: SignUpForm) -> Result<(UserDTO, ProfileDTO), ServerError<String>> {
        if !is_email_valid(&signup.email) {
            return Err(ServerError::InvalidEmail);
        }
        if !is_username_valid(&signup.username) {
            return Err(ServerError::InvalidUsername);
        }
        let password_hash_result = hash_password(&signup.password, true);
        signup.password.zeroize();
        let password_hash = match password_hash_result {
            Ok(hash) => hash,
            Err(e) => return Err(e)
        };

        // Create User and Profile in database and return these
        let transaction_result = self.db.begin().await;
        let mut transaction = match transaction_result {
            Ok(transaction) => transaction,
            Err(e) => {
                return Err(ServerError::InternalError(e.to_string()));
            }
        };

        // Create a user, dealing with any value parsing & database errors
        let user_id_result = ready(Query::insert()
            .into_table(UserDef::Table)
            .columns([UserDef::Email, UserDef::Password, UserDef::Role])
            .returning(Query::returning().column(UserDef::Id))
            .values([signup.email.to_lowercase().into(), password_hash.into(), "user".into()])
            .map_err(|e| ServerError::InternalError(e.to_string())))
            .and_then(|statement: &mut InsertStatement| async {
                // Build and execute query
                let (sql, values) = statement.build_sqlx(PostgresQueryBuilder);
                sqlx::query_with(&sql, values)
                    .fetch_one(&mut transaction)
                    .await
                    .map(|row| row.get::<Id, _>(0))
                    .map_err(|e| ServerError::InternalError(e.to_string()))
            }).await;

        let user_id = match user_id_result {
            Ok(user_id) => user_id,
            Err(e) => {
                return Err(e);
            }
        };

        let profile_id_result = ready(Query::insert()
            .into_table(ProfileDef::Table)
            .columns([ProfileDef::Username, ProfileDef::UserId])
            .returning(Query::returning().column(UserDef::Id))
            .values([signup.username.to_lowercase().into(), user_id.into()])
            .map_err(|e| ServerError::InternalError(e.to_string())))
            .and_then(|statement: &mut InsertStatement| async {
                // Build and execute query
                let (sql, values) = statement.build_sqlx(PostgresQueryBuilder);
                sqlx::query_with(&sql, values)
                    .fetch_one(&mut transaction)
                    .await
                    .map(|row| row.get::<Id, _>(0))
                    .map_err(|e| ServerError::InternalError(e.to_string()))
            }).await;
        let profile_id = match profile_id_result {
            Ok(profile_id) => profile_id,
            Err(e) => {
                return Err(e);
            }
        };

        let (sql, values) = Query::select()
            .expr(Expr::asterisk())
            .from(UserDef::Table)
            .build_sqlx(PostgresQueryBuilder);
        let query = sqlx::query_as_with::<_, User, _>(&sql, values)
            .fetch_one(&mut transaction).await;
        match query {
            Ok(user) => {
                println!("{:?}", user);
            }
            Err(e) => {
                println!("{}", e);
            }
        }

        match transaction.commit().await {
            Ok(()) => Ok((
                UserDTO {
                    email: signup.email,
                    role: "user".to_string(),
                    id: user_id,
                },
                ProfileDTO {
                    id: profile_id,
                    username: signup.username,
                    display_name: None,
                }
            )),
            Err(e) => Err(ServerError::InternalError(e.to_string()))
        }
    }

    async fn authenticate_user_by_email(&self, email: String, password: String) -> Result<(UserDTO, ProfileDTO), ServerError<String>> {
        let (sql, values) = Query::select()
            .expr_as(Expr::col((UserDef::Table, UserDef::Id)), Alias::new("user_id"))
            .columns([UserDef::Email, UserDef::Password, UserDef::Role])
            .column((ProfileDef::Table, ProfileDef::Id))
            .columns([ProfileDef::Username, ProfileDef::DisplayName])
            .from(UserDef::Table)
            .inner_join(ProfileDef::Table, Expr::col((UserDef::Table, UserDef::Id)).equals((ProfileDef::Table, ProfileDef::UserId)))
            .and_where(Expr::col(UserDef::Email).eq(email.to_lowercase()))
            .build_sqlx(PostgresQueryBuilder);
        let result = sqlx::query_as_with::<_, UserAndProfile, _>(&sql, values)
            .fetch_all(&self.db)
            .await;

        match result {
            Ok(pwu) => {
                println!("{:?}", pwu);
            }
            Err(e) => {
                println!("{}", e);
            }
        }

        // match result {
        //     Ok(vec) => {
        //         println!("{}", vec.len());
        //         for row in vec {
        //             row.get()
        //     }
        //     Err(e) => {
        //         println!("{}", e);
        //     }
        // }


        Err(ServerError::InvalidEmail)

        //     let user_result = UserEntity::find()
        //         .filter(user::Column::Email.eq(email.to_lowercase()))
        //         .find_also_related(ProfileEntity)
        //         .one(&self.db)
        //         .await;
        //     let user_option = match user_result {
        //         Ok(user_option) => user_option,
        //         Err(error) => {
        //             error!("{}", error);
        //             return Err(ServerError::InternalError);
        //         }
        //     };
        //
        //     let found_user = match user_option {
        //         Some(found_user) => found_user,
        //         None => return Err(ServerError::UserWithEmailNotFound)
        //     };
        //
        //     let parsed_hash_result = PasswordHash::new(&found_user.0.password);
        //     let parsed_hash = match parsed_hash_result {
        //         Ok(hash) => hash,
        //         Err(e) => {
        //             error!("{}", e);
        //             return Err(ServerError::InternalError);
        //         }
        //     };
        //
        //     let password_verification = Argon2::default().verify_password(password.as_bytes(), &parsed_hash);
        //     if password_verification.is_ok() {
        //         if let Some(profile) = found_user.1 {
        //             Ok((
        //                 UserDTO {
        //                     email: found_user.0.email,
        //                     role: found_user.0.role,
        //                     id: found_user.0.id,
        //                 },
        //                 ProfileDTO {
        //                     id: profile.id,
        //                     username: profile.username,
        //                     display_name: profile.display_name,
        //                 }
        //             ))
        //         }
        //         // If no profile associated with user is found
        //         else {
        //             return Err(ServerError::InternalError);
        //         }
        //     } else {
        //         Err(ServerError::WrongPassword)
        //     }
        // }
    }
}

pub async fn get_database_connection(database_url: String) -> Database {
    let db = PgPool::connect(&database_url).await.unwrap();
    Box::new(DatabaseImpl {
        db
    })
}