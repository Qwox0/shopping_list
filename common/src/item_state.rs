use serde::{Deserialize, Serialize};

/// replace with sections?
#[allow(unused)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ItemState {
    Needed,
    Completed,
}

impl ItemState {
    pub fn is_done(&self) -> bool {
        match self {
            ItemState::Completed => true,
            _ => false,
        }
    }

    pub fn as_attribute(&self) -> String {
        match self {
            ItemState::Completed => "completed",
            ItemState::Needed => "needed",
        }
        .to_string()
    }
}

impl From<bool> for ItemState {
    fn from(value: bool) -> Self {
        if value {
            ItemState::Completed
        } else {
            ItemState::Needed
        }
    }
}
