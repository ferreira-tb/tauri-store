/// A marker trait used to enable plugin interoperability.
pub trait CollectionMarker: Send + Sync + 'static {}

/// Default collection marker.
pub struct DefaultMarker;

impl CollectionMarker for DefaultMarker {}
