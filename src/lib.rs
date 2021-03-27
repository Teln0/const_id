use proc_macro_hack::proc_macro_hack;

extern crate const_id_macro;

/// Increments and returns the current ID, effectively creating a unique ID at compile time
/// ever time this function is called
///
/// ```
/// const ID: usize = const_id::const_id!();
/// ```
#[proc_macro_hack]
pub use const_id_macro::const_id;