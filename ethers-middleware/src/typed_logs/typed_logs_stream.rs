use std::{marker::PhantomData, pin::Pin};

use ethers_contract::EthLogDecode;
use ethers_core::{types::Log, abi::RawLog};
use futures_util::Stream;

#[must_use = "subscriptions do nothing unless you stream them"]
pub struct TypedLogsStream<R: EthLogDecode + Unpin> {
    inner_stream: Pin<Box<dyn Stream<Item = Log>>>,
    ret: PhantomData<R>
}

impl <R: EthLogDecode + Unpin> TypedLogsStream<R> {
    pub fn new(stream: Box<dyn Stream<Item = Log>>) -> Self {
        Self { 
            inner_stream: stream.into(), 
            ret: PhantomData
        }
    }
}

impl<R: EthLogDecode + Unpin> Stream for TypedLogsStream<R> {
    type Item = R;

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
       
        match self.get_mut().inner_stream.as_mut().poll_next(cx) {
            std::task::Poll::Ready(log) => {
                let log: Log = log.unwrap();
                let raw_log: RawLog = log.into();
                let item = R::decode_log(&raw_log).unwrap();
                std::task::Poll::Ready(Some(item))
            },
            std::task::Poll::Pending => std::task::Poll::Pending,
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, None)
    }

    /*fn poll_next(self: Pin<&mut Self>, ctx: &mut Context) -> Poll<Option<Self::Item>> {
        if !self.loaded_elements.is_empty() {
            let next_element = self.get_mut().loaded_elements.pop_front();
            return Poll::Ready(next_element)
        }

        let this = self.project();
        match futures_util::ready!(this.rx.poll_next(ctx)) {
            Some(item) => match serde_json::from_str(item.get()) {
                Ok(res) => Poll::Ready(Some(res)),
                Err(err) => {
                    error!("failed to deserialize item {:?}", err);
                    Poll::Pending
                }
            },
            None => Poll::Ready(None),
        }
    }*/
}
