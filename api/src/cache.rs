use futures::{future::BoxFuture, Future};
use std::{collections::HashMap, hash::Hash, marker::PhantomData, time::Duration};
use tokio::{sync::RwLock, time::Instant};

pub trait CacheAsyncFn<K: Clone + Hash + PartialEq + Eq, V: Clone, Error: std::error::Error>:
    Send + Sync
{
    fn call(&self) -> BoxFuture<'static, Result<HashMap<K, V>, Error>>;
}

impl<
        K: Clone + Hash + PartialEq + Eq,
        V: Clone,
        Error: std::error::Error,
        T: Fn() -> F + Send + Sync,
        F: Future<Output = Result<HashMap<K, V>, Error>> + 'static + Send,
    > CacheAsyncFn<K, V, Error> for T
{
    fn call(&self) -> BoxFuture<'static, Result<HashMap<K, V>, Error>> {
        Box::pin(self())
    }
}

pub struct Cache<K: Clone + Hash + PartialEq + Eq, V: Clone, Error: std::error::Error> {
    data: RwLock<HashMap<K, V>>,
    last_update: RwLock<Option<Instant>>,
    update_interval: Duration,
    update_fn: Box<dyn CacheAsyncFn<K, V, Error>>,
    _phantom: PhantomData<Error>,
}

impl<K: Clone + Hash + PartialEq + Eq, V: Clone, Error: std::error::Error> Cache<K, V, Error> {
    pub fn new<
        T: Fn() -> F + 'static + Send + Sync,
        F: Future<Output = Result<HashMap<K, V>, Error>> + 'static + Send,
    >(
        update_interval: Duration,
        update_fn: T,
    ) -> Cache<K, V, Error> {
        Cache {
            data: RwLock::new(HashMap::new()),
            last_update: RwLock::new(None),
            update_interval,
            update_fn: Box::new(update_fn),
            _phantom: PhantomData,
        }
    }

    pub async fn get_all(&self) -> Result<HashMap<K, V>, Error> {
        if self.check_needs_update().await {
            let new_values = self.update_fn.call().await?;
            let mut data_lg = self.data.write().await;
            *data_lg = new_values.clone();
            Ok(new_values)
        } else {
            Ok(self.data.read().await.clone())
        }
    }

    pub async fn get(&self, k: &K) -> Result<Option<V>, Error> {
        if self.check_needs_update().await {
            let new_values = self.update_fn.call().await?;
            let mut data_lg = self.data.write().await;
            *data_lg = new_values.clone();
            Ok(new_values.get(k).cloned())
        } else {
            Ok(self.data.read().await.get(k).cloned())
        }
    }

    pub async fn update(&self, k: K, v: V) {
        let mut data_lg = self.data.write().await;
        match data_lg.get_mut(&k) {
            Some(old_val) => {
                *old_val = v;
            }
            None => {
                data_lg.insert(k, v);
            }
        }
    }

    async fn check_needs_update(&self) -> bool {
        let now = Instant::now();
        let last_update = {
            let lg = self.last_update.read().await;
            (*lg).clone()
        };

        match last_update {
            None => true,
            Some(last_update) => (now - last_update) > self.update_interval,
        }
    }
}
