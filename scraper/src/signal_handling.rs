#[cfg(not(unix))]
use futures::future::pending;
#[cfg(unix)]
use tokio::signal::unix::{signal, SignalKind};

pub async fn handle_sigterm() {
    #[cfg(not(unix))]
    {
        pending::<()>().await;
        unreachable!("SIGTERM handling is not applicable in non-Unix platforms.");
    }

    #[cfg(unix)]
    {
        let mut term_signal = signal(SignalKind::terminate()).expect("Failed to set up SIGTERM handler");
        term_signal.recv().await;
        println!("SIGTERM signal received.");
    }
}
