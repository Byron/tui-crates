#[cfg(feature = "crossterm")]
mod _impl {
    pub use crossterm::cursor::MoveUp;
    pub use crossterm::cursor::{Hide, Show};
}
#[cfg(feature = "crossterm")]
pub use _impl::*;

#[cfg(feature = "crossterm")]
#[macro_export]
macro_rules! execute {
    ($writer:expr $(, $command:expr)* $(,)? ) => {
        $crate::crossterm::queue!($writer $(, $command)*).and_then(|()| {
            $writer.flush()
        })
    }
}

#[cfg(all(feature = "termion", not(feature = "crossterm")))]
#[macro_export]
macro_rules! execute {
    ($writer:expr $(, $command:expr)* $(,)? ) => {
        Ok(()) $(
            .and_then(|()| write!($writer, "{}", $command))
        )*.and_then(|_| $writer.flush())
    }
}

#[cfg(all(feature = "termion", not(feature = "crossterm")))]
mod _impl {
    pub use termion::cursor::Up as MoveUp;
    pub use termion::cursor::{Hide, Show};
}
#[cfg(all(feature = "termion", not(feature = "crossterm")))]
pub use _impl::*;
