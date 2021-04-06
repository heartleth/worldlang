pub use super::transpile;
pub use super::IS_QUIET;

pub fn is_quiet()->bool {
    unsafe { IS_QUIET }
}

pub mod filesys;