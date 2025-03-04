#![no_std]
#![no_main]

pub mod pid;

// Re-export the madgwick module/crate
// ahrs actually has madgwick and mahony filters based on the new Fusion library
pub use ahrs as madgwick;