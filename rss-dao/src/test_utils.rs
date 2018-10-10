use super::*;

pub fn create_test_pool() -> r2d2::Pool<PostgresConnectionManager> {
    create_pool(
        "localhost",
        5433,
        "pgactix",
        Some(String::from("pgactix")),
        Some(String::from("pgactix")),
    )
    .unwrap()
}
