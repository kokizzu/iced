pub use iced_core as core;

use crate::core::theme;
use crate::core::window;

pub use internal::Span;

pub fn init(name: &str) {
    internal::init(name);
}

pub fn open_comet() {}

pub fn log_message(_message: &impl std::fmt::Debug) {}

pub fn theme_changed(f: impl FnOnce() -> Option<theme::Palette>) {
    internal::theme_changed(f);
}

pub fn boot() -> Span {
    internal::boot()
}

pub fn update() -> Span {
    internal::update()
}

pub fn view(window: window::Id) -> Span {
    internal::view(window)
}

pub fn layout(window: window::Id) -> Span {
    internal::layout(window)
}

pub fn interact(window: window::Id) -> Span {
    internal::interact(window)
}

pub fn draw(window: window::Id) -> Span {
    internal::draw(window)
}

pub fn present(window: window::Id) -> Span {
    internal::present(window)
}

pub fn time(window: window::Id, name: impl AsRef<str>) -> Span {
    internal::time(window, name)
}

pub fn skip_next_timing() {
    internal::skip_next_timing();
}

#[cfg(feature = "enable")]
mod internal {
    use crate::core::theme;
    use crate::core::time::Instant;
    use crate::core::window;

    use iced_beacon as beacon;

    use beacon::client::{self, Client};
    use beacon::span;

    use once_cell::sync::Lazy;
    use std::sync::atomic::{self, AtomicBool};
    use std::sync::RwLock;

    pub fn init(name: &str) {
        name.clone_into(&mut NAME.write().expect("Write application name"));
    }

    pub fn theme_changed(f: impl FnOnce() -> Option<theme::Palette>) {
        let Some(palette) = f() else {
            return;
        };

        if LAST_PALETTE.read().expect("Read last palette").as_ref()
            != Some(&palette)
        {
            BEACON.log(client::Event::ThemeChanged(palette));

            *LAST_PALETTE.write().expect("Write last palette") = Some(palette);
        }
    }

    pub fn boot() -> Span {
        span(span::Stage::Boot)
    }

    pub fn update() -> Span {
        span(span::Stage::Update)
    }

    pub fn view(window: window::Id) -> Span {
        span(span::Stage::View(window))
    }

    pub fn layout(window: window::Id) -> Span {
        span(span::Stage::Layout(window))
    }

    pub fn interact(window: window::Id) -> Span {
        span(span::Stage::Interact(window))
    }

    pub fn draw(window: window::Id) -> Span {
        span(span::Stage::Draw(window))
    }

    pub fn present(window: window::Id) -> Span {
        span(span::Stage::Present(window))
    }

    pub fn time(window: window::Id, name: impl AsRef<str>) -> Span {
        span(span::Stage::Custom(window, name.as_ref().to_owned()))
    }

    pub fn skip_next_timing() {
        SKIP_NEXT_SPAN.store(true, atomic::Ordering::Relaxed);
    }

    fn span(span: span::Stage) -> Span {
        BEACON.log(client::Event::SpanStarted(span.clone()));

        Span {
            span,
            start: Instant::now(),
        }
    }

    #[derive(Debug)]
    pub struct Span {
        span: span::Stage,
        start: Instant,
    }

    impl Span {
        pub fn finish(self) {
            if SKIP_NEXT_SPAN.fetch_and(false, atomic::Ordering::Relaxed) {
                return;
            }

            BEACON.log(client::Event::SpanFinished(
                self.span,
                self.start.elapsed(),
            ));
        }
    }

    static BEACON: Lazy<Client> = Lazy::new(|| {
        client::connect(NAME.read().expect("Read application name").to_owned())
    });

    static NAME: RwLock<String> = RwLock::new(String::new());
    static LAST_PALETTE: RwLock<Option<theme::Palette>> = RwLock::new(None);
    static SKIP_NEXT_SPAN: AtomicBool = AtomicBool::new(false);
}

#[cfg(not(feature = "enable"))]
mod internal {
    use crate::core::theme;
    use crate::core::window;

    pub fn init(_name: &str) {}

    pub fn theme_changed(_f: impl FnOnce() -> Option<theme::Palette>) {}

    pub fn boot() -> Span {
        Span
    }

    pub fn update() -> Span {
        Span
    }

    pub fn view(_window: window::Id) -> Span {
        Span
    }

    pub fn layout(_window: window::Id) -> Span {
        Span
    }

    pub fn interact(_window: window::Id) -> Span {
        Span
    }

    pub fn draw(_window: window::Id) -> Span {
        Span
    }

    pub fn present(_window: window::Id) -> Span {
        Span
    }

    pub fn time(_window: window::Id, _name: impl AsRef<str>) -> Span {
        Span
    }

    pub fn skip_next_timing() {}

    #[derive(Debug)]
    pub struct Span;

    impl Span {
        pub fn finish(self) {}
    }
}
