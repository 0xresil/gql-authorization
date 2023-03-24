use juniper::{
    graphql_value, EmptyMutation, EmptySubscription, FieldError, FieldResult, GraphQLObject,
    RootNode,
};
use serde::{Deserialize, Serialize};

use crate::auth::AuthInfo;
#[derive(GraphQLObject, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
}

pub struct QueryRoot;

#[juniper::graphql_object(Context = AuthInfo)]
impl QueryRoot {
    fn user(context: &AuthInfo, id: String) -> FieldResult<User> {
        println!("auth {:?}", context);
        if context.token == "test_token".to_string() {
            Ok(User {
                id,
                username: "demo_user".to_string(),
            })
        } else {
            Err(FieldError::new(
                "The Auth token is illegal",
                graphql_value!({ "bad_request": "Connection refused" }),
            ))
        }
    }
}

pub type Schema =
    RootNode<'static, QueryRoot, EmptyMutation<AuthInfo>, EmptySubscription<AuthInfo>>;

pub fn create_schema() -> Schema {
    Schema::new(
        QueryRoot {},
        EmptyMutation::<AuthInfo>::new(),
        EmptySubscription::<AuthInfo>::new(),
    )
}
