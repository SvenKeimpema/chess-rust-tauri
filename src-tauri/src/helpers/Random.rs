/*
    explanation of how the XORSHIFT random number works: https://en.wikipedia.org/wiki/Xorshift
 */

//generated in python with random.randint(0, u32::max-1)
static mut RANDOM_STATE: u32 = 892777658;

pub unsafe fn get_random_u32_number() -> u32 {
    let mut number: u32 = RANDOM_STATE;

    number ^= number << 13;
    number ^= number >> 17;
    number ^= number << 5;

    RANDOM_STATE = number;

    return number;
}

pub unsafe fn get_random_u64_number() -> u64 {
    let (n1, n2, n3, n4): (u64, u64, u64, u64);

    n1 = (get_random_u32_number() & 0xFFFF) as u64;
    n2 = (get_random_u32_number() & 0xFFFF) as u64;
    n3 = (get_random_u32_number() & 0xFFFF) as u64;
    n4 = (get_random_u32_number() & 0xFFFF) as u64;

    return n1 | (n2 << 16) | (n3 << 32) | (n4 << 48);
}

pub fn generate_magic_number() -> u64 {
    let digit: u64 = unsafe { get_random_u64_number() & get_random_u64_number() & get_random_u64_number() };
    return digit;
}
