
pub mod prefabs;
pub use prefabs::*;
pub use bevy_prefabs_derive::Prefab;

#[cfg(feature = "generators")]
pub mod generators;