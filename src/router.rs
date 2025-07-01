use axum::{Router, routing::get};
use crate::{state::AppState};
pub mod auth_router;


pub fn create_router(state: AppState) -> anyhow::Result<Router> {
    // Create a new Axum router with your routes
    let app = Router::new()
        // Add a simple root route
        .route("/", get(|| async { "Hello, World!" }))
        // Merge the auth routes from auth_router module
        .merge(auth_router::auth_router(state));

    // Return the router with state
    Ok(app)
}