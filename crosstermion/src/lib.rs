/// Features related to terminal user input
#[cfg(feature = "crossterm")]
pub mod input;
/// Features related to the terminal settings and terminal user interfaces.
#[cfg(feature = "crossterm")]
pub mod terminal;

#[cfg(feature = "crossterm")]
pub mod cursor;

pub mod color;

// Reexports
#[cfg(feature = "nu-ansi-term")]
pub use nu_ansi_term;
#[cfg(feature = "crossterm")]
pub use crossterm;
#[cfg(feature = "tui")]
pub use tui;
#[cfg(feature = "tui-react")]
pub use tui_react;
