#![feature(try_from)]

extern crate futures;
extern crate postgres;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate serde;
extern crate serde_json;
extern crate uuid;

#[macro_use]
extern crate serde_derive;
pub mod users;

use r2d2_postgres::{PostgresConnectionManager, TlsMode};
use std::error::Error;

pub fn create_pool(
    host: &str,
    port: u16,
    db: &str,
    username: Option<String>,
    password: Option<String>,
) -> Result<r2d2::Pool<PostgresConnectionManager>, r2d2::Error> {
    let connection = match (username, password) {
        (Some(username), Some(password)) => format!(
            "postgres://{}:{}@{}:{}/{}",
            username, password, host, port, db
        ),
        _ => format!("postgres://{}:{}/{}", host, port, db),
    };
    let manager = PostgresConnectionManager::new(connection, TlsMode::None).unwrap();
    r2d2::Pool::new(manager)
}

#[derive(Debug)]
pub struct DaoError {
    description: String,
    cause: Option<Box<Error>>,
}

impl DaoError {
    pub fn new(description: String, cause: Option<Box<Error>>) -> DaoError {
        DaoError { description, cause }
    }
}

// impl Error for DaoError {}

impl From<postgres::Error> for DaoError {
    fn from(error: postgres::Error) -> Self {
        DaoError::new(String::from(error.description()), Some(Box::new(error)))
    }
}

impl From<serde_json::Error> for DaoError {
    fn from(error: serde_json::Error) -> Self {
        DaoError::new(String::from(error.description()), Some(Box::new(error)))
    }
}

#[cfg(test)]
mod test_utils;

#[cfg(test)]
mod tests {

    use super::test_utils::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn connection_works() {
        let pool = create_test_pool();
        let _ = pool.clone();
    }

}
