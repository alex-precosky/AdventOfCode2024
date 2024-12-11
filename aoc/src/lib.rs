pub mod aoc {
    use core::fmt;

    #[derive(Debug)]
    pub enum AocError {
        ElfFellOffTheShelf,
        ReindeerMalfunction,
        ParseError,
    }

    pub type AocResult<T> = Result<T, AocError>;

    impl std::error::Error for AocError {}

    impl fmt::Display for AocError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                AocError::ElfFellOffTheShelf => write!(f, "Elf not on the shelf"),
                AocError::ReindeerMalfunction => write!(f, "Rudolf needs a nose lightbulb change"),
                AocError::ParseError => write!(f, "Parse Error"),
            }
        }
    }
}
