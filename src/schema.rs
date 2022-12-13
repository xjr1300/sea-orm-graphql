use async_graphql::{ComplexObject, Context};
use sea_orm::{DatabaseConnection, DbErr, EntityTrait, ModelTrait};

use crate::entities::{prelude::*, *};

pub(crate) struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
    async fn hello(&self) -> String {
        String::from("Hello GraphQL")
    }

    async fn bakeries(&self, ctx: &Context<'_>) -> Result<Vec<bakery::Model>, DbErr> {
        let conn = ctx.data::<DatabaseConnection>().unwrap();

        Bakery::find().all(conn).await
    }

    async fn bakery(&self, ctx: &Context<'_>, id: i32) -> Result<Option<bakery::Model>, DbErr> {
        let conn = ctx.data::<DatabaseConnection>().unwrap();

        Bakery::find_by_id(id).one(conn).await
    }
}

#[ComplexObject]
impl bakery::Model {
    async fn chefs(&self, ctx: &Context<'_>) -> Result<Vec<chef::Model>, DbErr> {
        let conn = ctx.data::<DatabaseConnection>().unwrap();

        self.find_related(Chef).all(conn).await
    }
}
