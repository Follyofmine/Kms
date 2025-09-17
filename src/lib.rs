//! Main library file for the Aidoku multi-source extension

mod sources;

use aidoku::prelude::*;
use sources::mangabuddy;

// Register all sources here
#[no_mangle]
pub extern "C" fn register_sources() {
    mangabuddy::register();
    // Add more sources here in the future
}
