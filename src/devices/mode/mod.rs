mod continuous;
mod oneshot;
#[cfg(not(feature = "async"))]
mod oneshot_sync;
