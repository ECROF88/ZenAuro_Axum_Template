use crate::handler::auth_handler::login;
use crate::AppState;
use axum::{
    middleware::from_fn_with_state, routing::{get, post}, Router
};

pub fn auth_router(state:AppState) -> Router {
    // Create a router for authentication routes
    let auth_routes = Router::new()
        .route("/login", post(login))
        // .layer(from_fn_with_state(state, f))
        .with_state(state);

    auth_routes
}