//! Read-only views of the signed-release/native update owner.
//! A plan is policy intent, never proof that an update ran or is available.

#[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
mod adapter;
pub mod view;

#[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
pub use adapter::Updater;
#[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
pub use nddev_device_sync_adapter_io::{CancellationToken, IoError, NativeIo, ProcessIo};

#[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] IoError),
    #[error("updater protocol response is invalid or unsupported")]
    Protocol,
    #[error("native updater provider is not supported on this platform")]
    UnsupportedPlatform,
}

/// Application port: callers depend on typed observations, not a native process.
#[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
pub trait ReadUpdater {
    fn plan(
        &self,
        cancellation: CancellationToken,
    ) -> impl std::future::Future<Output = Result<view::Plan, Error>> + Send;
    fn last_run(
        &self,
        cancellation: CancellationToken,
    ) -> impl std::future::Future<Output = Result<view::UpdateRunSummary, Error>> + Send;
}

/// Canonical compiled-in runtime manifest for this integration.
pub const MANIFEST: &str = include_str!("../contracts/module.json");
