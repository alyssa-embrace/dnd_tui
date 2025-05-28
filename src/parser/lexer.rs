pub trait Lexer<T, E> {
    fn lex(&self, input: String) -> Result<Vec<T>, E>;
}
