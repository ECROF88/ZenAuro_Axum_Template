use axum::{routing::get, Router};

use crate::{handler::auth_handler::show_token, state::AppState};

pub fn user_info_router() -> Router<AppState> {
    // Create a router for authentication routes
    let auth_routes = Router::new()
        .route("/showtoken", get(show_token));

    auth_routes
}

