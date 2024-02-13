pub mod math {
    use crate::board::bitboard::constants::{DEBRUIJ_T, DEBRUIJ_M};

    /// sets the bit on a bitboard for example:
    /// bb=0000, sq=1 => 0100(sq starts by 0)
    #[macro_export]
    macro_rules! set_bit {
        ($bb:expr, $sq:expr) => {
            {
                let bb: &mut u64 = $bb;
                let sq: u64 = $sq as u64;

                if(sq < 64) {
                    *bb |= (1u64 << sq);
                }

                bb
            }
        };
    }

    /// returns true if a bit is one on the given square. for example:
    /// bb=0100, sq=1 => true because the first bit equals one(sq starts by 0)
    #[macro_export]
    macro_rules! get_bit {
        ($bb:expr, $sq:expr) => {
            {
                let mut bb: u64 = $bb;
                let sq: u64 = $sq as u64;

                if(sq < 64) {
                    bb &= (1u64 << sq);
                }else {
                    bb = 0;
                }

                bb != 0
            }
        };
    }

    /// clears or set's a bit on any given square. for example:\
    /// <br>bb=0100, sq=1 => 0000 because the first bit equals one(sq starts by 0)\
    /// <br>NOTE: only use this function whenever you are sure there is a bit set to one on said square.\
    /// <br>if you don't do this it might do undefined behaviour.
    #[macro_export]
    macro_rules! clear_bit {
       ($bb:expr, $sq:expr) => {
           {
               let bb: &mut u64 = &mut $bb;
               let sq: u64 = $sq as u64;

               if(sq < 64) {
                   *bb ^= (1u64 << sq);
               }

               bb
           }
       };
    }

    ///sets the bit on a square if it doesn't exists on the mask
    pub(crate) fn set_bit_not_exists(mut bb: u64, mask: u64, sq: i32) -> u64 {
        if sq < 64 && !get_bit!(mask, sq) {
            set_bit!(&mut bb, sq);

            return bb;
        }

        return bb;
    }

    // dead_code is allowed here, only used for debugging!
    #[allow(dead_code)]
    /// Testing tool for printing out bitboards to console
    pub fn print_bitboard(bb: u64) {
        for row in 0..8 {
            for col in 0..8 {
                let sq: i32 = row * 8 + col;

                // if there is a bit found on the square we will display it with an X
                if get_bit!(bb, sq) {
                    print!("X ");
                } else {
                    print!("- ");
                }
            }
            // make sure there is spacing between rows
            println!();
        }
        println!();
    }

    /// function is used to set every single bit until the index
    /// so if index is 4(00100) the result will be (11100)
    pub fn set_occ(mut mask: u64, bits: u64, index: u64) -> u64 {
        let mut result: u64 = 0;
        for bit in 0..bits {
            // least significant first square
            let ls1sq = get_ls1b(mask);

            clear_bit!(&mut mask, ls1sq as i32);

            if (index & (1u64 << bit)) != 0 {
                result |= 1u64 << ls1sq;
            }
        }
        return result;
    }

    /// gets the lowest first significant bit
    /// this means that 6(011) will output 2 since the second bit is a 1
    /// https://www.chessprogramming.org/BitScan De Bruijn Multiplication: With separated LS1B
    #[inline(always)]
    pub fn get_ls1b(bits: u64) -> u64 {
        DEBRUIJ_T[(((bits ^ bits.wrapping_sub(1)).wrapping_mul(DEBRUIJ_M)).wrapping_shr(58)) as usize]
    }
}

/// contains all u64 constants used in the program
pub mod constants {
    //  bitboard info for moving pieces
    pub const A_FILE: u64 = 72340172838076673u64;
    pub const AB_FILE: u64 = 217020518514230019u64;
    pub const GH_FILE: u64 = 13889313184910721216u64;
    pub const H_FILE: u64 = 9259542123273814144u64;

    pub const BISHOP_RELEVANT_BITS: [u64; 64] = [
        6, 5, 5, 5, 5, 5, 5, 6,
        5, 5, 5, 5, 5, 5, 5, 5,
        5, 5, 7, 7, 7, 7, 5, 5,
        5, 5, 7, 9, 9, 7, 5, 5,
        5, 5, 7, 9, 9, 7, 5, 5,
        5, 5, 7, 7, 7, 7, 5, 5,
        5, 5, 5, 5, 5, 5, 5, 5,
        6, 5, 5, 5, 5, 5, 5, 6
    ];

    // rook relevant occupancy bit count for every square on board
    pub const ROOK_RELEVANT_BITS: [u64; 64] = [
        12, 11, 11, 11, 11, 11, 11, 12,
        11, 10, 10, 10, 10, 10, 10, 11,
        11, 10, 10, 10, 10, 10, 10, 11,
        11, 10, 10, 10, 10, 10, 10, 11,
        11, 10, 10, 10, 10, 10, 10, 11,
        11, 10, 10, 10, 10, 10, 10, 11,
        11, 10, 10, 10, 10, 10, 10, 11,
        12, 11, 11, 11, 11, 11, 11, 12
    ];

    pub(crate) static DEBRUIJ_T: &'static [u64] = &[
        0, 47,  1, 56, 48, 27,  2, 60,
        57, 49, 41, 37, 28, 16,  3, 61,
        54, 58, 35, 52, 50, 42, 21, 44,
        38, 32, 29, 23, 17, 11,  4, 62,
        46, 55, 26, 59, 40, 36, 15, 53,
        34, 51, 20, 43, 31, 22, 10, 45,
        25, 39, 14, 33, 19, 30,  9, 24,
        13, 18,  8, 12,  7,  6,  5, 63
    ];

    pub(crate) const DEBRUIJ_M: u64 = 0x03f7_9d71_b4cb_0a89;

    // we need to store the magic we have generated so that we can ensure the move generation will go as planned
    pub(crate) const BISHOP_MAGIC: [u64; 64] = [
        1206973522010850048u64,
        74335820789710848u64,
        1166436742339887121u64,
        1143775561253184u64,
        5765172741032387096u64,
        294807428697472u64,
        86694851308945408u64,
        5102850680031752u64,
        118764505138196u64,
        2306144344723718272u64,
        576478637629513746u64,
        3544879191820290u64,
        4553203255816u64,
        9007818003194176u64,
        1251253096155138u64,
        292066431040u64,
        2382404838606504976u64,
        578712569314279936u64,
        576884133016387841u64,
        4631952253394157648u64,
        5635686445096960u64,
        11529355801005361216u64,
        11538785197435656384u64,
        1225542049303241216u64,
        1164831449997640200u64,
        27619732358430864u64,
        9241959830812885056u64,
        5767000198345785856u64,
        281543780089888u64,
        29557071582220290u64,
        9223550710212723851u64,
        5139426619556352u64,
        4512501568196608u64,
        2306975540579602432u64,
        595337460156205056u64,
        9289088698417280u64,
        18050682661699904u64,
        2938616350119069696u64,
        4509441882456578u64,
        1748228991222088u64,
        146371454749902848u64,
        4612833915344783360u64,
        2450522263951313411u64,
        7205900279072430080u64,
        659786145810875393u64,
        706505217147136u64,
        218724782970049040u64,
        468669103389803008u64,
        9226752013713539076u64,
        145805490072339456u64,
        360979369988u64,
        675549022064741377u64,
        9403517310985175041u64,
        317896333103104u64,
        13669335268614148u64,
        579293386323558784u64,
        6900544656646145u64,
        4611704711367623936u64,
        9368051025455416320u64,
        206913701889u64,
        3458785748675994624u64,
        74309413447540224u64,
        10664634008496898176u64,
        37163596639732224u64,];

    pub(crate) const ROOK_MAGIC: [u64; 64] = [
        4647723886419382913u64,
        234187318132473856u64,
        3819087668652294150u64,
        36081573653118980u64,
        1188954699974836357u64,
        108096295264321536u64,
        36029896539111426u64,
        144115812058276896u64,
        282024736718849u64,
        27162610132582400u64,
        4721461398932080640u64,
        703756161777674u64,
        9288777311726608u64,
        562965590315008u64,
        5629645631260672u64,
        874261294849593472u64,
        9223425363172917328u64,
        76562018301116546u64,
        4611705810177785859u64,
        1153520738579251208u64,
        151732807016704u64,
        2305845208304586784u64,
        9020394216620672u64,
        2305843558973767688u64,
        5188217141623545856u64,
        9223688698355254274u64,
        2386907976453586948u64,
        2306458737873256452u64,
        4521453809041408u64,
        9150137918291970u64,
        116153096089601u64,
        882712128329388640u64,
        9290050802034688u64,
        576636742887538960u64,
        292742789054070788u64,
        2919608060760564224u64,
        35562631201808u64,
        1134765826638976u64,
        2199040033796u64,
        144259241312518208u64,
        7062947416177246212u64,
        4756293867706286080u64,
        45036271959080968u64,
        211141433360392u64,
        73755252943257600u64,
        5193798261604909064u64,
        1224981314933096458u64,
        2467973712499048464u64,
        289393660645245952u64,
        4611826893358891264u64,
        4611703611714437376u64,
        17592320303360u64,
        70442296017920u64,
        4398214348928u64,
        18016610686612480u64,
        4613938419540754944u64,
        140754940854305u64,
        35186523769395u64,
        576742302446280716u64,
        4683752684578144290u64,
        1734325804064047106u64,
        865254233027906049u64,
        9302190581817018372u64,
        2308376292598251633u64,
    ];
}

