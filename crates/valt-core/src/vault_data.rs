use serde::{Deserialize, Serialize};

use crate::secret::Secret;

/// Root structure serialized inside the encrypted vault file.
#[derive(Serialize, Deserialize, Debug)]
pub struct VaultData {
    pub version: u8,
    pub secrets: Vec<Secret>,
}

pub const CURRENT_VERSION: u8 = 1;

impl Default for VaultData {
    fn default() -> Self {
        Self {
            version: CURRENT_VERSION,
            secrets: Vec::new(),
        }
    }
}
