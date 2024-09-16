pub struct NewType<T> {
    pub name: String,
    pub doc: Option<String>,
    pub ty: T,
    pub public: bool,
}
