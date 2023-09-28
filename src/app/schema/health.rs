use async_graphql::*;
use std::time::Duration;
use tokio_stream::{Stream, StreamExt};

#[derive(Debug, Default)]
pub struct HealthQuery;

#[Object]
impl HealthQuery {
    async fn health(&self) -> bool {
        true
    }
}

#[derive(Default)]
pub struct HealthMutation;
#[Object]
impl HealthMutation {
    pub async fn get_health(&self) -> Result<&'static str> {
        Ok("health")
    }
}

#[derive(Default)]
pub struct HealthSubscription;

#[derive(Clone, PartialEq)]
pub struct OnlineUser {
    pub username: String,
}
#[Object]
impl OnlineUser {
    async fn username(&self) -> &str {
        &self.username
    }
}

#[Subscription]
impl HealthSubscription {
    pub async fn interval(&self, #[graphql(default = 1)] step: i32) -> impl Stream<Item = i32> {
        let mut value = 0;
        let interval = tokio::time::interval(Duration::from_secs(1));
        let stream: tokio_stream::wrappers::IntervalStream = tokio_stream::wrappers::IntervalStream::new(interval);
        stream.map(move |_| {
            value += step;
            value
        })
    }
    pub async fn online_user(&self) -> impl Stream<Item = OnlineUser> {
        super::SimpleBroker::<OnlineUser>::subscribe()
    }
}
