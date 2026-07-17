pub trait TBCSSerialize: serde::Serialize {}
pub trait TBCSDeserialize<'de>: serde::Deserialize<'de> {}

// Convenience for owned deserialize
pub trait TBCSDeserializeOwned: for<'de> TBCSDeserialize<'de> {}
impl<T> TBCSDeserializeOwned for T where T: for<'de> TBCSDeserialize<'de> {}
