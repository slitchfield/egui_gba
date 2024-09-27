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

pub fn get_halfword(byte_array: &[u8], address: usize) -> u16 {
    // Assumes address sanitization and translation has already occurred.
    // I.e., this method just uses address to generate successive indices to `byte_array`
    ((byte_array[address + 1] as u16) << 8) | (byte_array[address] as u16)
}

pub fn get_word(byte_array: &[u8], address: usize) -> u32 {
    ((byte_array[address + 3] as u32) << 24)
        | ((byte_array[address + 2] as u32) << 16)
        | ((byte_array[address + 1] as u32) << 8)
        | (byte_array[address] as u32)
}
