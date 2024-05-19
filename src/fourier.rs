// God bless this file
use crate::{
    engine::{BitBoard, Board},
    piececonstants,
};
use rustfft::{num_complex::Complex, FftPlanner};

// Turns a bitboard u64 into a 144 length transform vector
pub fn bitboard_fft(
    result: &mut Vec<Complex<f32>>,
    scratch: &mut Vec<Complex<f32>>,
    bitboard: &u64,
) {
    let size = 12;
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(size * size);

    //println!("{}", fft.get_inplace_scratch_len());

    for i in 0..size * size {
        result[i] = Complex {
            re: 0.0f32,
            im: 0.0f32,
        };
    }

    // First set up an array from board
    for i in 0..8 {
        for j in 0..8 {
            result[i * size + j].re = bitboard.get_bit(i * 8 + j) as f32;
        }
    }
    // Then run fft

    fft.process_with_scratch(result, scratch);

    //println!("{:?}", buffer);
}

// Turns a bitboard u64 into a 144 length transform vector using hashed values if available
pub fn lazy_bitboard_fft(
    result: &mut Vec<Complex<f32>>,
    scratch: &mut Vec<Complex<f32>>,
    bitboard: &u64,
) {
    let size = 12;

    //println!("{}", fft.get_inplace_scratch_len());

    let ones = bitboard.count_ones();
    if ones == 0 {
        // If there are no pieces on board, then ft is empty
        for i in 0..size * size {
            result[i] = Complex {
                re: 0.0f32,
                im: 0.0f32,
            };
        }
        return;
    } else if ones <= 2 {
        // 2 piece boards are precomputed and can be looked up
        let upper = bitboard.trailing_zeros() as usize;
        let lower = 63 - bitboard.leading_zeros() as usize;

        *result = piececonstants::BOARD_FFTS[upper * 64 + lower].clone();
        return;
    } else {
        // For higher piece boards, no way around computation
        bitboard_fft(result, scratch, bitboard)
    }
}

// Return the fft onto a 144 length vector of a square filter
pub fn filter_fft(
    result: &mut Vec<Complex<f32>>,
    scratch: &mut Vec<Complex<f32>>,
    filter: &Vec<Vec<f32>>,
) {
    let filtersize = filter.len();
    let center = filtersize / 2;
    let size = 12;
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(size * size);
    for i in 0..size * size {
        result[i] = Complex {
            re: 0.0f32,
            im: 0.0f32,
        };
    }
    for i in 0..filtersize {
        for j in 0..filtersize {
            let (n, m) = match (i < center, j < center) {
                (false, false) => ((i - center), j - center),

                (true, false) => ((size - center + i), j - center),
                (false, true) => {
                    if i == center {
                        ((size - 1), (size - center + j))
                    } else {
                        ((i - center - 1), size - center + j)
                    }
                }
                (true, true) => ((size - center + i - 1), (size - center + j)),
            };
            result[n * size + m].re = filter[filtersize - i - 1][filtersize - j - 1];
        }
    }
    //print_ifft(result);

    fft.process_with_scratch(result, scratch);
}
// After all fft's are combined, inverse transformation into final square values
pub fn ifft(result: &mut Vec<Complex<f32>>, scratch: &mut Vec<Complex<f32>>) {
    let size = 12;
    let mut planner = FftPlanner::new();
    let ifft = planner.plan_fft_inverse(size * size);
    ifft.process_with_scratch(result, scratch);
}

// Compute elementwise multiplication of two complex matricies, allows for convolution
pub fn signal_conv(result: &mut Vec<Complex<f32>>, filter: &Vec<Complex<f32>>) {
    for i in 0..result.len() {
        result[i] *= filter[i];
    }
}

// take a stack of signal arrays and add them element wise
// Used to add all channel signals to then convert into a final square value.
pub fn collapse_conv(result: &mut Vec<Complex<f32>>, layers: &Vec<Vec<Complex<f32>>>) {
    for i in 0..result.len() {
        result[i] = layers.iter().map(|x| x[i]).sum();
    }
}
pub fn print_ifft(result: &Vec<Complex<f32>>) {
    let size = 12.;
    for i in 0..8 {
        for j in 0..8 {
            print!("{:<6.2} ", result[i * 12 + j].re / (size * size));
        }
        println!();
    }
}
/* let ifft = planner.plan_fft_inverse(size);

ifft.process(&mut buffer);
//println!("{:?}", buffer);
for i in 0..8 {
    for j in 0..8 {
        print!("{} ", buffer[i * size + j].re);
    }
    println!()
}*/

// Initialize 1 and 2 man ft boards
pub fn bitboard_fft_init() -> Vec<Vec<Complex<f32>>> {
    let mut result = vec![
        vec![
            Complex {
                re: 0.0f32,
                im: 0.0f32
            };
            144
        ];
        64 * 64
    ];
    let mut scratch = result[0].clone();
    for i in 0..64 {
        for j in 0..64 {
            let bitobard = (1 << i) | (1 << j);
            bitboard_fft(&mut result[i * 64 + j], &mut scratch, &bitobard);
        }
    }
    return result;
}

// takes a vector of square mask sets (12 x n x n) and returns a vector of all ffts 
pub fn mask_fft_init(masks: Vec<Vec<Vec<Vec<f32>>>>) -> Vec<Vec<Vec<Complex<f32>>>> {
    let n = masks.len();
    let mut result = vec![
        vec![
            vec![
                Complex {
                    re: 0.0f32,
                    im: 0.0f32
                };
                144
            ];
            12
        ];
        n
    ];
    let mut scratch = result[0][0].clone();
    for i in 0..n {
        for j in 0..12 {
            filter_fft(&mut result[i][j], &mut scratch, &masks[i][j]);
        }
    }

    result
}
