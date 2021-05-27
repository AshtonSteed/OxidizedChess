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

const ROOKBITS: [u64; 64] =
        [12, 11, 11, 11, 11, 11, 11, 12,
        11, 10, 10, 10, 10, 10, 10, 11,
        11, 10, 10, 10, 10, 10, 10, 11,
        11, 10, 10, 10, 10, 10, 10, 11,
        11, 10, 10, 10, 10, 10, 10, 11,
        11, 10, 10, 10, 10, 10, 10, 11,
        11, 10, 10, 10, 10, 10, 10, 11,
        12, 11, 11, 11, 11, 11, 11, 12];
const BISHOPBITS: [u64; 64] =
    [6, 5, 5, 5, 5, 5, 5, 6,
    5, 5, 5, 5, 5, 5, 5, 5,
    5, 5, 7, 7, 7, 7, 5, 5,
    5, 5, 7, 9, 9, 7, 5, 5,
    5, 5, 7, 9, 9, 7, 5, 5,
    5, 5, 7, 7, 7, 7, 5, 5,
    5, 5, 5, 5, 5, 5, 5, 5,
    6, 5, 5, 5, 5, 5, 5, 6];

const BISHOPMAGICNUMBERS: [u64; 64] = [0x40080821404004,
    0x400042080031008,
    0x22404000524000,
    0x28C060800104400,
    0x802200102400000,
    0x33604C000344C210,
    0x821049810410001,
    0x820000021841044,
    0x90652C0004180024,
    0x44DC0800801012,
    0x20000100468000C1,
    0x100100A060880000,
    0x148048500,
    0x45C000A100204880,
    0x8000210000E200,
    0x205000C048102012,
    0x7049421400402,
    0x102020A80082199,
    0x500121000240C,
    0x4102200120000,
    0x10200800440040,
    0x308600002081800,
    0x8A00003A2040221,
    0x4000800060300028,
    0x900010492041900,
    0x5A00150003C28000,
    0xCA22208020110000,
    0x1000000810404890,
    0x1624022100415802,
    0x892B020010002140,
    0xC0050002A20D0,
    0x50000020E8000,
    0x790000321000A,
    0x200004500000920,
    0x2000007048009100,
    0x4021C01002020,
    0x800100000CA000,
    0x800000A9108424,
    0x400064008900,
    0x3400600910004,
    0x4008020882001420,
    0x1200441805040840,
    0x8087080004000221,
    0x470602002002084,
    0x4010000008A01000,
    0x1000024890102220,
    0x20008004A0890100,
    0x6010230190400020,
    0x2440007502020040,
    0x400001402A8000,
    0x228B22004018080,
    0x20000104010000,
    0x121000102020900,
    0x1104C28000031D00,
    0x10020030C60204,
    0x9223080080004E00,
    0x4600040200402B08,
    0x1401600D81000702,
    0x2204120002820,
    0x2041820000809080,
    0x802208108020,
    0x4440000228881010,
    0x2300100008C2,
    0x8080088224A004 ];
const ROOKMAGICNUMBERS: [u64; 64] = [0x8024050000604240,
    0x8400201020080,
    0x2104404800102020,
    0x4910040000A0050,
    0x402000220000010,
    0x1001448082100084,
    0x6052220200281,
    0x481010425023020,
    0x8A8003C080180102,
    0xC00400040000,
    0x1030004600186000,
    0x40044002010038,
    0x1002D0004400D18,
    0x52240260200,
    0x100000AA09420101,
    0x404011052402400,
    0x100840431020A040,
    0x3C80A09024002408,
    0x102812820000000,
    0xE21000C0010010,
    0x80000C48001200C,
    0x100000084000,
    0x4010010100448001,
    0x1808060000000600,
    0x1081000C02008081,
    0x20000209106800,
    0x1000030188004000,
    0x400080000400710,
    0x2000004040000000,
    0x8241040298020,
    0xA100A00003000,
    0x80001008200400,
    0x80600101000000,
    0x908DE00000807001,
    0x408080092400018,
    0x500532600000402,
    0x400000002010028,
    0x80A0008000409E1,
    0x184020012001040,
    0x4020A80420880820,
    0x120008800A022074,
    0x4000020604808,
    0x200010100600400,
    0x828082000501010,
    0x2241001004484904,
    0x12880000102200,
    0x400000002020000,
    0x20A0A8020408,
    0x5040824004004,
    0x1001100400217090,
    0x1180000140141000,
    0x200000A08085201,
    0x2002000440,
    0x1000025C01000005,
    0x6000C000801006C,
    0x100400C080092110,
    0x400030000000008,
    0x800000031000001,
    0x13900440010120,
    0x2114010080400,
    0x4001000000320508,
    0x2141088080101421,
    0x121000004882002,
    0x1090A0284000006
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
fn mask_rook_attacks(square: i8) -> u64 { //crude bishop/rook move gen,  just gives all relevant occupancy bits
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


//magic numbers
fn set_occupancy(magic_index: usize, mut attack_mask: u64) -> u64{
    let mut occupancy = 0u64;
    let bitcount = attack_mask.count_ones();
    //loop over bits in mask
    for count in 0..bitcount {
        //get ls1b
        let square = attack_mask.trailing_zeros();
        pop_bit!(attack_mask, square);

        //make sure occupancy is on board
        if magic_index != 0 && (1 << count) != 0 {
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

pub fn init_magic_numbers() {
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

/* TODO: Check occupancy generation, Check magic numbers */