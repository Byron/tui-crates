pub use crossterm::event::{Event, KeyEvent as Key};

enum Action<T> {
    Continue,
    Result(Result<T, std::io::Error>),
}

fn continue_on_interrupt<T>(result: Result<T, std::io::Error>) -> Action<T> {
    match result {
        Ok(v) => Action::Result(Ok(v)),
        Err(err) if err.kind() == std::io::ErrorKind::Interrupted => Action::Continue,
        Err(err) => Action::Result(Err(err)),
    }
}

/// Return a receiver of user key input events to avoid blocking the main thread.
pub fn key_input_channel() -> std::sync::mpsc::Receiver<Key> {
    let (key_send, key_receive) = std::sync::mpsc::sync_channel(0);
    std::thread::spawn(move || -> Result<(), std::io::Error> {
        loop {
            let event = match continue_on_interrupt(crossterm::event::read()) {
                Action::Continue => continue,
                Action::Result(res) => res?,
            };
            match event {
                crossterm::event::Event::Key(key) => {
                    if key_send.send(key).is_err() {
                        break;
                    }
                }
                _ => continue,
            };
        }
        Ok(())
    });
    key_receive
}

/// Return a receiver of user input events to avoid blocking the main thread.
pub fn input_channel() -> std::sync::mpsc::Receiver<Event> {
    use std::convert::TryInto;

    let (key_send, key_receive) = std::sync::mpsc::sync_channel(0);
    std::thread::spawn(move || -> Result<(), std::io::Error> {
        loop {
            let event = match continue_on_interrupt(crossterm::event::read()) {
                Action::Continue => continue,
                Action::Result(res) => res?,
            };
            if let Ok(event) = event.try_into() {
                if key_send.send(event).is_err() {
                    break;
                }
            }
        }
        Ok(())
    });
    key_receive
}

/// Return a stream of key input Events
///
/// Requires the `input-async` feature.
#[cfg(feature = "input-async-crossterm")]
pub fn key_input_stream() -> impl futures_core::stream::Stream<Item = Key> {
    use futures_lite::StreamExt;
    crossterm::event::EventStream::new()
        .filter_map(|r| r.ok())
        .filter_map(|e| match e {
            crossterm::event::Event::Key(key) => Some(key),
            _ => None,
        })
}

/// Return a stream of input Events
///
/// Requires the `input-async` feature.
#[cfg(feature = "input-async-crossterm")]
pub fn input_stream() -> impl futures_core::stream::Stream<Item = Event> {
    use futures_lite::StreamExt;
    crossterm::event::EventStream::new().filter_map(|r| r.ok())
}
