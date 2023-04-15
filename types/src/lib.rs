//! C basic types for build target architecture
include!(concat!(env!("OUT_DIR"), "/types.inc"));
/// void
pub type Void = std::ffi::c_void;
/// sighandler_t
pub type SigHandler= extern fn(sig: SInt);
