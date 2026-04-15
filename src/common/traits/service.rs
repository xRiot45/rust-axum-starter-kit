/// Marker trait for application service structs.
/// Useful for dependency injection and testing with mocks.
pub trait Service: Send + Sync {}
