pub mod users;

use r2d2_postgres::{PostgresConnectionManager, TlsMode};
use std::error::Error;

pub struct DaoPbConnPool {
    db_connection_pool: r2d2::Pool<PostgresConnectionManager>,
}

impl Clone for DaoPbConnPool {
    fn clone(&self) -> DaoPbConnPool {
        DaoPbConnPool {
            db_connection_pool: self.db_connection_pool.clone(),
        }
    }
}

pub struct DaoDbConnection {
    connection: r2d2::PooledConnection<PostgresConnectionManager>,
}

impl From<r2d2::PooledConnection<PostgresConnectionManager>> for DaoDbConnection {
    fn from(pooled_conn: r2d2::PooledConnection<PostgresConnectionManager>) -> Self {
        Self {
            connection: pooled_conn,
        }
    }
}

impl Into<r2d2::PooledConnection<PostgresConnectionManager>> for DaoDbConnection {
    fn into(self: Self) -> r2d2::PooledConnection<PostgresConnectionManager> {
        self.connection
    }
}

impl DaoPbConnPool {
    pub fn new(
        host: &str,
        port: u16,
        db: &str,
        username: Option<String>,
        password: Option<String>,
    ) -> Result<Self, DaoError> {
        create_pool(host, port, db, username, password)
            .map(|pool| DaoPbConnPool {
                db_connection_pool: pool,
            })
            .map_err(DaoError::from)
    }

    pub fn new_connection(&self) -> Result<DaoDbConnection, DaoError> {
        self.db_connection_pool
            .clone()
            .get()
            .map(DaoDbConnection::from)
            .map_err(DaoError::from)
    }
}

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
    pub description: String,
    cause: Option<Box<Error>>,
}

impl DaoError {
    pub fn new(description: String, cause: Option<Box<Error>>) -> DaoError {
        DaoError { description, cause }
    }
}

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

impl From<r2d2::Error> for DaoError {
    fn from(error: r2d2::Error) -> Self {
        DaoError::new(String::from(error.description()), Some(Box::new(error)))
    }
}

#[cfg(test)]
mod test_utils;

#[cfg(test)]
mod tests {

    use super::test_utils::*;
    use super::users::find_by_email;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn connection_works() {
        let pool = create_test_pool();
        let _ = pool.new_connection();
    }

    #[test]
    fn find_user1() {
        let pool = create_test_pool();
        let conn = pool.new_connection().unwrap();
        let (_, user) = find_by_email(&conn, "user1@test.com").unwrap().unwrap();
        assert_eq!("user1", user.user_name);
    }

}
