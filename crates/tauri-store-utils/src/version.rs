use semver::Version;
use std::borrow::Cow;
use std::sync::Arc;

pub trait Semver {
  /// Returns the semver version.
  ///
  /// # Panics
  ///
  /// Panics if the version is not a valid [semver](https://semver.org/).
  fn semver(&self) -> Version;
}

impl Semver for Version {
  #[inline]
  fn semver(&self) -> Version {
    self.clone()
  }
}

impl Semver for &str {
  fn semver(&self) -> Version {
    Version::parse(self).unwrap_or_else(|_| {
      panic!("{self} is not a valid semver version");
    })
  }
}

impl Semver for String {
  #[inline]
  fn semver(&self) -> Version {
    self.as_str().semver()
  }
}

impl Semver for &String {
  #[inline]
  fn semver(&self) -> Version {
    self.as_str().semver()
  }
}

impl Semver for Arc<str> {
  fn semver(&self) -> Version {
    self.as_ref().semver()
  }
}

impl Semver for Box<str> {
  fn semver(&self) -> Version {
    self.as_ref().semver()
  }
}

impl Semver for Cow<'_, str> {
  fn semver(&self) -> Version {
    self.as_ref().semver()
  }
}

impl Semver for u64 {
  #[inline]
  fn semver(&self) -> Version {
    Version::new(*self, 0, 0)
  }
}

impl Semver for (u64, u64) {
  #[inline]
  fn semver(&self) -> Version {
    Version::new(self.0, self.1, 0)
  }
}

impl Semver for (u64, u64, u64) {
  #[inline]
  fn semver(&self) -> Version {
    Version::new(self.0, self.1, self.2)
  }
}
