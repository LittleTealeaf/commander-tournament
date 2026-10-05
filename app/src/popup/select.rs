#[derive(Debug, Clone, derive_more::Constructor)]
pub struct Select<T> {
    title: String,
    options: Vec<T>,
}
