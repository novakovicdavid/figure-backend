use async_closure::{async_closure_mut as cb, capture_lifetimes::AsyncFnMut};
use sqlx::{Pool, Postgres, Transaction};
use crate::entities::user::User;
use crate::server_errors::ServerError;
use sqlx::Row;

pub trait UserRepositoryTrait: Send + Sync + Clone {
    async fn start_transaction<'env, F, RES>(&self, repository: Self, f: F) -> Result<RES, ServerError<String>>
        where
            F: for<'any> AsyncFnMut<
                'env, (UserRepository, &'any Transaction<'any, Postgres>), Output=Result<RES, ServerError<String>>,
            >,
            RES: Send;
    async fn create<'a>(&self, transaction: Option<&'a Transaction<Postgres>>, email: String, password_hash: String, username: String) -> Result<(), ServerError<String>>;
}

pub struct UserRepository {
    db: Pool<Postgres>,
}

impl Clone for UserRepository {
    fn clone(&self) -> Self {
        Self {
            db: self.db.clone()
        }
    }
}

impl UserRepositoryTrait for UserRepository {
    async fn start_transaction<'env, F, RES>(&self, repository: Self, f: F) -> Result<RES, ServerError<String>>
        where
            F: for<'any> AsyncFnMut<
                'env, (UserRepository, &'any Transaction<'any, Postgres>), Output=Result<RES, ServerError<String>>,
            >,
            RES: Send {
        let transaction_result = self.db.begin().await;
        if let Ok(transaction) = transaction_result {
            let future_result = f.call_once((repository, &transaction)).await;
            if let Ok(result) = future_result {
                let commit_result = transaction.commit().await.map_err(|e| ServerError::InternalError(e.to_string()));
                if commit_result.is_ok() {
                    return Ok(result);
                }
                return Err(ServerError::TransactionFailed);
            }
            Err(ServerError::TransactionFailed)
        } else {
            Err(ServerError::TransactionFailed)
        }
    }

    async fn create<'a>(&self, transaction: Option<&'a Transaction<'_, Postgres>>, email: String, password_hash: String, username: String) -> Result<(), ServerError<String>> {
        let query = sqlx::query(r#"
            INSERT INTO users (email, password, role)
            VALUES ($1, $2, 'user')
            RETURNING id"#)
            .bind(email.to_lowercase())
            .bind(&password_hash);
        let query_result = query.fetch_one(&self.db).await;
        // let query_result = match transaction {
        //     Some(mut transaction) => query.fetch_one(transaction).await,
        //     None => query.fetch_one(&self.db).await
        // };

        return Ok(());
    }
}

trait UserServiceTrait: Send + Sync {
    async fn signup_user(&self, email: String, password: String, username: String) -> Result<(), ServerError<String>>;
}

struct UserService<T: UserRepositoryTrait + Send + Sync> {
    user_repository: T,
    db: AppDatabase,
}

struct Repo {
    db: AppDatabase,
}

impl<T: UserRepositoryTrait + Send + Sync> UserServiceTrait for UserService<T> {
    async fn signup_user(&self, email: String, password: String, username: String) -> Result<(), ServerError<String>> {
        let closure = cb!({}; async |repository: UserRepository, transaction: &Transaction<'_, Postgres>| -> Result<(), ServerError<String>> {
            repository.create(Some(transaction), "".to_string(), "".to_string(), "".to_string()).await;
            repository.create(Some(transaction), "".to_string(), "".to_string(), "".to_string()).await;

            Ok(())
        });
        // self.db.execute_transaction(closure).await
        self.user_repository.start_transaction(self.user_repository.clone(), closure).await
    }
}


impl Repo {
    async fn tryit(&self) {
        let closure = cb!({}; async |db: &AppDatabase, session: &mut DatabaseSession| -> Result<(), ServerError<String>> {
                let val = 5;
                db.insert_with_session(val, session).await
            });
        self.db.execute_transaction(closure).await;
    }
}

pub async fn main() {
    println!("Hello, world!");
    let db = AppDatabase(0);
    // let closure =
    //
    // let r = db
    //     .execute_transaction(closure)
    //     .await;
    let repo = Repo {
        db
    };

    let wow = repo.tryit();
    wow.await;

    // println!("{:?}", r);
}

struct AppDatabase(i32);

struct DatabaseSession(i32);

impl DatabaseSession {
    fn commit(&self) {}
    fn abort(&self) {}
}

impl AppDatabase {
    async fn execute_transaction<'env, F>(&self, mut f: F) -> Result<(), ServerError<String>>
        where
            F: for<'any> AsyncFnMut<
                'env,
                (&'any AppDatabase, &'any mut DatabaseSession),
                Output=Result<(), ServerError<String>>,
            >,
    {
        let mut session = DatabaseSession(0);
        let result = f.call_mut((self, &mut session)).await;
        if result.is_err() {
            session.abort();
            println!("rollback transaction here");
        } else {
            session.commit();
            println!("commit transaction here");
        }
        Ok(())
    }

    async fn insert_with_session(&self, val: i32, session: &mut DatabaseSession) -> Result<(), ServerError<String>> {
        println!("some dummy insert operation here");
        session.0 = val;
        Ok(())
    }
}