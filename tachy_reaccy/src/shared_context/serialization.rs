use std::{error::Error, sync::Arc};
use thiserror::Error;

/// Describes errors that can occur while serializing and deserializing data,
/// typically during the process of streaming [`Resource`](crate::Resource)s from
/// the server to the client.
#[derive(Debug, Clone, Error)]
pub enum SerializationError {
    /// Errors that occur during serialization.
    #[error("error serializing Resource: {0}")]
    Serialize(Arc<dyn Error + Send + Sync>),
    /// Errors that occur during deserialization.
    #[error("error deserializing Resource: {0}")]
    Deserialize(Arc<dyn Error + Send + Sync>),
}

/// Describes an object that can be serialized to or from a supported format
/// Currently those are JSON and Cbor
///
/// This is primarily used for serializing and deserializing [`Resource`](crate::Resource)s
/// so they can begin on the server and be resolved on the client, but can be used
/// for any data that needs to be serialized/deserialized.
///
/// This trait is intended to abstract over various serialization crates,
/// as selected between by the crate features `serde` (default), `serde-lite`,
/// and `miniserde`.
pub trait Serializable {
    /// Serializes the object to a string.
    fn ser(&self) -> Result<String, SerializationError>;

    /// Deserializes the object from some bytes.
    fn de(bytes: &str) -> Result<Self, SerializationError>
    where
        Self: Sized;
}

#[cfg(feature = "serde")]
use serde::{de::DeserializeOwned, Serialize};

#[cfg(feature = "serde")]
impl<T> Serializable for T
where
    T: DeserializeOwned + Serialize,
{
    fn ser(&self) -> Result<String, SerializationError> {
        serde_json::to_string(&self)
            .map_err(|e| SerializationError::Serialize(Arc::new(e)))
    }

    fn de(json: &str) -> Result<Self, SerializationError> {
        serde_json::from_str(json)
            .map_err(|e| SerializationError::Deserialize(Arc::new(e)))
    }
}
