//! SIGINT and SIGTERM as a flag, so an interrupted command unwinds.

use std::ffi::c_int;
use std::io;
use std::process::ExitCode;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

use signal_hook::consts::SIGINT;
use signal_hook::consts::SIGTERM;
use signal_hook::flag;
use signal_hook::low_level;

/// The record of a signal that asked the process to stop.
#[derive(Debug)]
pub(crate) struct Interrupt {
    /// Set once a signal has arrived, which lets a second one end the process.
    received: Arc<AtomicBool>,
    /// The number of the signal that arrived, or zero.
    signal: Arc<AtomicUsize>,
    /// Whether the signals are watched already.
    watching: AtomicBool,
}

impl Interrupt {
    /// Creates a record that no signal sets until [`Self::watch`] is called.
    pub(crate) fn new() -> Self {
        Self {
            received: Arc::new(AtomicBool::new(false)),
            signal: Arc::new(AtomicUsize::new(0)),
            watching: AtomicBool::new(false),
        }
    }

    /// Makes SIGINT and SIGTERM set this record instead of ending the process.
    ///
    /// A command that holds something to clean up calls this before it creates
    /// it. A second signal ends the process at once. A second call does nothing:
    /// registered twice, the actions would end the process on the first signal.
    pub(crate) fn watch(&self) -> io::Result<()> {
        if self.watching.swap(true, Ordering::Relaxed) {
            return Ok(());
        }
        for signal in [SIGINT, SIGTERM] {
            // NOTE: the conditional action goes first. Registered after the
            // flag, it would end the process on the first signal.
            flag::register_conditional_default(signal, Arc::clone(&self.received))?;
            flag::register(signal, Arc::clone(&self.received))?;
            flag::register_usize(
                signal,
                Arc::clone(&self.signal),
                signal.cast_unsigned() as usize,
            )?;
        }
        Ok(())
    }

    /// Returns the signal that arrived, if one has.
    pub(crate) fn signal(&self) -> Option<c_int> {
        match self.signal.load(Ordering::Relaxed) {
            0 => None,
            signal => c_int::try_from(signal).ok(),
        }
    }
}

/// Ends the process by `signal`, as if no handler had been installed.
pub(crate) fn terminate(signal: c_int) -> ExitCode {
    // The default action of SIGINT and SIGTERM ends the process, so this
    // returns only when that action could not be taken.
    let _ = low_level::emulate_default_handler(signal);
    ExitCode::from(2)
}
