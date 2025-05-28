pub trait Lexer<T> {
    fn lex(&self, input: String) -> Vec<T>;
}
