use logos::Logos;

#[repr(u8)]
#[derive(Debug, Clone, Copy, Logos, PartialEq)]
pub enum Token {
    #[token("+")]
    Add = 1,

    #[token("-")]
    Subtract = 2,

    #[token("*")]
    Multiply = 3,

    #[token("/")]
    Divide = 4,

    #[regex(r"-?[0-9]+", |lex| lex.slice().parse().ok())]
    Number(i64),

    #[token("dup")]
    Clone,

    #[token(".")]
    Print,

    #[token("..")]
    Println,

    #[token("if")]
    IF,

    #[token("else")]
    ELSE,

    #[token("end")]
    END,

    #[token("<")]
    Lt,

    #[token(">")]
    Gt,

    #[token("==")]
    Eq,

    #[token("!=")]
    NEq,

    #[token(">=")]
    GtEq,

    #[token("<=")]
    LtEq,

    #[regex(r"[ \t\r\n]")]
    Skip,
}
