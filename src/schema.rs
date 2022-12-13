pub(crate) struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
    async fn hello(&self) -> String {
        String::from("Hello GraphQL")
    }
}
