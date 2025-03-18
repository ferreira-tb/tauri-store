use std::borrow::Cow;

pub trait Version {
  /// Returns the semver version.
  ///
  /// # Panics
  ///
  /// Panics if the version is not a valid [semver](https://semver.org/).
  fn version(&self) -> semver::Version;
}

impl Version for semver::Version {
  #[inline]
  fn version(&self) -> semver::Version {
    self.clone()
  }
}

impl Version for &str {
  fn version(&self) -> semver::Version {
    semver::Version::parse(self).unwrap_or_else(|_| {
      panic!("{self} is not a valid semver version");
    })
  }
}

impl Version for String {
  #[inline]
  fn version(&self) -> semver::Version {
    self.as_str().version()
  }
}

impl Version for &String {
  #[inline]
  fn version(&self) -> semver::Version {
    self.as_str().version()
  }
}

impl Version for Box<str> {
  fn version(&self) -> semver::Version {
    self.as_ref().version()
  }
}

impl Version for Cow<'_, str> {
  fn version(&self) -> semver::Version {
    self.as_ref().version()
  }
}

impl Version for u64 {
  #[inline]
  fn version(&self) -> semver::Version {
    semver::Version::new(*self, 0, 0)
  }
}

impl Version for (u64, u64) {
  #[inline]
  fn version(&self) -> semver::Version {
    semver::Version::new(self.0, self.1, 0)
  }
}

impl Version for (u64, u64, u64) {
  #[inline]
  fn version(&self) -> semver::Version {
    semver::Version::new(self.0, self.1, self.2)
  }
}
