use crate::domain::models::BrowserTarget;
use crate::platform::traits::BrowserDiscoverer;
use std::sync::Arc;

pub struct DiscoveryService {
    discoverer: Arc<dyn BrowserDiscoverer>,
}

impl DiscoveryService {
    pub fn new(discoverer: Arc<dyn BrowserDiscoverer>) -> Self {
        Self { discoverer }
    }

    pub fn discover_installed_browsers(&self) -> Vec<BrowserTarget> {
        self.discoverer.discover()
    }
}
