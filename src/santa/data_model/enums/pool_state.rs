use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub enum PoolState {
    Created = 0,
    Pooling = 1,
    Started = 2,
    Ended = 3
}