lazy_static! { //allows me to use this stuff as statics, neat
    static ref SLIDER_ATTACKS: Vec<u64>= crate::pieceinit::init_slider_attacks2().0;
}

pub fn get_bishop_attacks(square: usize, mut occupancy: u64) -> u64 {
    occupancy &= BISHOP_MASKS[square];
    occupancy = occupancy.wrapping_mul(BISHOPMAGICNUMBERS[square]);
    occupancy >>= 64 - BISHOPBITS[square];

    SLIDER_ATTACKS[BISHOP_POINTERS[square] + occupancy as usize]
}

pub fn get_rook_attacks(square: usize, mut occupancy: u64) -> u64 {
    occupancy &= ROOK_MASKS[square];
    occupancy = occupancy.wrapping_mul(ROOKMAGICNUMBERS[square]);
    occupancy >>= 64 - ROOKBITS[square];

    SLIDER_ATTACKS[ROOK_POINTERS[square] + occupancy as usize]
}

pub fn get_queen_attacks(square: usize, occupancy: u64) -> u64 {
    let mut bishop_occupancy = occupancy;

    let mut rook_occupancy = occupancy;

    bishop_occupancy &= BISHOP_MASKS[square];
    bishop_occupancy = bishop_occupancy.wrapping_mul(BISHOPMAGICNUMBERS[square]);
    bishop_occupancy >>= 64 - BISHOPBITS[square];

    rook_occupancy &= ROOK_MASKS[square];
    rook_occupancy = rook_occupancy.wrapping_mul(ROOKMAGICNUMBERS[square]);
    rook_occupancy >>= 64 - ROOKBITS[square];

    SLIDER_ATTACKS[BISHOP_POINTERS[square] + bishop_occupancy as usize]
        | SLIDER_ATTACKS[ROOK_POINTERS[square] + rook_occupancy as usize]
    // returns rook + bishop attacks from square
}

pub enum Color {
    WHITE,
    BLACK,
}
pub enum Rook_Bishop {
    ROOK,
    BISHOP,
}
pub enum Castling {
    wk = 1,
    wq = 2,
    bk = 4,
    bq = 8,
}
pub enum Pieces {
    P, // white pawn
    N, // white night
    B, // white bishop
    R, // white rook
    Q, // white queen
    K, // white king
    p, // black pawnn
    n, // black knight
    b, // black bishop
    r, // black rook
    q, // black queen
    k, // black king
}
pub enum Square {
    //helper enums for squares and color to move, nothing too fancy
    A8,
    B8,
    C8,
    D8,
    E8,
    F8,
    G8,
    H8,
    A7,
    B7,
    C7,
    D7,
    E7,
    F7,
    G7,
    H7,
    A6,
    B6,
    C6,
    D6,
    E6,
    F6,
    G6,
    H6,
    A5,
    B5,
    C5,
    D5,
    E5,
    F5,
    G5,
    H5,
    A4,
    B4,
    C4,
    D4,
    E4,
    F4,
    G4,
    H4,
    A3,
    B3,
    C3,
    D3,
    E3,
    F3,
    G3,
    H3,
    A2,
    B2,
    C2,
    D2,
    E2,
    F2,
    G2,
    H2,
    A1,
    B1,
    C1,
    D1,
    E1,
    F1,
    G1,
    H1,
    NOSQ,
}

pub const ASCII_PIECES: [char; 12] = ['P', 'N', 'B', 'R', 'Q', 'K', 'p', 'n', 'b', 'r', 'q', 'k'];

pub const UNICODE_PIECES: [char; 12] = ['♟', '♞', '♝', '♜', '♛', '♚', '♙', '♘', '♗', '♖', '♕', '♔']; // inverts the correct unicode stuff, black looks white and white looks black so tuff

pub const SQUARE_TO_COORDINATES: [&str; 65] = [
    "A8", "B8", "C8", "D8", "E8", "F8", "G8", "H8", "A7", "B7", "C7", "D7", "E7", "F7", "G7", "H7",
    "A6", "B6", "C6", "D6", "E6", "F6", "G6", "H6", "A5", "B5", "C5", "D5", "E5", "F5", "G5", "H5",
    "A4", "B4", "C4", "D4", "E4", "F4", "G4", "H4", "A3", "B3", "C3", "D3", "E3", "F3", "G3", "H3",
    "A2", "B2", "C2", "D2", "E2", "F2", "G2", "H2", "A1", "B1", "C1", "D1", "E1", "F1", "G1", "H1",
    "NA",
];

pub const ROOK_MASKS: [u64; 64] = [
    282578800148862,
    565157600297596,
    1130315200595066,
    2260630401190006,
    4521260802379886,
    9042521604759646,
    18085043209519166,
    36170086419038334,
    282578800180736,
    565157600328704,
    1130315200625152,
    2260630401218048,
    4521260802403840,
    9042521604775424,
    18085043209518592,
    36170086419037696,
    282578808340736,
    565157608292864,
    1130315208328192,
    2260630408398848,
    4521260808540160,
    9042521608822784,
    18085043209388032,
    36170086418907136,
    282580897300736,
    565159647117824,
    1130317180306432,
    2260632246683648,
    4521262379438080,
    9042522644946944,
    18085043175964672,
    36170086385483776,
    283115671060736,
    565681586307584,
    1130822006735872,
    2261102847592448,
    4521664529305600,
    9042787892731904,
    18085034619584512,
    36170077829103616,
    420017753620736,
    699298018886144,
    1260057572672512,
    2381576680245248,
    4624614895390720,
    9110691325681664,
    18082844186263552,
    36167887395782656,
    35466950888980736,
    34905104758997504,
    34344362452452352,
    33222877839362048,
    30979908613181440,
    26493970160820224,
    17522093256097792,
    35607136465616896,
    9079539427579068672,
    8935706818303361536,
    8792156787827803136,
    8505056726876686336,
    7930856604974452736,
    6782456361169985536,
    4485655873561051136,
    9115426935197958144,
];

pub const BISHOP_MASKS: [u64; 64] = [
    18049651735527936,
    70506452091904,
    275415828992,
    1075975168,
    38021120,
    8657588224,
    2216338399232,
    567382630219776,
    9024825867763712,
    18049651735527424,
    70506452221952,
    275449643008,
    9733406720,
    2216342585344,
    567382630203392,
    1134765260406784,
    4512412933816832,
    9024825867633664,
    18049651768822272,
    70515108615168,
    2491752130560,
    567383701868544,
    1134765256220672,
    2269530512441344,
    2256206450263040,
    4512412900526080,
    9024834391117824,
    18051867805491712,
    637888545440768,
    1135039602493440,
    2269529440784384,
    4539058881568768,
    1128098963916800,
    2256197927833600,
    4514594912477184,
    9592139778506752,
    19184279556981248,
    2339762086609920,
    4538784537380864,
    9077569074761728,
    562958610993152,
    1125917221986304,
    2814792987328512,
    5629586008178688,
    11259172008099840,
    22518341868716544,
    9007336962655232,
    18014673925310464,
    2216338399232,
    4432676798464,
    11064376819712,
    22137335185408,
    44272556441600,
    87995357200384,
    35253226045952,
    70506452091904,
    567382630219776,
    1134765260406784,
    2832480465846272,
    5667157807464448,
    11333774449049600,
    22526811443298304,
    9024825867763712,
    18049651735527936,
];

pub const ROOK_RAW_ATTACKS: [u64; 64] = [
    72340172838076926,
    144680345676153597,
    289360691352306939,
    578721382704613623,
    1157442765409226991,
    2314885530818453727,
    4629771061636907199,
    9259542123273814143,
    72340172838141441,
    144680345676217602,
    289360691352369924,
    578721382704674568,
    1157442765409283856,
    2314885530818502432,
    4629771061636939584,
    9259542123273813888,
    72340172854657281,
    144680345692602882,
    289360691368494084,
    578721382720276488,
    1157442765423841296,
    2314885530830970912,
    4629771061645230144,
    9259542123273748608,
    72340177082712321,
    144680349887234562,
    289360695496279044,
    578721386714368008,
    1157442769150545936,
    2314885534022901792,
    4629771063767613504,
    9259542123257036928,
    72341259464802561,
    144681423712944642,
    289361752209228804,
    578722409201797128,
    1157443723186933776,
    2314886351157207072,
    4629771607097753664,
    9259542118978846848,
    72618349279904001,
    144956323094725122,
    289632270724367364,
    578984165983651848,
    1157687956502220816,
    2315095537539358752,
    4629910699613634624,
    9259541023762186368,
    143553341945872641,
    215330564830528002,
    358885010599838724,
    645993902138460168,
    1220211685215703056,
    2368647251370188832,
    4665518383679160384,
    9259260648297103488,
    18302911464433844481,
    18231136449196065282,
    18087586418720506884,
    17800486357769390088,
    17226286235867156496,
    16077885992062689312,
    13781085504453754944,
    9187484529235886208,
];

pub const BISHOP_RAW_ATTACKS: [u64; 64] = [
    9241421688590303744,
    36099303471056128,
    141012904249856,
    550848566272,
    6480472064,
    1108177604608,
    283691315142656,
    72624976668147712,
    4620710844295151618,
    9241421688590368773,
    36099303487963146,
    141017232965652,
    1659000848424,
    283693466779728,
    72624976676520096,
    145249953336262720,
    2310355422147510788,
    4620710844311799048,
    9241421692918565393,
    36100411639206946,
    424704217196612,
    72625527495610504,
    145249955479592976,
    290499906664153120,
    1155177711057110024,
    2310355426409252880,
    4620711952330133792,
    9241705379636978241,
    108724279602332802,
    145390965166737412,
    290500455356698632,
    580999811184992272,
    577588851267340304,
    1155178802063085600,
    2310639079102947392,
    4693335752243822976,
    9386671504487645697,
    326598935265674242,
    581140276476643332,
    1161999073681608712,
    288793334762704928,
    577868148797087808,
    1227793891648880768,
    2455587783297826816,
    4911175566595588352,
    9822351133174399489,
    1197958188344280066,
    2323857683139004420,
    144117404414255168,
    360293502378066048,
    720587009051099136,
    1441174018118909952,
    2882348036221108224,
    5764696068147249408,
    11529391036782871041,
    4611756524879479810,
    567382630219904,
    1416240237150208,
    2833579985862656,
    5667164249915392,
    11334324221640704,
    22667548931719168,
    45053622886727936,
    18049651735527937,
];

pub const ROOKBITS: [u64; 64] = [
    12, 11, 11, 11, 11, 11, 11, 12, 11, 10, 10, 10, 10, 10, 10, 11, 11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11, 11, 10, 10, 10, 10, 10, 10, 11, 11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11, 12, 11, 11, 11, 11, 11, 11, 12,
];
pub const BISHOPBITS: [u64; 64] = [
    6, 5, 5, 5, 5, 5, 5, 6, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 7, 7, 7, 7, 5, 5, 5, 5, 7, 9, 9, 7, 5, 5,
    5, 5, 7, 9, 9, 7, 5, 5, 5, 5, 7, 7, 7, 7, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 6, 5, 5, 5, 5, 5, 5, 6,
];

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
    0x60080A08005412,
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
    0x200002400410082,
];

pub const ROOK_POINTERS: [usize; 64] = [
    0, 4160, 6240, 8320, 10400, 12480, 14560, 16640, 20800, 22880, 23936, 24992, 26048, 27104,
    28160, 29216, 31296, 33376, 34432, 35584, 36736, 37888, 39040, 40096, 42176, 44256, 45312,
    46464, 48000, 49536, 50688, 51744, 53824, 55904, 56960, 58112, 59648, 61184, 62336, 63392,
    65472, 67552, 68608, 69760, 70912, 72064, 73216, 74272, 76352, 78432, 79488, 80544, 81600,
    82656, 83712, 84768, 86848, 91008, 93088, 95168, 97248, 99328, 101408, 103488,
];

pub const BISHOP_POINTERS: [usize; 64] = [
    4096, 6208, 8288, 10368, 12448, 14528, 16608, 20736, 22848, 23904, 24960, 26016, 27072, 28128,
    29184, 31264, 33344, 34400, 35456, 36608, 37760, 38912, 40064, 42144, 44224, 45280, 46336,
    47488, 49024, 50560, 51712, 53792, 55872, 56928, 57984, 59136, 60672, 62208, 63360, 65440,
    67520, 68576, 69632, 70784, 71936, 73088, 74240, 76320, 78400, 79456, 80512, 81568, 82624,
    83680, 84736, 86816, 90944, 93056, 95136, 97216, 99296, 101376, 103456, 107584,
];

pub const KNIGHT_ATTACKS: [u64; 64] = [
    132096,
    329728,
    659712,
    1319424,
    2638848,
    5277696,
    10489856,
    4202496,
    33816580,
    84410376,
    168886289,
    337772578,
    675545156,
    1351090312,
    2685403152,
    1075839008,
    8657044482,
    21609056261,
    43234889994,
    86469779988,
    172939559976,
    345879119952,
    687463207072,
    275414786112,
    2216203387392,
    5531918402816,
    11068131838464,
    22136263676928,
    44272527353856,
    88545054707712,
    175990581010432,
    70506185244672,
    567348067172352,
    1416171111120896,
    2833441750646784,
    5666883501293568,
    11333767002587136,
    22667534005174272,
    45053588738670592,
    18049583422636032,
    145241105196122112,
    362539804446949376,
    725361088165576704,
    1450722176331153408,
    2901444352662306816,
    5802888705324613632,
    11533718717099671552,
    4620693356194824192,
    288234782788157440,
    576469569871282176,
    1224997833292120064,
    2449995666584240128,
    4899991333168480256,
    9799982666336960512,
    1152939783987658752,
    2305878468463689728,
    1128098930098176,
    2257297371824128,
    4796069720358912,
    9592139440717824,
    19184278881435648,
    38368557762871296,
    4679521487814656,
    9077567998918656,
];

pub const KING_ATTACKS: [u64; 64] = [
    770,
    1797,
    3594,
    7188,
    14376,
    28752,
    57504,
    49216,
    197123,
    460039,
    920078,
    1840156,
    3680312,
    7360624,
    14721248,
    12599488,
    50463488,
    117769984,
    235539968,
    471079936,
    942159872,
    1884319744,
    3768639488,
    3225468928,
    12918652928,
    30149115904,
    60298231808,
    120596463616,
    241192927232,
    482385854464,
    964771708928,
    825720045568,
    3307175149568,
    7718173671424,
    15436347342848,
    30872694685696,
    61745389371392,
    123490778742784,
    246981557485568,
    211384331665408,
    846636838289408,
    1975852459884544,
    3951704919769088,
    7903409839538176,
    15806819679076352,
    31613639358152704,
    63227278716305408,
    54114388906344448,
    216739030602088448,
    505818229730443264,
    1011636459460886528,
    2023272918921773056,
    4046545837843546112,
    8093091675687092224,
    16186183351374184448,
    13853283560024178688,
    144959613005987840,
    362258295026614272,
    724516590053228544,
    1449033180106457088,
    2898066360212914176,
    5796132720425828352,
    11592265440851656704,
    4665729213955833856,
];

pub const PAWN_ATTACKS: [[u64; 64]; 2] = [
    //[side(0 for white)][square]
    [
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        2,
        5,
        10,
        20,
        40,
        80,
        160,
        64,
        512,
        1280,
        2560,
        5120,
        10240,
        20480,
        40960,
        16384,
        131072,
        327680,
        655360,
        1310720,
        2621440,
        5242880,
        10485760,
        4194304,
        33554432,
        83886080,
        167772160,
        335544320,
        671088640,
        1342177280,
        2684354560,
        1073741824,
        8589934592,
        21474836480,
        42949672960,
        85899345920,
        171798691840,
        343597383680,
        687194767360,
        274877906944,
        2199023255552,
        5497558138880,
        10995116277760,
        21990232555520,
        43980465111040,
        87960930222080,
        175921860444160,
        70368744177664,
        562949953421312,
        1407374883553280,
        2814749767106560,
        5629499534213120,
        11258999068426240,
        22517998136852480,
        45035996273704960,
        18014398509481984,
    ],
    [
        512,
        1280,
        2560,
        5120,
        10240,
        20480,
        40960,
        16384,
        131072,
        327680,
        655360,
        1310720,
        2621440,
        5242880,
        10485760,
        4194304,
        33554432,
        83886080,
        167772160,
        335544320,
        671088640,
        1342177280,
        2684354560,
        1073741824,
        8589934592,
        21474836480,
        42949672960,
        85899345920,
        171798691840,
        343597383680,
        687194767360,
        274877906944,
        2199023255552,
        5497558138880,
        10995116277760,
        21990232555520,
        43980465111040,
        87960930222080,
        175921860444160,
        70368744177664,
        562949953421312,
        1407374883553280,
        2814749767106560,
        5629499534213120,
        11258999068426240,
        22517998136852480,
        45035996273704960,
        18014398509481984,
        144115188075855872,
        360287970189639680,
        720575940379279360,
        1441151880758558720,
        2882303761517117440,
        5764607523034234880,
        11529215046068469760,
        4611686018427387904,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ],
];
