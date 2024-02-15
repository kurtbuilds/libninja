
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Visibility {
    Public,
    Crate,
    Private,
}


impl Visibility {
    pub fn public(&self) -> bool {
        match self {
            Visibility::Public => true,
            Visibility::Crate => false,
            Visibility::Private => false,
        }
    }
}

impl Default for Visibility {
    fn default() -> Self {
        Self::Private
    }
}
