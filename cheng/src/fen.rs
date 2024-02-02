pub trait FromIntoFen: Sized {
    type Error;

    fn from_fen(fen: &str) -> Result<Self, Self::Error>;

    #[must_use]
    fn as_fen(&self) -> String;
}
