mod _impl {
    pub use crossterm::cursor::MoveUp;
    pub use crossterm::cursor::{Hide, Show};
}
pub use _impl::*;

#[macro_export]
macro_rules! execute {
    ($writer:expr $(, $command:expr)* $(,)? ) => {
        $crate::crossterm::queue!($writer $(, $command)*).and_then(|()| {
            $writer.flush()
        })
    }
}
