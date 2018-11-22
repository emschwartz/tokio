pub use std::time::Duration;

#[cfg(not(target_arch = "wasm32"))]
pub use std::time::Instant;

#[cfg(target_arch = "wasm32")]
use js_sys::Date;
#[cfg(target_arch = "wasm32")]
use std::ops::{Add, Sub};

#[cfg(target_arch = "wasm32")]
#[derive(PartialEq, Eq, Ord, PartialOrd, Debug, Copy, Clone)]
pub struct Instant(u64);

#[cfg(target_arch = "wasm32")]
impl Instant {
  pub fn now() -> Self {
    let now = Date::new_0();
    let millisecs_since_unix_epoch: u64 = now.get_time() as u64;
    Instant(millisecs_since_unix_epoch)
  }
}

#[cfg(target_arch = "wasm32")]
impl Add<Duration> for Instant {
  type Output = Instant;

  fn add(self, other: Duration) -> Instant {
    Instant(self.0 + other.as_secs() * 1000 + (other.subsec_millis() as u64))
  }
}

#[cfg(target_arch = "wasm32")]
impl Sub<Duration> for Instant {
  type Output = Instant;

  fn sub(self, other: Duration) -> Instant {
    Instant(self.0 - (other.as_secs() * 1000 + (other.subsec_millis() as u64)))
  }
}

#[cfg(target_arch = "wasm32")]
impl Sub<Instant> for Instant {
  type Output = Duration;

  fn sub(self, other: Instant) -> Duration {
    Duration::from_millis(self.0 - other.0)
  }
}
