use crate::domain::models::BrowserTarget;

pub trait BrowserDiscoverer: Send + Sync {
    fn discover(&self) -> Vec<BrowserTarget>;
}
