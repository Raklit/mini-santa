use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum PoolState {
    Created = 0,
    Pooling = 1,
    Started = 2,
    Ended = 3
}

impl TryFrom<usize> for PoolState {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            value if value == PoolState::Created as usize => Ok(PoolState::Created),
            value if value == PoolState::Pooling as usize => Ok(PoolState::Pooling),
            value if value == PoolState::Started as usize => Ok(PoolState::Started),
            value if value == PoolState::Ended as usize => Ok(PoolState::Ended),
            _ => Err(())
        }
    }
}