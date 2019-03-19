#[derive(Default, Clone)]
pub struct LayoutOptions {
    use_fibre_channel: bool,
    loop_only: bool,
}

impl LayoutOptions {
    pub fn new() -> Self {
        Self {
            use_fibre_channel: false,
            loop_only: false
        }
    }

    pub fn use_fibre_channel(&mut self) -> &mut Self {
        self.use_fibre_channel = true;
        self
    }

    pub fn loop_only(&mut self) -> &mut Self {
        self.loop_only = true;
        self
    }

    pub fn layout(&self) -> Layout {
        Layout {
            options: LayoutOptions {
                loop_only: self.loop_only,
                use_fibre_channel: self.use_fibre_channel
            }
        }
    }
}

pub struct Layout {
    pub options: LayoutOptions
}



#[test]
fn test_builder() {
    let layout = LayoutOptions::new()
        .use_fibre_channel()
        .loop_only()
        .layout();
    assert_eq!(layout.options.loop_only, true);
    assert_eq!(layout.options.use_fibre_channel, true);

    let mut layout_options = LayoutOptions::new();

    layout_options.use_fibre_channel();

    let layout = layout_options.layout();
    assert_eq!(layout.options.loop_only, false);
    assert_eq!(layout.options.use_fibre_channel, true);

}