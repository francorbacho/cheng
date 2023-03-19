use cheng::Board;

mod perft;
use perft::perft;

macro_rules! perft {
    ($perft_name:ident; $($fen:expr => $nodes:expr),* $(,)?) => {
        #[ignore]
        #[test]
        fn $perft_name() {
            cheng::init();
            $(
                let board = Board::from_fen($fen).unwrap();
                assert_eq!(perft(&board, 3), $nodes);
            )*
        }
    };
}

include!("perft_en_masse.txt");
