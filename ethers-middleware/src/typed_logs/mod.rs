use std::sync::Arc;
mod typed_logs_stream;

use async_trait::async_trait;
use ethers_core::types::{Filter, Log};
use ethers_providers::{FromErr, Middleware, PubsubClient, SubscriptionStream};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError<M: Middleware> {
    #[error("{0}")]
    MiddlewareError(M::Error),
}

impl<M: Middleware> FromErr<M::Error> for MyError<M> {
    fn from(src: M::Error) -> MyError<M> {
        MyError::MiddlewareError(src)
    }
}

#[derive(Debug)]
pub struct TypedLogsMiddleware<M> {
    pub(crate) inner: Arc<M>,
}

#[async_trait]
impl<M> Middleware for TypedLogsMiddleware<M>
where
    M: Middleware,
{
    type Error = MyError<M>;
    type Provider = M::Provider;
    type Inner = M;

    fn inner(&self) -> &M {
        &self.inner
    }
}

impl<M> TypedLogsMiddleware<M> where M: Middleware {
    async fn subscribe_logs_b<'a>(
        &'a self,
        filter: &Filter,
    ) -> Result<SubscriptionStream<'a, M::Provider, Log>,  MyError<M>>
    where
        <Self as Middleware>::Provider: PubsubClient,
    {
        self.inner()
            .subscribe_logs(filter)
            .await
            .map_err(FromErr::from)
    }
}
