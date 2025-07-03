use crate::{router::user_info_router::user_info_router, state::AppState};
use axum::{Router, routing::get};
pub mod auth_router;
pub mod user_info_router;

pub fn create_router(state: AppState) -> anyhow::Result<Router> {
    // Create a new Axum router with your routes
    let app = Router::new()
        // Add a simple root route
        .route("/", get(|| async { "Hello, World!" }))
        .nest("/api/vi/auth",auth_router::auth_router())
        .nest("/api/v1/user",user_info_router())
        .with_state(state);  
    // Return the router with state
    Ok(app)
}
