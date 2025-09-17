//! Main library file for the Aidoku multi-source extension

pub mod sources;

use aidoku::prelude::*;
use sources::mangabuddy;

// Register all sources here
#[no_mangle]
pub extern "C" fn register_sources() {
    // Sources are registered via macros in their respective modules.
    // Add more sources here in the future if needed.
}
