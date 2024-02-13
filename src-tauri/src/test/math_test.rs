use crate::board::bitboard::math::{get_ls1b, set_occ};
use crate::helpers::random::generate_magic_number;

#[test]
fn test_ls1b() {
    assert_eq!(get_ls1b(1u64 << 22u64), 22);
    assert_eq!(get_ls1b(1u64 << 23u64), 23);
    assert_eq!(get_ls1b(1u64 << 24u64), 24);
}

/// we are just going to test if the number != 0
#[test]
fn test_xorshift_random_num() {
    assert_ne!(generate_magic_number(), 0)
}

#[test]
fn test_set_occ() {
    let mut testing_index: u64 = 0u64;
    for i in 0..5 {
        testing_index |= 1u64 << i as u64;
    }
    let relevant_bits: u64 = testing_index.count_ones() as u64;
    let mask = testing_index;
    testing_index ^= 1u64 << 2;
    let occ = set_occ(mask, relevant_bits, testing_index);
    assert_eq!(occ, testing_index);
}