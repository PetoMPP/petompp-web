pub struct Extension<T>(pub T);

impl<T> Extension<T> {
    pub fn into(self) -> T {
        self.0
    }
}

#[derive(Clone)]
pub struct ExtensionCl<T: Clone>(pub T);

impl<T: Clone> ExtensionCl<T> {
    pub fn into(self) -> T {
        self.0
    }
}

impl<T: Clone> From<T> for ExtensionCl<T> {
    fn from(value: T) -> Self {
        ExtensionCl(value)
    }
}

