/// Application-wide shared state.
/// Modules receive this via Axum's State extractor.
#[derive(Clone)]
pub struct AppState {
    // Add shared resources here (e.g., DB pool, config)
}

impl AppState {
    pub fn new() -> Self {
        Self {}
    }
}
