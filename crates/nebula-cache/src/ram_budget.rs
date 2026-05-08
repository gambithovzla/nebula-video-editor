/// Soft cap for decoded-frame RAM (bytes). Eviction policy comes later.
#[derive(Debug, Clone, Copy)]
pub struct RamBudget {
    max_bytes: u64,
    used_bytes: u64,
}

impl RamBudget {
    #[must_use]
    pub const fn new(max_bytes: u64) -> Self {
        Self {
            max_bytes,
            used_bytes: 0,
        }
    }

    #[must_use]
    pub fn max_bytes(&self) -> u64 {
        self.max_bytes
    }

    #[must_use]
    pub fn used_bytes(&self) -> u64 {
        self.used_bytes
    }

    pub fn try_reserve(&mut self, bytes: u64) -> bool {
        if self.used_bytes.saturating_add(bytes) > self.max_bytes {
            return false;
        }
        self.used_bytes += bytes;
        true
    }

    pub fn release(&mut self, bytes: u64) {
        self.used_bytes = self.used_bytes.saturating_sub(bytes);
    }
}
