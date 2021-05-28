pub const ROOK_MASKS:[u64;64] = [
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


pub const BISHOP_MASKS:[u64;64] = [
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

pub const ROOK_POINTERS: [usize; 64] = [
    0,
    4160,
    6240,
    8320,
    10400,
    12480,
    14560,
    16640,
    20800,
    22880,
    23936,
    24992,
    26048,
    27104,
    28160,
    29216,
    31296,
    33376,
    34432,
    35584,
    36736,
    37888,
    39040,
    40096,
    42176,
    44256,
    45312,
    46464,
    48000,
    49536,
    50688,
    51744,
    53824,
    55904,
    56960,
    58112,
    59648,
    61184,
    62336,
    63392,
    65472,
    67552,
    68608,
    69760,
    70912,
    72064,
    73216,
    74272,
    76352,
    78432,
    79488,
    80544,
    81600,
    82656,
    83712,
    84768,
    86848,
    91008,
    93088,
    95168,
    97248,
    99328,
    101408,
    103488,
];

pub const BISHOP_POINTERS: [usize; 64] = [
    4096,
    6208,
    8288,
    10368,
    12448,
    14528,
    16608,
    20736,
    22848,
    23904,
    24960,
    26016,
    27072,
    28128,
    29184,
    31264,
    33344,
    34400,
    35456,
    36608,
    37760,
    38912,
    40064,
    42144,
    44224,
    45280,
    46336,
    47488,
    49024,
    50560,
    51712,
    53792,
    55872,
    56928,
    57984,
    59136,
    60672,
    62208,
    63360,
    65440,
    67520,
    68576,
    69632,
    70784,
    71936,
    73088,
    74240,
    76320,
    78400,
    79456,
    80512,
    81568,
    82624,
    83680,
    84736,
    86816,
    90944,
    93056,
    95136,
    97216,
    99296,
    101376,
    103456,
    107584,
];
