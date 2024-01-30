/*
    explanation of how the XORSHIFT random number works: https://en.wikipedia.org/wiki/Xorshift
 */

///generated in python with random.randint(0, u32::max-1)
static mut RANDOM_STATE: u32 = 892777658;

///random u32 number based on XORSHIFT method
pub unsafe fn get_random_u32_number() -> u32 {
    let mut number: u32 = RANDOM_STATE;

    number ^= number << 13;
    number ^= number >> 17;
    number ^= number << 5;

    RANDOM_STATE = number;

    return number;
}

/// generates a fast pseudo random u64 number based on 4 pseudo random u32 numbers
/// we use the first 16 bits of the u32 number to unsure it randomness
pub unsafe fn get_random_u64_number() -> u64 {
    let (n1, n2, n3, n4): (u64, u64, u64, u64);

    // 0xFFFF = first 16 bits of a board
    n1 = (get_random_u32_number() & 0xFFFF) as u64;
    n2 = (get_random_u32_number() & 0xFFFF) as u64;
    n3 = (get_random_u32_number() & 0xFFFF) as u64;
    n4 = (get_random_u32_number() & 0xFFFF) as u64;

    return n1 | (n2 << 16) | (n3 << 32) | (n4 << 48);
}

/// generates a fast pseudo random number
pub fn generate_magic_number() -> u64 {
    let digit: u64 = unsafe { get_random_u64_number() & get_random_u64_number() & get_random_u64_number() };
    return digit;
}
