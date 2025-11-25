use std::collections::HashMap;
use tokio::sync::{Mutex, OnceCell};

use crate::ai::agent::Agent;

pub mod agent;
pub mod core;
pub mod request;

pub static AGENTS: OnceCell<Mutex<HashMap<u64, Agent>>> = OnceCell::const_new();

pub async fn init_agents() {
    AGENTS.set(Mutex::new(HashMap::new())).ok();
}

pub async fn agents() -> tokio::sync::MutexGuard<'static, HashMap<u64, Agent>> {
    AGENTS.get().expect("AGENTS not initialized").lock().await
}
