pub enum Strings {
    UiFont,
}

impl Strings {
    pub fn path(&self) -> &'static str {
        match self {
            Strings::UiFont => "/resources/fonts/Comfortaa-Regular.ttf",
        }
    }
}
