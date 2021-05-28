use rand::Rng;
use crate::print_bitboard;
// MACROS
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
//CONSTS
const NOTAFILE: u64 = 18374403900871474942; // masks giving 1s for all files but the edge files
const NOTHFILE: u64 = 9187201950435737471;
const NOTHGFILE: u64 = 4557430888798830399;
const NOTABFILE: u64 = 18229723555195321596;

pub const ROOKBITS: [u64; 64] =
        [12, 11, 11, 11, 11, 11, 11, 12,
        11, 10, 10, 10, 10, 10, 10, 11,
        11, 10, 10, 10, 10, 10, 10, 11,
        11, 10, 10, 10, 10, 10, 10, 11,
        11, 10, 10, 10, 10, 10, 10, 11,
        11, 10, 10, 10, 10, 10, 10, 11,
        11, 10, 10, 10, 10, 10, 10, 11,
        12, 11, 11, 11, 11, 11, 11, 12];
pub const BISHOPBITS: [u64; 64] =
    [6, 5, 5, 5, 5, 5, 5, 6,
    5, 5, 5, 5, 5, 5, 5, 5,
    5, 5, 7, 7, 7, 7, 5, 5,
    5, 5, 7, 9, 9, 7, 5, 5,
    5, 5, 7, 9, 9, 7, 5, 5,
    5, 5, 7, 7, 7, 7, 5, 5,
    5, 5, 5, 5, 5, 5, 5, 5,
    6, 5, 5, 5, 5, 5, 5, 6];

pub const BISHOPMAGICNUMBERS: [u64; 64] = [
    0x82080801040020,
    0x4010010214204020,
    0x810A10200202020,
    0x484040090081044,
    0x8604242020280108,
    0x108900420000000,
    0x8402010160102404,
    0x2203404404202280,
    0x2030382808280050,
    0x4084C424481E0028,
    0x800041400860480,
    0x40020A0208000C,
    0x3181A20210A10004,
    0x11008040404,
    0xA01010108200400,
    0x1000004124012000,
    0x222001002820800,
    0xC7000200410C283,
    0x1004082080200A,
    0x4402005040104100,
    0x103000811400200,
    0x242860100A000,
    0xC210488A100230,
    0x828A000100808440,
    0x50884024004080C,
    0x8381042660800,
    0x1880900080AC104,
    0x8002002008008120,
    0x8010840000802000,
    0x2112120000480A08,
    0x3000810008880800,
    0x4810404002110C21,
    0x208080454082020,
    0x41AC026000080100,
    0x40A0101080840,
    0x140020080080080,
    0x40004100481100,
    0x401010A00210044,
    0x10224240048400,
    0xC02008100220050,
    0x8010880410004040,
    0x842420110280,
    0x20200220C0400,
    0x80202218012400,
    0x8080104004042,
    0x11210020E208200,
    0x204283801022042,
    0x21C0042000088,
    0x580208A00040,
    0x8220208410080000,
    0x4082002C02080200,
    0x6021010041108004,
    0x404802060410802,
    0x100104408082210,
    0x2012100A08124000,
    0x8082042404104600,
    0x14210440404406E,
    0xC4400A80904,
    0x20424A4608D000,
    0x4C102200020D1400,
    0x301A80004050402,
    0x1080924284682080,
    0x91409001024880,
    0x60080A08005412
];
pub const ROOKMAGICNUMBERS: [u64; 64] = [
    0x1080002080400010,
    0x40004020001000,
    0x10020010040100A,
    0x1000421000A1000,
    0xA002004020010C8,
    0x200080200041001,
    0x1080008002002100,
    0x10010204100008E,
    0x28001A280C004,
    0x4001002040011080,
    0x420802000100082,
    0xA0808008001000,
    0x20800800040080,
    0x14804400800200,
    0x8004008208011004,
    0x4000801D00006180,
    0x1680004000200048,
    0x40048020008040,
    0x4020818010002004,
    0x5080848010008800,
    0x800808004000802,
    0x80D0808002000400,
    0xE0040001080210,
    0x80220000408421,
    0x1000400180006080,
    0x810024440002000,
    0x80F0018280200210,
    0x8002011200204208,
    0x400040080080080,
    0x12000200081004,
    0x1020400881001,
    0x141048A00041241,
    0x900080C001800022,
    0x804000802000,
    0x90080020200400,
    0x320100021000900,
    0x80110005000800,
    0x3034004100400200,
    0x821024004108,
    0x9001408406000041,
    0x3208140028004,
    0x50002004424000,
    0x3201600102B10040,
    0x200010400A020021,
    0x85000800050010,
    0x40002008080,
    0x8400010002008080,
    0x2001010040820004,
    0x8000800041002100,
    0x1900902100400100,
    0x222700020008280,
    0x888009001800880,
    0x100800800040080,
    0x4292000400800280,
    0x4008023110388400,
    0x840010040940200,
    0x2400800010250841,
    0x40050010238141,
    0x1000090040102001,
    0x2082100041001,
    0x12020004A1081002,
    0x8901000804000201,
    0x4008210280104,
    0x200002400410082
];

//JUMP ATACK GEN
fn mask_pawn_attacks(square:u8, side:u8) -> u64 {
    //result bitboard
    let mut attacks = 0u64;
    // piece bitboard
    let mut bitboard = 0u64;

    //set piece on board
    set_bit!(bitboard, square);

    if side == 0 { // white
        attacks |= NOTAFILE & bitboard >> 7;
        attacks |= NOTHFILE & bitboard >> 9;
    }
    else{//black
        attacks |= NOTHFILE & bitboard << 7;
        attacks |= NOTAFILE & bitboard << 9;
    }
    //return attack map
    attacks
}
pub fn init_pawn_attacks() -> [[u64; 64]; 2] {
    let mut pawn_attacks = [[0u64; 64]; 2];
    for square in 0..64 {
        pawn_attacks[0][square] = mask_pawn_attacks(square as u8, 0);
    }
    for square in 0..64 {
        pawn_attacks[1][square] = mask_pawn_attacks(square as u8, 1);
    }
    pawn_attacks
}
fn mask_knight_attacks(square:u8) -> u64 {
    //result bitboard
    let mut attacks = 0u64;
    // piece bitboard
    let mut bitboard = 0u64;

    //set piece on board
    set_bit!(bitboard, square);
    // set mask for all knight attacked squares
    attacks |= NOTAFILE & bitboard >> 15;
    attacks |= NOTHFILE & bitboard >> 17;
    attacks |= NOTHGFILE & bitboard >> 10;
    attacks |= NOTABFILE & bitboard >> 6;
    attacks |= NOTHFILE & bitboard << 15;
    attacks |= NOTAFILE & bitboard << 17;
    attacks |= NOTABFILE & bitboard << 10;
    attacks |= NOTHGFILE & bitboard << 6;

    //return attack map
    attacks
}
pub fn init_knight_attacks() -> [u64; 64] {
    let mut knight_attacks = [0u64; 64];
    for square in 0..64 {
        knight_attacks[square] = mask_knight_attacks(square as u8);
    }
    knight_attacks
}

fn mask_king_attacks(square:u8) -> u64 {
    //result bitboard
    let mut attacks = 0u64;
    // piece bitboard
    let mut bitboard = 0u64;

    //set piece on board
    set_bit!(bitboard, square);
    // set mask for all knight attacked squares
    attacks |= NOTHFILE & bitboard >> 9;
    attacks |= bitboard >> 8;
    attacks |= NOTAFILE & bitboard >> 7;
    attacks |= NOTHFILE & bitboard >> 1;
    attacks |= NOTAFILE & bitboard << 9;
    attacks |= bitboard << 8;
    attacks |= NOTHFILE & bitboard << 7;
    attacks |= NOTAFILE & bitboard << 1;

    //return attack map
    attacks
}
pub fn init_king_attacks() -> [u64; 64] {
    let mut king_attacks = [0u64; 64];
    for square in 0..64 {
        king_attacks[square] = mask_king_attacks(square as u8);
    }
    king_attacks
}

//SLIDER ATTACK GEN

//mask bishop attacks


fn mask_bishop_attacks(square: i8) -> u64 { //crude bishop/rook move gen, just gives all relevant occupancy bits
    //output attacks
    let mut attacks: u64 = 0;

    let targetrank = square / 8;
    let targetfile = square % 8;

    let mut r = targetrank + 1;
    let mut f = targetfile + 1;

    //mask bits
    while r < 7 && f < 7 {
        set_bit!(attacks, (r * 8 + f));
        r += 1;
        f += 1;
    };
    r = targetrank - 1;
    f = targetfile + 1;
    while r > 0 && f < 7 {
        set_bit!(attacks, (r * 8 + f));
        r -= 1;
        f += 1;
    };
    r = targetrank + 1;
    f = targetfile - 1;
    while r < 7 && f > 0 {
        set_bit!(attacks, (r * 8 + f));
        r += 1;
        f -= 1;
    };
    r = targetrank - 1;
    f = targetfile - 1;
    while r > 0 && f > 0 {
        set_bit!(attacks, (r * 8 + f));
        r -= 1;
        f -= 1;
    };

    attacks

}
// mask rook attacks
pub fn mask_rook_attacks(square: i8) -> u64 { //crude bishop/rook move gen,  just gives all relevant occupancy bits
    //output attacks
    let mut attacks: u64 = 0;

    let targetrank = square / 8;
    let targetfile = square % 8;

    let mut r = targetrank + 1;
    let mut f = targetfile;

    //mask bits
    while r < 7{
        set_bit!(attacks, (r * 8 + f));
        r += 1;
    };
    r = targetrank - 1;
    while r > 0{
        set_bit!(attacks, (r * 8 + f));
        r -= 1;
    };
    r = targetrank;
    f = targetfile - 1;
    while f > 0 {
        set_bit!(attacks, (r * 8 + f));
        f -= 1;
    };
    f = targetfile + 1;
    while f < 7 {
        set_bit!(attacks, (r * 8 + f));
        f += 1;
    };
    attacks
}

//atacks on the fly

fn bishop_attacks_on_fly(square: i8, block: u64) -> u64 { //crude bishop/rook move gen, generates attacks given occupancy board
    //output attacks
    //output attacks
    let mut attacks: u64 = 0;

    let targetrank = square / 8;
    let targetfile = square % 8;

    let mut r = targetrank + 1;
    let mut f = targetfile + 1;

    //mask bits
    while r <= 7 && f <= 7 { // continually adds 1 to rank and file, sets the bit, and checks if theres a block.
    // If there is, break the loop and stop iterating
        set_bit!(attacks, (r * 8 + f));
        if get_bit!(block, (r * 8 + f)) == 1 { break }
        r += 1;
        f += 1;
    };
    r = targetrank - 1;
    f = targetfile + 1;
    while r >= 0 && f <= 7 {
        set_bit!(attacks, (r * 8 + f));
        if get_bit!(block, (r * 8 + f)) == 1 { break }
        r -= 1;
        f += 1;
    };
    r = targetrank + 1;
    f = targetfile - 1;
    while r <= 7 && f >= 0 {
        set_bit!(attacks, (r * 8 + f));
        if get_bit!(block, (r * 8 + f)) == 1 { break }
        r += 1;
        f -= 1;
    };
    r = targetrank - 1;
    f = targetfile - 1;
    while r >= 0 && f >= 0 {
        set_bit!(attacks, (r * 8 + f));
        if get_bit!(block, (r * 8 + f)) == 1 { break }
        r -= 1;
        f -= 1;
    };

    attacks

}
// mask rook attacks
fn rook_attacks_on_fly(square: i8, block: u64) -> u64 { //crude bishop/rook move gen,  generates attacks given occupancy board
    //output attacks
    let mut attacks: u64 = 0;

    let targetrank = square / 8;
    let targetfile = square % 8;

    let mut r = targetrank;
    let mut f = targetfile;

    //generate bishop attacks
    r = targetrank + 1;
    while r <= 7{
        set_bit!(attacks, (r * 8 + f));
        if get_bit!(block, (r * 8 + f)) == 1 { break }
        r += 1;
    };
    r = targetrank - 1;
    while r >= 0{
        set_bit!(attacks, (r * 8 + f));
        if get_bit!(block, (r * 8 + f)) == 1 { break }
        r -= 1;
    };
    r = targetrank;
    f = targetfile - 1;
    while f >= 0 {
        set_bit!(attacks, (r * 8 + f));
        if get_bit!(block, (r * 8 + f)) == 1 { break }
        f -= 1;
    };
    f = targetfile + 1;
    while f <= 7 {
        set_bit!(attacks, (r * 8 + f));
        if get_bit!(block, (r * 8 + f)) == 1 { break }
        f += 1;
    };
    attacks
}

pub fn init_slider_attacks() -> Vec<Vec<Vec<u64>>> {
    let mut bishop_masks = vec![0u64; 64];
    let mut rook_masks= vec![0u64; 64];
    let mut rook_attacks = vec![vec![0u64; 4096]; 64];
    let mut bishop_attacks = vec![vec![0u64; 512]; 64];


    for square in 0..64 {
        bishop_masks[square] = mask_bishop_attacks(square as i8);
        rook_masks[square] = mask_rook_attacks(square as i8);


        let bishop_attack_mask = bishop_masks[square];
        let rook_attack_mask = rook_masks[square];
        let b_relevant_bit_count = bishop_attack_mask.count_ones();
        let r_relevant_bit_count = rook_attack_mask.count_ones();

        let occupancy_indicies:usize = 1 << b_relevant_bit_count;

        for index in 0..occupancy_indicies {
            let occupancy = set_occupancy(index as usize, bishop_attack_mask);

            let magic_index = occupancy.wrapping_mul(BISHOPMAGICNUMBERS[square]) >> (64 - BISHOPBITS[square]);

            bishop_attacks[square][magic_index as usize] = bishop_attacks_on_fly(square as i8, occupancy);
        }
        let occupancy_indicies:usize = 1 << r_relevant_bit_count;
        for index in 0..occupancy_indicies {


                let occupancy = set_occupancy(index as usize,  rook_attack_mask);

                let magic_index = occupancy.wrapping_mul(ROOKMAGICNUMBERS[square]) >> (64 - ROOKBITS[square]);

                rook_attacks[square][magic_index as usize] = rook_attacks_on_fly(square as i8, occupancy);

        }


    }
    return vec!(rook_attacks, bishop_attacks)
}










//magic numbers
fn set_occupancy(magic_index: usize, mut attack_mask: u64) -> u64{
    let mut occupancy = 0u64;
    let bitcount = attack_mask.count_ones();
    //loop over bits in mask
    for count in 0..bitcount {
        //get ls1b
        let square = attack_mask.trailing_zeros();
        pop_bit!(attack_mask, square);
        //print_bitboard(attack_mask);

        //make sure occupancy is on board
        if magic_index as u64 & 1u64 << count != 0 {
            //set occupancy map
            set_bit!(occupancy, square);
        }
    }
    occupancy
}
fn get_random_number() -> u64 {
    let mut rng = rand::thread_rng();
    let x: u64 = rng.gen();
    let y: u64 = rng.gen();
    let z: u64 = rng.gen();
    x & y & z
}

fn find_magic_number(square: i8, relevant_bits: u64, bishop: bool) -> u64 {
    //init occupancies
    let mut occupancies = [0u64; 4096];

    //init attack tables
    let mut attacks = [0u64; 4096];

    //init used attacks



    //init attack mask for piece
    let attack_mask = {
        if bishop {mask_bishop_attacks(square)}
        else {mask_rook_attacks(square)}
    };

    //init occupancy indicies
    let occupancy_indicies = 1 << relevant_bits;

    //loop over occupancy indicies
    for index in 0..occupancy_indicies {
        occupancies[index] = set_occupancy(index, attack_mask);

        attacks[index] = {
            if bishop {bishop_attacks_on_fly(square, occupancies[index])}
            else { rook_attacks_on_fly(square, occupancies[index])}
        };
    }


    //test magic numbers
    for _random_count in 0..100000000 {
        // generate candidates
        let magic_number = get_random_number();
        //println!("{}", magic_number);
        //println!("{}       {}         {}", attack_mask.saturating_mul(magic_number), magic_number, attack_mask);

        // skip dumb numbers
        if (((attack_mask.wrapping_mul(magic_number)) & 0xFF00000000000000).count_ones()) < 6 {continue}
        //println!("{}       {}         {}", attack_mask.wrapping_mul(magic_number), magic_number, attack_mask);
        let mut used_attacks = [0u64; 4096];

        //init index and fail
        let mut index: usize = 0;
        let mut fail: bool = false;


        // test magic index

        while !fail && index < occupancy_indicies {
            let magic_index:usize = ((occupancies[index].wrapping_mul(magic_number)) >> (64 - relevant_bits) as usize) as usize;
            //println!("{}", occupancies[index]);
            //println!("{}", magic_index);
            //println!("{}", magic_index);

            if (used_attacks[magic_index] == 0) {
                used_attacks[magic_index] = attacks[index];
            }
            else if (used_attacks[magic_index] != attacks[index]) {
                fail = true;
            }

            index += 1;

        }
        if !fail {
            return magic_number}
    }
    println!("NUMBER BROKEY");
    return 0u64;
}

fn init_magic_numbers() {
    // loop for 64 board squares
    for square in 0..64 {
        let magic = find_magic_number(square, BISHOPBITS[square as usize], true);
        println!("0x{},", format!("{:X}", magic));
        //println!("{}", magic);
    }
    println!();
    println!();
    for square in 0..64 {
        let magic = find_magic_number(square, ROOKBITS[square as usize], false);
        println!("0x{},", format!("{:X}", magic));
        //println!("{}", magic);
    }
}

