pub struct Engine<I, W> {
    input: I,
    watcher: W,
}

impl Engine<!, !> {
    pub fn new() -> Engine<(), ()> {
        Engine {
            input: (),
            watcher: (),
        }
    }
}

impl<W> Engine<(), W> {
    pub fn with_input<I>(self, input: I) -> Engine<I, W> {
        Engine {
            input,
            watcher: self.watcher,
        }
    }
}

impl<I> Engine<I, ()> {
    pub fn start(mut self) -> ! {
        loop {
            // if let Ok(Some(_)) = self.touchpad.read_touch() {}
        }
    }

    pub fn with_watcher<W>(self, watcher: W) -> Engine<I, W>
    where
        W: Fn() -> (),
    {
        Engine {
            input: self.input,
            watcher,
        }
    }
}

impl<T, W> Engine<T, W>
where
    W: Fn() -> (),
{
    pub fn start(mut self) -> ! {
        loop {
            // if let Ok(Some(touch)) = self.touchpad.read_touch() {
            //     (self.watcher)(touch)
            // }
        }
    }
}
