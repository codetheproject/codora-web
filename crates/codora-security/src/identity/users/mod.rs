use crate::identity::emails::EmailService;
use derive_more::Display;
use std::borrow::Cow;
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
#[error("{0}")]
pub enum Error {}

pub type Result<T, E = Error> = core::result::Result<T, E>;

pub trait UserService {
    async fn create_user(&self, create_user: &dto::CreateUser) -> Result<dto::User<String>>;
}

pub trait UserRepository {
    type Error;
    type ID: Into<String>;

    // crud: TODO
    // create other crud related api like delete, update, others when needed
    async fn get_user_by_id(&self, id: &Self::ID) -> Result<dto::User<Self::ID>, Self::Error>;
    async fn get_user_by_username(&self, username: &str) -> Result<dto::User<Self::ID>, Self::Error>;
    async fn create_user(&self, create_user: &dto::CreateUser) -> Result<dto::User<Self::ID>, Self::Error>;
}

/// UserOption configure all service needed by our UserService
///
/// This is very good as it allows flexible changes based on enviroment
/// User should be responsible providing implementation for this trait
pub trait UserOption {
    type Error: std::fmt::Debug;
    type Email: EmailService;
    type Repository: UserRepository;

    fn get_repository(&self) -> Result<Self::Repository, Self::Error>;
    fn get_email(&self) -> Result<Self::Email, Self::Error>;
}

#[derive(Debug, Clone)]
pub struct User<S> {
    user_option: S,
}

impl<S: UserOption> User<S> {
    pub fn new(user_option: S) -> Self {
        Self { user_option }
    }
}

impl<S> UserService for User<S>
where
    S: UserOption,
{
    async fn create_user(&self, create_user: &dto::CreateUser) -> Result<dto::User<String>> {
        todo!()
    }
}

mod dto {

    #[derive(Debug, Clone, PartialEq, PartialOrd)]
    pub(crate) struct User<I = String> {
        id: I,
        email: String,
        username: String,
        // created at and updated_at and other stuff needed
    }

    impl<I> User<I> {
        pub fn new(id: I, username: String, email: String) -> super::Result<Self> {
            // TODO: Fix this proper validation error
            todo!()
        }
    }

    #[derive(Debug, Clone)]
    pub struct CreateUser {
        email: String,
        username: String,
        password: String,
    }

    impl CreateUser {
        pub fn new(email: String, username: String, password: String) -> Self {
            Self { email, username, password }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::identity::emails::ConsoleEmail;
    use uuid::Uuid;

    // This adapter would definently be provided by the framework, this is just a template
    struct Postgres;

    impl UserRepository for Postgres {
        type Error = ();
        type ID = String;

        async fn get_user_by_id(&self, id: &Self::ID) -> Result<dto::User<Self::ID>, Self::Error> {
            todo!()
        }

        async fn get_user_by_username(&self, username: &str) -> Result<dto::User<Self::ID>, Self::Error> {
            todo!()
        }

        async fn create_user(&self, create_user: &dto::CreateUser) -> Result<dto::User<Self::ID>, Self::Error> {
            todo!()
        }
    }

    // User app is responsible to create all the config needed to create any of the services
    struct OurApp {}

    #[derive(Debug)]
    enum Error {}

    impl UserOption for OurApp {
        type Error = Error;
        type Email = ConsoleEmail;
        type Repository = Postgres;

        fn get_repository(&self) -> Result<Self::Repository, Self::Error> {
            todo!()
        }

        fn get_email(&self) -> Result<Self::Email, Self::Error> {
            todo!()
        }
    }

    #[tokio::test]
    async fn create_user() -> anyhow::Result<()> {
        tracing_subscriber::fmt::init();
        let user = User::new(OurApp {});

        Ok(())
    }
}
