pub trait Parser<T, C, E> {
    fn parse(&self, tokens: Vec<T>) -> Result<C, E>;
}
