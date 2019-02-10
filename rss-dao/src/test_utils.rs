use super::*;

pub fn create_test_pool() -> DaoPbConnPool {
    DaoPbConnPool::new(
        "localhost",
        5433,
        "pgactix",
        Some(String::from("pgactix")),
        Some(String::from("pgactix")),
    )
    .unwrap()
}
