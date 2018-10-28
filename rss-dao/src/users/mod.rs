use super::DaoError;
use r2d2::PooledConnection;
use r2d2_postgres::PostgresConnectionManager;
use serde_json::Value;
use uuid::Uuid;

use std::convert::TryFrom;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub user_name: String,
}

impl TryFrom<&str> for User {
    type Error = DaoError;

    fn try_from(val: &str) -> Result<Self, Self::Error> {
        serde_json::from_str(val).map_err(DaoError::from)
    }
}

impl TryFrom<Value> for User {
    type Error = DaoError;

    fn try_from(val: Value) -> Result<Self, Self::Error> {
        serde_json::from_value(val).map_err(DaoError::from)
    }
}

pub fn find_by_email(
    conn: &PooledConnection<PostgresConnectionManager>,
    email: &'static str,
) -> Result<Option<(Uuid, User)>, DaoError> {
    let prepared_s = conn.prepare("SELECT id, data FROM users WHERE data->>'email' = $1")?;

    let result = prepared_s.query(&[&email])?;
    if result.is_empty() {
        Ok(None)
    } else {
        let row = &result.get(0);
        let uuid: Uuid = row.get(0);
        let user: User = User::try_from(row.get::<_, Value>(1))?;

        Ok(Some((uuid, user)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;
    #[test]
    fn test_users_tbl_exists() {
        let pool = create_test_pool();
        let pool_c = pool.clone();
        let conn = pool_c.get().unwrap();
        let _ = conn.query("SELECT data FROM users", &[]).unwrap();
    }

    #[test]
    fn test_find_by_email() {
        let pool = create_test_pool();
        let pool_c = pool.clone();
        let conn = pool_c.get().unwrap();
        let (_, user) = find_by_email(&conn, "user1@test.com").unwrap().unwrap();
        assert_eq!("user1", user.user_name);
    }
}
