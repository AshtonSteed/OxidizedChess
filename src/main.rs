mod pieceinit;
//                                          enums and constants
/*const NOTAFILE: u64 = 18374403900871474942; // masks giving 1s for all files but the edge files
const NOTHFILE: u64 = 9187201950435737471; //probably not needed in this file
const NOTHGFILE: u64 = 4557430888798830399;
const NOTABFILE: u64 = 18229723555195321596;*/

enum Square {
    A8, B8, C8, D8, E8, F8, G8, H8,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A1, B1, C1, D1, E1, F1, G1, H1, }
enum Color { WHITE, BLACK,}





//                                      bit manipulations

//          Bit macros
macro_rules! get_bit { //returns either 1 or 0, depending on if the square is a active bit
    ($bb:expr, $square:expr) => {
        if ($bb & 1u64 << $square) != 0 {1} else {0} // if statement checks if the and operator
        // between bitboard and square is non zero, checks and returns square bit value
    }
}
macro_rules! set_bit {//sets a bit on a board to a 1
    ($bb:expr, $square:expr) => {
        $bb |= 1u64 <<$square // takes the or between the bitboard the the shifted square number
    }
}
macro_rules! pop_bit {//sets a bit on a board to a 0
    ($bb:expr, $square:expr) => {
        $bb &= !(1u64 <<$square) // takes the nand between the bitboard and the shifted square
    }
}



fn print_bitboard(bitboard: u64) -> (){ //prints a bitboard
    println!();
    for rank in 0..8 {
        for file in 0..8 {
            // init board square, turn file and rank into square
            let square = rank * 8 + file;
            if file == 0 {
                print!("{}  ", 8 - rank);
            }
            //println!("{}", bitboard & 1u64 << square);
            print!("{} ", get_bit!(bitboard, square));
        }
        //print new line to seperate ranks
        println!();
    }
    println!("   a b c d e f g h");

    println!("Bitboard Value: {}", bitboard)

}

//                                              attacks

//                                              main driver











fn main() {
    let pawn_attacks = pieceinit::init_pawn_attacks();
    let knight_attacks = pieceinit::init_knight_attacks();
    let king_attacks = pieceinit::init_king_attacks();


    print_bitboard(king_attacks[Square::D5 as usize])
    //bitboard = mask_pawn_attacks(Square::A4 as u8, Color::WHITE as u8);
    //print_bitboard(bitboard);


}
