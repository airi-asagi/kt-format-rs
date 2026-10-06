use crate::tracing::*;

#[test]
fn error() {
    error!("value = ${}", 42);
}

#[test]
fn warn() {
    warn!("value = ${}", 42);
}

#[test]
fn info() {
    info!("value = ${}", 42);
}

#[test]
fn debug() {
    debug!("value = ${}", 42);
}

#[test]
fn trace() {
    trace!("value = ${}", 42);
}
