pub mod users;
use actix::prelude::*;
use actix::{Actor, SyncContext};
use actix_web::AsyncResponder;
use futures::future::Future;
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;

use rss_dao::DaoPbConnPool;
use serde_derive::*;

pub struct AppState {
    pub executor: Addr<GraphQLExecutor>,
}

#[derive(Serialize, Deserialize)]
pub struct GraphQLData(GraphQLRequest);

impl Message for GraphQLData {
    type Result = Result<String, actix_web::Error>;
}

pub struct GraphQLExecutor {
    schema: std::sync::Arc<users::Schema>,
    dao_db_connection: users::DbConnection,
}

impl GraphQLExecutor {
    pub fn new(
        schema: std::sync::Arc<users::Schema>,
        dao_db_connection: DaoPbConnPool,
    ) -> GraphQLExecutor {
        GraphQLExecutor {
            schema: schema,
            dao_db_connection: users::DbConnection::from(
                dao_db_connection.new_connection().unwrap(),
            ),
        }
    }
}

impl Actor for GraphQLExecutor {
    type Context = SyncContext<Self>;
}

impl Handler<GraphQLData> for GraphQLExecutor {
    type Result = Result<String, actix_web::Error>;

    fn handle(&mut self, msg: GraphQLData, _: &mut Self::Context) -> Self::Result {
        let res = msg.0.execute(&self.schema, &self.dao_db_connection);
        let res_text = serde_json::to_string(&res)?;
        Ok(res_text)
    }
}

pub fn graphiql(
    _req: &actix_web::HttpRequest<AppState>,
) -> Result<actix_web::HttpResponse, actix_web::Error> {
    let html = graphiql_source("http://127.0.0.1:8080/graphql");
    Ok(actix_web::HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html))
}

pub fn graphql(
    (st, data): (actix_web::State<AppState>, actix_web::Json<GraphQLData>),
) -> actix_web::FutureResponse<actix_web::HttpResponse> {
    st.executor
        .send(data.0)
        .from_err()
        .and_then(|res| match res {
            Ok(user) => Ok(actix_web::HttpResponse::Ok()
                .content_type("application/json")
                .body(user)),
            Err(_) => Ok(actix_web::HttpResponse::InternalServerError().into()),
        })
        .responder()
}
