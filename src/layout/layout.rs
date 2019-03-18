#[derive(Default)]
pub struct LayoutOptions {
    use_fibre_channel: bool,
    loop_only: bool,
}

impl LayoutOptions {
    pub fn new() -> Self {
        LayoutOptions {
            use_fibre_channel: false,
            loop_only: false
        }
    }

    pub fn use_fibre_channel<'a>(&'a mut self) -> &'a mut Self {
        self.use_fibre_channel = true;
        self
    }

    pub fn loop_only<'a>(&'a mut self) -> &'a mut Self {
        self.loop_only = true;
        self
    }
}


#[test]
fn test_builder() {
    let layout_options = LayoutOptions::new()
        .loop_only()
        .use_fibre_channel();
    assert_eq!(&layout_options.loop_only, &true);
    assert_eq!(&layout_options.use_fibre_channel, &true);
}