// https://github.com/rust-lang/rust/issues/70142
pub trait FlattenResult<T, E> {
    fn flatten_result(self) -> Result<T, E>;
}

impl<T, E> FlattenResult<T, E> for Result<Result<T, E>, E> {
    fn flatten_result(self) -> Result<T, E> {
        self.and_then(std::convert::identity)
    }
}
