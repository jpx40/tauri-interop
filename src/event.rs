use serde::{de::DeserializeOwned, Deserialize, Serialize};
#[cfg(not(target_family = "wasm"))]
use tauri::{AppHandle, Error, Wry};

#[cfg(not(target_family = "wasm"))]
#[doc(cfg(not(target_family = "wasm")))]
pub use emit::*;
#[cfg(any(target_family = "wasm", doc))]
#[doc(cfg(target_family = "wasm"))]
pub use listen::*;
#[cfg(doc)]
use tauri_interop_macro::{Emit, EmitField, Event, Listen, ListenField};

/// traits for event emitting in the host code
#[cfg(not(target_family = "wasm"))]
#[doc(cfg(not(target_family = "wasm")))]
mod emit;

/// related generic struct and functions for autogenerated listen functions
#[cfg(any(target_family = "wasm", doc))]
#[doc(cfg(target_family = "wasm"))]
mod listen;

#[allow(clippy::needless_doctest_main)]
/// Trait defining a [Field] to a related struct implementing [Parent] with the related [Field::Type]
///
/// When using [Event], [Emit] or [Listen], for each field of the struct, a struct named after the 
/// field is generated. The field naming is snake_case to PascalCase, but because of the possibility
/// that the type and the field name are the same, the generated field has a "F" appended at the 
/// beginning to separate each other and avoid type collision.
/// 
/// ```
/// use serde::{Deserialize, Serialize};
/// use tauri_interop::{Event, event::ManagedEmit};
/// 
/// #[derive(Default, Clone, Serialize, Deserialize)]
/// struct Bar {
///     foo: u16
/// }
///
/// #[derive(Event)]
/// struct Test {
///     bar: Bar
/// }
/// 
/// impl ManagedEmit for Test {}
///
/// fn main() {
///     let _ = test::FBar;
/// }
/// ```
pub trait Field<P>
where
    P: Parent,
    Self::Type: Default + Clone + Serialize + DeserializeOwned,
{
    /// The type of the field
    type Type;

    /// The event of the field
    const EVENT_NAME: &'static str;

    /// Tries to retrieve the current value from the backend
    #[allow(async_fn_in_trait)]
    #[cfg(any(all(target_family = "wasm", feature = "initial_value"), doc))]
    #[doc(cfg(all(target_family = "wasm", feature = "initial_value")))]
    async fn get_value() -> Result<Self::Type, EventError>;

    #[cfg(not(target_family = "wasm"))]
    #[doc(cfg(not(target_family = "wasm")))]
    /// Emits event of the related field with their value
    ///
    /// not in wasm available
    fn emit(parent: &P, handle: &AppHandle<Wry>) -> Result<(), Error>;

    #[cfg(not(target_family = "wasm"))]
    #[doc(cfg(not(target_family = "wasm")))]
    /// Updates the related field and emit its event
    ///
    /// not in wasm available
    fn update(s: &mut P, handle: &AppHandle<Wry>, v: Self::Type) -> Result<(), Error>;
}

#[cfg(any(feature = "initial_value", doc))]
#[doc(cfg(feature = "initial_value"))]
/// General errors that can happen during event exchange
#[derive(Debug, Serialize, Deserialize, thiserror::Error)]
pub enum EventError {
    /// The given name (struct) is not as tauri::State registered
    #[error("{0} is not as tauri state registered")]
    StateIsNotRegistered(String),
}
