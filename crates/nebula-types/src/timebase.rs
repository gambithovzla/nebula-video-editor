use serde::{Deserialize, Serialize};

use crate::TypesError;

/// Project frame rate (e.g. 24/1, 30000/1001).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct FrameRate {
    numerator: u32,
    denominator: u32,
}

impl FrameRate {
    /// Creates a frame rate from `numerator / denominator` (e.g. 24000, 1001).
    ///
    /// # Errors
    /// Returns [`TypesError::InvalidFrameRate`] if numerator is 0 or denominator is 0.
    pub fn from_rational(numerator: u32, denominator: u32) -> Result<Self, TypesError> {
        if numerator == 0 || denominator == 0 {
            return Err(TypesError::InvalidFrameRate(0.0));
        }
        Ok(Self {
            numerator,
            denominator,
        })
    }

    #[must_use]
    pub const fn numerator(&self) -> u32 {
        self.numerator
    }

    #[must_use]
    pub const fn denominator(&self) -> u32 {
        self.denominator
    }

    /// Approximate frames per second as `f64`.
    #[must_use]
    pub fn as_f64(&self) -> f64 {
        f64::from(self.numerator) / f64::from(self.denominator)
    }
}

/// Duration or offset on the timeline in seconds (high-level; sub-frame quantisation lives in timeline).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct TimeSpan {
    seconds: f64,
}

impl TimeSpan {
    /// # Errors
    /// Returns [`TypesError::NegativeDuration`] if `seconds` is negative.
    pub fn from_seconds(seconds: f64) -> Result<Self, TypesError> {
        if seconds < 0.0 {
            return Err(TypesError::NegativeDuration);
        }
        Ok(Self { seconds })
    }

    #[must_use]
    pub const fn from_seconds_unchecked(seconds: f64) -> Self {
        Self { seconds }
    }

    #[must_use]
    pub fn as_seconds(&self) -> f64 {
        self.seconds
    }
}
