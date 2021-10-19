use nix::Result;

// See https://github.com/rust-lang/rust/issues/46016#issuecomment-605624865
pub fn reset_signal_pipe_handler() -> Result<()> {
    #[cfg(target_family = "unix")]
    {
        use nix::sys::signal;

        unsafe {
            signal::signal(signal::Signal::SIGPIPE, signal::SigHandler::SigDfl)?;
        }
    }

    Ok(())
}
