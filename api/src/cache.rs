use std::{collections::HashMap, hash::Hash, time::Duration};
use tokio::{sync::RwLock, time::Instant};

#[derive(Debug)]
pub struct Cache<K: Clone + Hash + PartialEq + Eq, V: Clone> {
    data: RwLock<HashMap<K, V>>,
    last_update: RwLock<Option<Instant>>,
    update_interval: Duration,
}

impl<K: Clone + Hash + PartialEq + Eq, V: Clone> Cache<K, V> {
    pub fn new(update_interval: Duration) -> Cache<K, V> {
        Cache {
            data: RwLock::new(HashMap::new()),
            last_update: RwLock::new(None),
            update_interval,
        }
    }

    pub async fn get(&self) -> Option<HashMap<K, V>> {
        let now = Instant::now();
        match *self.last_update.read().await {
            None => None,
            Some(last_update) => {
                if (now - last_update) > self.update_interval {
                    None
                } else {
                    Some(self.data.read().await.clone())
                }
            }
        }
    }

    pub async fn update_all(&self, new_data: &HashMap<K, V>) {
        let mut last_update_lg = self.last_update.write().await;
        let mut data_lg = self.data.write().await;

        *last_update_lg = Some(Instant::now());
        *data_lg = new_data.clone();
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
}
