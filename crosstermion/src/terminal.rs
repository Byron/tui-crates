#[cfg(feature = "crossterm")]
mod _impl {
    use crossterm::{
        execute,
        terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    };
    use std::io;

    /// Return the horizontal and vertical size of the terminal, if available.
    pub fn size() -> io::Result<(u16, u16)> {
        crossterm::terminal::size()
    }

    /// A utility writer to activate an alternate screen in raw mode on instantiation, and resets to previous settings on drop.
    ///
    /// Additionally, it will activate _raw_ mode, which causes user input not to show on screen,
    /// and gets handled byte by byte.
    pub struct AlternateRawScreen<T: io::Write> {
        inner: T,
    }

    impl<T: io::Write> AlternateRawScreen<T> {
        pub fn try_from(mut write: T) -> Result<Self, io::Error> {
            terminal::enable_raw_mode()?;
            execute!(write, EnterAlternateScreen)?;
            Ok(AlternateRawScreen { inner: write })
        }
    }

    impl<T: io::Write> io::Write for AlternateRawScreen<T> {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.inner.write(buf)
        }

        fn flush(&mut self) -> io::Result<()> {
            self.inner.flush()
        }
    }

    impl<T: io::Write> Drop for AlternateRawScreen<T> {
        fn drop(&mut self) {
            terminal::disable_raw_mode().ok();
            execute!(self.inner, LeaveAlternateScreen).ok();
        }
    }

    #[cfg(all(feature = "tui-crossterm-backend", not(feature = "tui-react")))]
    pub mod tui {
        use tui::backend::CrosstermBackend;

        pub fn new_terminal<W: std::io::Write>(
            write: W,
        ) -> Result<tui::Terminal<CrosstermBackend<W>>, std::io::Error> {
            let backend = CrosstermBackend::new(write);
            Ok(tui::Terminal::new(backend)?)
        }
    }

    #[cfg(all(feature = "tui-crossterm-backend", feature = "tui-react"))]
    pub mod tui {
        use tui::backend::CrosstermBackend;

        pub fn new_terminal<W: std::io::Write>(
            write: W,
        ) -> Result<tui_react::Terminal<CrosstermBackend<W>>, std::io::Error> {
            let backend = CrosstermBackend::new(write);
            Ok(tui_react::Terminal::new(backend)?)
        }
    }
}
pub use _impl::*;
