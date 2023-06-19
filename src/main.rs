use std::collections::HashMap;

use async_graphql::{
    dataloader::{DataLoader, Loader},
    http::GraphiQLSource,
    ComplexObject, Context, EmptyMutation, EmptySubscription, Object, Result, Schema, SimpleObject,
};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    async_trait,
    response::{Html, IntoResponse},
    routing::get,
    Extension, Router, Server,
};
use data::{authors_db, books_db};

mod data;

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Book {
    id: i32,
    name: String,
    published: i32,
    author_id: i32,
    author: Option<Author>,
}

#[derive(Clone, SimpleObject)]
pub struct Author {
    id: i32,
    name: String,
}

#[ComplexObject]
impl Book {
    pub async fn author_dataloaded(&self, ctx: &Context<'_>) -> Result<Option<Author>> {
        let dataloader = ctx.data_unchecked::<DataLoader<BatchAuthorById>>();
        let result = dataloader.load_one(self.author_id).await?;
        Ok(result)
    }
}

struct BatchAuthorById;

#[async_trait]
impl Loader<i32> for BatchAuthorById {
    type Value = Author;
    type Error = String;

    async fn load(&self, keys: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        let authors = authors_db()
            .iter()
            .filter(|a| keys.contains(&a.id))
            .map(|a| (a.id, a.clone()))
            .collect();
        Ok(authors)
    }
}

struct Query;

#[Object]
impl Query {
    pub async fn books_dataloaded(&self) -> Vec<Book> {
        books_db()
    }

    pub async fn books(&self) -> Vec<Book> {
        let books = books_db();
        let authors = authors_db();

        books
            .into_iter()
            .map(|b| {
                let author = match authors.iter().find(|a| a.id == b.author_id) {
                    Some(a) => Some(a.clone()),
                    None => None,
                };

                Book { author, ..b }
            })
            .collect()
    }
}

type MySchema = Schema<Query, EmptyMutation, EmptySubscription>;

async fn graphql_handler(schema: Extension<MySchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphiql_handler() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/graphql").finish())
}

#[tokio::main]
async fn main() {
    let schema = Schema::build(Query, EmptyMutation, EmptySubscription)
        .data(DataLoader::new(BatchAuthorById, tokio::spawn))
        .finish();

    let app = Router::new()
        .route("/graphql", get(graphiql_handler).post(graphql_handler))
        .layer(Extension(schema));

    let addr = "127.0.0.1:4000";
    Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap()
}
