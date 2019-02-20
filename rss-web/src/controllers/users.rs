use juniper::*;
use rss_dao::{users, DaoDbConnection,DaoError};

pub struct DbConnection(DaoDbConnection);

impl From<DbConnection> for DaoDbConnection {
    fn from(ctx: DbConnection) -> DaoDbConnection {
        ctx.0
    }
}

impl From<DaoDbConnection> for DbConnection {
    fn from(ctx: DaoDbConnection) -> DbConnection {
        DbConnection(ctx)
    }
}

struct ControllerError(DaoError);


impl From<ControllerError> for FieldError {
    fn from(c_err : ControllerError) -> FieldError {
        FieldError::new(
            c_err.0.description,
        graphql_value!({ "internal_error": "Internal server error" }))
    }
}

impl Context for DbConnection {}

pub struct QueryRoot;

pub struct JUser(users::User);

graphql_object!(JUser: () |&self| {
    field first_name() -> &str {
        self.0.first_name.as_str()
    }
    field last_name() -> &str{
        self.0.last_name.as_str()
    },
    field email() -> &str{
        self.0.email.as_str()
    },
    field user_name() -> &str{
        self.0.user_name.as_str()
    },
});

#[derive(GraphQLInputObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
struct NewUser {
    first_name: String,
    last_name: String,
    email: String,
    user_name: String,
    
}

graphql_object!(QueryRoot: DbConnection |&self| {
    field user(&executor, email: String) -> FieldResult<Option<JUser>> {
        let conn = &executor.context().0;

        match users::find_by_email(conn, &email[..]) {
            Ok(opt_result) => Ok(match opt_result {
                Some((_, user)) => Some(JUser(user)),
                _ => None
            }),
            Err(err) => Err(FieldError::from(ControllerError(err))) 
        }

    }
});

pub struct MutationRoot;

graphql_object!(MutationRoot: DbConnection |&self| {
    field createUser(&executor, new_user: NewUser) -> FieldResult<JUser> {
        Err(FieldError::new("Not implemented", graphql_value!({ "internal_error": "not implemented" })))
    }
});

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {})
}