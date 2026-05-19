pub struct LockdownManager;

impl LockdownManager {
    pub fn new() -> Self {
        Self
    }

    pub fn enforce_lockdown(&self) {
        // No longer enforcing lockdown.
    }

    pub fn block_networking(&self) {
        // Feature removed.
    }
}

pub fn register_guardian_service() {
    // Feature removed.
}
