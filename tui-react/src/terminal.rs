//! Derived from TUI-rs, license: MIT, Copyright (c) 2016 Florian Dehau
use std::{borrow::Borrow, io};

use tui::{backend::Backend, buffer::Buffer, layout::Rect, prelude::Position};

/// A component meant to be rendered by `Terminal::render(...)`.
/// All other components don't have to implement this trait, and instead
/// provide a render method by convention, tuned towards their needs using whichever
/// generic types or lifetimes they need.
pub trait ToplevelComponent {
    type Props;

    fn render(&mut self, props: impl Borrow<Self::Props>, area: Rect, buf: &mut Buffer);
}

#[derive(Debug)]
pub struct Terminal<B>
where
    B: Backend,
{
    pub backend: B,
    buffers: [Buffer; 2],
    current: usize,
    hidden_cursor: bool,
    known_size: Rect,
}

impl<B> Drop for Terminal<B>
where
    B: Backend,
{
    fn drop(&mut self) {
        // Attempt to restore the cursor state
        if self.hidden_cursor {
            // We can't use the where clause in drop, so we need to work around it
            // For backends that don't implement Into<io::Error>, this won't compile
            // but that's acceptable since most real backends do implement this
            let _ = self.backend.show_cursor();
        }
    }
}

impl<B> Terminal<B>
where
    B: Backend,
{
    pub fn new(backend: B) -> io::Result<Terminal<B>>
    where
        B::Error: Send + Sync + 'static,
    {
        let size = backend.size().map_err(io::Error::other)?.into();
        Ok(Terminal {
            backend,
            buffers: [Buffer::empty(size), Buffer::empty(size)],
            current: 0,
            hidden_cursor: false,
            known_size: size,
        })
    }

    pub fn current_buffer_mut(&mut self) -> &mut Buffer {
        &mut self.buffers[self.current]
    }

    pub fn reconcile_and_flush(&mut self) -> io::Result<()>
    where
        B::Error: Send + Sync + 'static,
    {
        let previous_buffer = &self.buffers[1 - self.current];
        let current_buffer = &self.buffers[self.current];
        let updates = previous_buffer.diff(current_buffer);
        self.backend
            .draw(updates.into_iter())
            .map_err(io::Error::other)
    }

    pub fn resize(&mut self, area: Rect) -> io::Result<()>
    where
        B::Error: Send + Sync + 'static,
    {
        self.buffers[self.current].resize(area);
        self.buffers[1 - self.current].reset();
        self.buffers[1 - self.current].resize(area);
        self.known_size = area;
        self.backend.clear().map_err(io::Error::other)
    }

    pub fn autoresize(&mut self) -> io::Result<()>
    where
        B::Error: Send + Sync + 'static,
    {
        let size = self.size()?;
        if self.known_size != size {
            self.resize(size)?;
        }
        Ok(())
    }

    /// Get ready for rendering and return the maximum display size as `Rect`
    pub fn pre_render(&mut self) -> io::Result<Rect>
    where
        B::Error: Send + Sync + 'static,
    {
        // Autoresize - otherwise we get glitches if shrinking or potential desync between widgets
        // and the terminal (if growing), which may OOB.
        self.autoresize()?;
        Ok(self.known_size)
    }

    pub fn post_render(&mut self) -> io::Result<()>
    where
        B::Error: Send + Sync + 'static,
    {
        self.reconcile_and_flush()?;

        self.buffers[1 - self.current].reset();
        self.current = 1 - self.current;

        self.backend.flush().map_err(io::Error::other)?;
        Ok(())
    }

    pub fn render<C>(&mut self, component: &mut C, props: impl Borrow<C::Props>) -> io::Result<()>
    where
        C: ToplevelComponent,
        B::Error: Send + Sync + 'static,
    {
        self.pre_render()?;
        component.render(props, self.known_size, self.current_buffer_mut());
        self.post_render()
    }

    pub fn hide_cursor(&mut self) -> io::Result<()>
    where
        B::Error: Send + Sync + 'static,
    {
        self.backend.hide_cursor().map_err(io::Error::other)?;
        self.hidden_cursor = true;
        Ok(())
    }
    pub fn show_cursor(&mut self) -> io::Result<()>
    where
        B::Error: Send + Sync + 'static,
    {
        self.backend.show_cursor().map_err(io::Error::other)?;
        self.hidden_cursor = false;
        Ok(())
    }
    pub fn get_cursor(&mut self) -> io::Result<(u16, u16)>
    where
        B::Error: Send + Sync + 'static,
    {
        let pos = self
            .backend
            .get_cursor_position()
            .map_err(io::Error::other)?;
        Ok((pos.x, pos.y))
    }
    pub fn set_cursor(&mut self, x: u16, y: u16) -> io::Result<()>
    where
        B::Error: Send + Sync + 'static,
    {
        self.backend
            .set_cursor_position(Position { x, y })
            .map_err(io::Error::other)
    }
    pub fn clear(&mut self) -> io::Result<()>
    where
        B::Error: Send + Sync + 'static,
    {
        self.backend.clear().map_err(io::Error::other)
    }
    pub fn size(&self) -> io::Result<Rect>
    where
        B::Error: Send + Sync + 'static,
    {
        self.backend
            .size()
            .map_err(io::Error::other)
            .map(|size| size.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tui::backend::TestBackend;

    #[expect(dead_code)]
    #[derive(Default, Clone)]
    struct ComplexProps {
        x: usize,
        y: String,
    }

    #[derive(Default)]
    struct StatefulComponent {
        x: usize,
    }

    #[derive(Default)]
    struct StatelessComponent;

    impl ToplevelComponent for StatefulComponent {
        type Props = usize;

        fn render(&mut self, props: impl Borrow<Self::Props>, _area: Rect, _buf: &mut Buffer) {
            self.x += *props.borrow();
        }
    }

    impl ToplevelComponent for StatelessComponent {
        type Props = ComplexProps;
        fn render(&mut self, _props: impl Borrow<Self::Props>, _area: Rect, _buf: &mut Buffer) {
            // does not matter - we want to see it compiles essentially
        }
    }

    #[test]
    fn it_does_render_with_simple_and_complex_props() {
        let mut term = Terminal::new(TestBackend::new(20, 20)).unwrap();
        let mut c = StatefulComponent::default();

        term.render(&mut c, 3usize).ok();
        assert_eq!(c.x, 3);

        let mut c = StatelessComponent;
        term.render(&mut c, ComplexProps::default()).ok();
    }
}
