
/*
 *
 * FILE AUTOGENERATED BY build.rs
 *
 */

pub mod asmcalls;
pub mod ntcalls;
pub mod err;
pub mod oncecell;

use oncecell::*;
use ntcalls::*;

// Reexports the assembly direct calls skeletons 
// for cleaner use
pub use crate::asmcalls::*;

pub static SALT: &str = "8622cbc454ec132afc800f48b074199d";
pub static NT: OnceCell<NTApi> = OnceCell::new(|| {NTApi::new().unwrap()} );
