use serde::{Deserialize, Serialize};

use crate::app::Instance;

#[derive(Default, Serialize, Deserialize)]
pub struct Config {
    pub instance: Vec<Instance>,
}

