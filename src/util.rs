
pub fn set_bits(input: u32, offset: u8, num: u8, bits: u32) -> u32 {
    let mut mask: u32 = ((1 << num) - 1) as u32;
    let mut new_val = bits & mask;
    mask = mask << offset;
    new_val = new_val << offset;

    let mut output = input & !mask;
    output |= new_val;
    output
}

pub fn get_bits(input: u32, offset: u8, num: u8) -> u32 {
    let mut mask: u32 = ((1 << num) - 1) as u32;
    mask = mask << offset;

    let mut output: u32 = input & mask;
    output >>= offset;
    output
}