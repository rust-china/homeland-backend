pub mod health;

use async_graphql::extensions::{Analyzer, ApolloTracing};
use async_graphql::*;

#[derive(Default, MergedObject)]
pub struct AppQuery(health::HealthQuery);

#[derive(Default, MergedObject)]
pub struct AppMutation(health::HealthMutation);

#[derive(Default, MergedSubscription)]
pub struct AppSubscription(health::HealthSubscription);

pub type AppSchema = Schema<AppQuery, AppMutation, AppSubscription>;

pub fn build_schema() -> AppSchema {
    Schema::build(AppQuery::default(), AppMutation::default(), AppSubscription::default())
        .extension(Analyzer) // 启用 Analyzer 扩展
        .extension(ApolloTracing) // 启用 ApolloTracing 扩展
        // .limit_depth(5) // 深度 限制最大深度为 5
        // .limit_complexity(30) // 复杂度 限制最大深度为 30
        .finish()
}

use once_cell::sync::Lazy;
use slab::Slab;
use std::{
    any::{Any, TypeId},
    collections::HashMap,
    marker::PhantomData,
    pin::Pin,
    sync::Mutex,
    task::{Context, Poll},
};
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};
use tokio_stream::Stream;
/**
 * SimpleBroker
 */
static SUBSCRIBERS: Lazy<Mutex<HashMap<TypeId, Box<dyn Any + Send>>>> = Lazy::new(Default::default);
pub struct Senders<T>(Slab<UnboundedSender<T>>);
struct BrokerStream<T: Sync + Send + Clone + 'static>(usize, UnboundedReceiver<T>);
impl<T: Sync + Send + Clone + 'static> Stream for BrokerStream<T> {
    type Item = T;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.1.poll_recv(cx)
    }
}
pub struct SimpleBroker<T>(PhantomData<T>);
impl<T: Sync + Send + Clone + 'static> SimpleBroker<T> {
    pub fn publish(msg: T) {
        let type_id = TypeId::of::<T>();
        let mut map = SUBSCRIBERS.lock().unwrap();
        let senders = map.entry(type_id).or_insert_with(|| Box::new(Senders::<T>(Default::default())));
        let senders = senders.downcast_mut::<Senders<T>>().unwrap();
        for (_, sender) in senders.0.iter_mut() {
            sender.send(msg.clone()).ok();
        }
    }
    pub fn subscribe() -> impl Stream<Item = T> {
        let type_id = TypeId::of::<T>();
        let mut map = SUBSCRIBERS.lock().unwrap();

        let senders = map.entry(type_id).or_insert_with(|| Box::new(Senders::<T>(Default::default())));
        let senders = senders.downcast_mut::<Senders<T>>().unwrap();
        let (tx, rx) = mpsc::unbounded_channel::<T>();
        let tx_id = senders.0.insert(tx);
        BrokerStream(tx_id, rx)
    }
}
