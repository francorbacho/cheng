pub trait FromIntoFen: Sized {
    type Error;

    #[must_use]
    fn from_fen(fen: &str) -> Result<Self, Self::Error>;

    #[must_use]
    fn into_fen(&self) -> String;
}
