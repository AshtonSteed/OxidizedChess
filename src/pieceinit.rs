use rand::Rng;
use crate::piececonstants;


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

            let magic_index = occupancy.wrapping_mul(piececonstants::BISHOPMAGICNUMBERS[square]) >> (64 - BISHOPBITS[square]);

            bishop_attacks[square][magic_index as usize] = bishop_attacks_on_fly(square as i8, occupancy);
        }
        let occupancy_indicies:usize = 1 << r_relevant_bit_count;
        for index in 0..occupancy_indicies {


                let occupancy = set_occupancy(index as usize,  rook_attack_mask);

                let magic_index = occupancy.wrapping_mul(piececonstants::ROOKMAGICNUMBERS[square]) >> (64 - ROOKBITS[square]);

                rook_attacks[square][magic_index as usize] = rook_attacks_on_fly(square as i8, occupancy);

        }


    }
    return vec!(rook_attacks, bishop_attacks)
}


pub fn init_slider_attacks2() -> (Vec<u64>, Vec<usize>, Vec<usize>) { //didnt work : (
    let mut bishop_masks = vec![0u64; 64];
    let mut rook_masks= vec![0u64; 64];
    let mut slider_attacks:Vec<u64> = vec![];
    let mut rook_pointers:Vec<usize> = vec![0usize; 65];
    let mut bishop_pointers:Vec<usize> = vec![0usize; 65];


    for square in 0..64 {
        bishop_masks[square] = mask_bishop_attacks(square as i8);
        rook_masks[square] = mask_rook_attacks(square as i8);

        let bishop_attack_mask = bishop_masks[square];
        let rook_attack_mask = rook_masks[square];
        let b_relevant_bit_count = bishop_attack_mask.count_ones();
        let r_relevant_bit_count = rook_attack_mask.count_ones();

        let b_occupancy_indicies:usize = 1 << b_relevant_bit_count;
        let r_occupancy_indicies:usize = 1 << r_relevant_bit_count;

        let mut max_bishop_index = 0;
        let mut max_rook_index = 0;

        for index in 0..r_occupancy_indicies {


            let occupancy = set_occupancy(index as usize,  rook_attack_mask);

            let magic_index = occupancy.wrapping_mul(piececonstants::ROOKMAGICNUMBERS[square]) >> (64 - ROOKBITS[square]);
            if magic_index > max_rook_index { max_rook_index = magic_index}

            slider_attacks.push(rook_attacks_on_fly(square as i8, occupancy));

        }

        bishop_pointers[square] = rook_pointers[square] + max_rook_index as usize;

        for index in 0..b_occupancy_indicies {
            let occupancy = set_occupancy(index as usize, bishop_attack_mask);

            let magic_index = occupancy.wrapping_mul(piececonstants::BISHOPMAGICNUMBERS[square]) >> (64 - BISHOPBITS[square]);
            if magic_index > max_bishop_index { max_bishop_index = magic_index}

            slider_attacks.push(bishop_attacks_on_fly(square as i8, occupancy));
        };

        rook_pointers[square + 1] = bishop_pointers[square] + max_bishop_index as usize;

    }
    return (slider_attacks, rook_pointers, bishop_pointers)
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

