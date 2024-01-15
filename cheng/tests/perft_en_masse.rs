use cheng::{Board, FromIntoFen};

macro_rules! perft {
    ($perft_name:ident; $($fen:expr => $nodes:expr),* $(,)?) => {
        #[ignore]
        #[test]
        fn $perft_name() {
            cheng::init();
            $(
                let board = Board::from_fen($fen).unwrap();
                assert_eq!(board.perft(3), $nodes);
            )*
        }
    };
}

include!("perft_en_masse.txt");
