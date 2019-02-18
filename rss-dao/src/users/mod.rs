use super::DaoError;
use serde_json::Value;
use uuid::Uuid;

use super::*;
use serde_derive::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub user_name: String,
}

pub fn find_by_email(
    dao_conn: &DaoDbConnection,
    email: &str,
) -> Result<Option<(Uuid, User)>, DaoError> {
    let prepared_s = dao_conn
        .connection
        .prepare("SELECT id, data FROM users WHERE data->>'email' = $1")?;

    let result = prepared_s.query(&[&email])?;
    if result.is_empty() {
        Ok(None)
    } else {
        let row = &result.get(0);
        let uuid: Uuid = row.get(0);
        // let user: User = User::try_from(row.get::<_, Value>(1))?;
        let user: User = serde_json::from_value(row.get::<_, Value>(1)).map_err(DaoError::from)?;

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
        let conn: r2d2::PooledConnection<PostgresConnectionManager> =
            pool.new_connection().unwrap().into();
        let _ = conn.query("SELECT data FROM users", &[]).unwrap();
    }

    #[test]
    fn test_find_by_email() {
        let pool = create_test_pool();
        let conn = pool.new_connection().unwrap();
        let (_, user) = find_by_email(&conn, "user1@test.com").unwrap().unwrap();
        assert_eq!("user1", user.user_name);
    }
}
