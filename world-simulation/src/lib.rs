// SpacetimeDB World Simulation Module

use log;

pub mod types;
pub mod tables;
pub mod systems;
pub mod reducers;
pub mod data_import;

use spacetimedb::{ReducerContext, Table};

// Re-export public APIs except conflicting types
pub use systems::*;
pub use reducers::*;
pub use data_import::*;

/// Module initialization
#[spacetimedb::reducer(init)]
pub fn init(ctx: &ReducerContext) {
    log::info!("World Simulation Module Initialized");
    log::info!("Run 'init_simulation' reducer to start");
}

/// Module connection handler
#[spacetimedb::reducer(client_connected)]
pub fn on_connect(ctx: &ReducerContext) {
    log::info!("Client connected");
}

/// Module disconnection handler  
#[spacetimedb::reducer(client_disconnected)]
pub fn on_disconnect(ctx: &ReducerContext) {
    log::info!("Client disconnected");
}