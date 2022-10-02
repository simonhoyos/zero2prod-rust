// include a new module into the module tree.
mod health_check;
mod subscriptions;

// Make all routes public and accessible from a single file.
pub use health_check::*;
pub use subscriptions::*;
