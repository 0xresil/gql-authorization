use juniper::{EmptyMutation, EmptySubscription, FieldResult, GraphQLObject, RootNode};

#[derive(GraphQLObject)]
pub struct User {
    pub id: String,
    pub username: String,
}

pub struct QueryRoot;

#[juniper::graphql_object]
impl QueryRoot {
    fn user(id: String) -> FieldResult<User> {
        Ok(User {
            id,
            username: "demo_user".to_string(),
        })
    }
}

pub type Schema = RootNode<'static, QueryRoot, EmptyMutation<()>, EmptySubscription<()>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, EmptyMutation::new(), EmptySubscription::new())
}
