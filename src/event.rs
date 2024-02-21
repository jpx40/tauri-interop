use serde::{Deserialize, Serialize};
#[cfg(not(target_family = "wasm"))]
use tauri::{AppHandle, Error, Wry};

/// traits for event emitting in the host code (feat: `tauri`)
#[cfg(not(target_family = "wasm"))]
pub mod emit;
/// related generic struct and functions for autogenerated listen functions (target: `wasm` or feat: `wasm`)
#[cfg(any(target_family = "wasm", feature = "wasm"))]
pub mod listen;

/// The trait which needs to be implemented for a [Field]
///
/// Conditionally changes between [listen::Listen] and [emit::Emit]
///
/// When compiled to "target_family = wasm" then following is true.
/// ```ignore
/// trait Parent = listen::Listen;
/// ```
#[cfg(not(target_family = "wasm"))]
pub trait Parent = emit::Emit;
/// The trait which needs to be implemented for a [Field]
///
/// Conditionally changes between [listen::Listen] and [emit::Emit]
#[cfg(target_family = "wasm")]
pub trait Parent = listen::Listen;

/// Trait defining a [Field] to a related struct implementing [Parent] with the related [Field::Type]
pub trait Field<P>
where
    P: Parent,
    <Self as Field<P>>::Type: Clone + Serialize + for<'de> Deserialize<'de>,
{
    /// The type of the field
    type Type;

    #[cfg(any(target_family = "wasm", feature = "wasm"))]
    /// The event of the field
    const EVENT_NAME: &'static str;

    #[cfg(not(target_family = "wasm"))]
    /// Emits event of the related field with their value
    fn emit(parent: &P, handle: &AppHandle<Wry>) -> Result<(), Error>;

    #[cfg(not(target_family = "wasm"))]
    /// Updates the related field and emit its event
    ///
    /// Only required for "target_family = wasm"
    fn update(s: &mut P, handle: &AppHandle<Wry>, v: Self::Type) -> Result<(), Error>;
}
