mod controllers;
use env_logger;
use rss_dao::DaoPbConnPool;
use std::sync::Arc;

fn create_pool() -> DaoPbConnPool {
    DaoPbConnPool::new(
        "localhost",
        5433,
        "pgactix",
        Some(String::from("pgactix")),
        Some(String::from("pgactix")),
    )
    .unwrap()
}

fn main() {
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let sys = actix::System::new("juniper-example");

    let schema = Arc::new(controllers::users::create_schema());
    let pool = create_pool();

    let addr = actix::SyncArbiter::start(3, move || {
        controllers::GraphQLExecutor::new(schema.clone(), pool.clone())
    });

    // Start http server
    actix_web::server::new(move || {
        actix_web::App::with_state(controllers::AppState {
            executor: addr.clone(),
        })
        // enable logger
        .middleware(actix_web::middleware::Logger::default())
        .resource("/graphql", |r| {
            r.method(actix_web::http::Method::POST)
                .with(controllers::graphql)
        })
        .resource("/graphiql", |r| {
            r.method(actix_web::http::Method::GET)
                .h(controllers::graphiql)
        })
    })
    .bind("127.0.0.1:8080")
    .unwrap()
    .start();

    println!("Started http server: 127.0.0.1:8080");
    let _ = sys.run();
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_r2d2_pool() {
        super::create_pool();
    }

}
