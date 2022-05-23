use std::collections::HashMap;
use std::fmt;

use serde::{Deserialize, Serialize};

pub const BOARD_PORT: u16 = 8085;

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Bulletin {
    pub user: String,
    pub ballot: String,
}
pub type BulletinBoard = HashMap<String, Bulletin>;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VoteOption {
    pub id: usize,
    pub name: String,
}
impl VoteOption {
    pub fn new(id: usize, name: &str) -> VoteOption {
        VoteOption {
            id,
            name: name.to_string(),
        }
    }
}
impl fmt::Display for VoteOption {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
