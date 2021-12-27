use std::fs::read_to_string;

fn read_bits(bytes: &[u8], cursor: &mut usize, size: usize) -> Vec<u8> {
    let start = *cursor / 8;
    let bit_end = *cursor + size;
    let end = bit_end / 8;
    let start_alignment = *cursor % 8;
    let mut end_alignment = bit_end % 8;

    let mut bits = bytes[start..=end].to_owned();
    if bits.len() > 1 {
        *bits.first_mut().expect("bits were empty") &= !0 >> start_alignment;
    }
    if bits.len() > size / 8 + 1 {
        bits = bits
            .windows(2)
            .map(|bits| bits[0] << start_alignment | bits[1] >> (8 - start_alignment))
            .collect();
        end_alignment += 8 - start_alignment;
        end_alignment %= 8;
    }
    let last_bit = bits.last_mut().expect("bits were empty");
    *last_bit >>= 8 - end_alignment;
    *last_bit &= !0 >> (8 - (size % 8));

    *cursor += size;
    bits
}

fn read_packet(bytes: &[u8], cursor: &mut usize) -> u32 {
    let mut version_sum = read_bits(bytes, cursor, 3)[0] as u32;
    let type_id = read_bits(bytes, cursor, 3)[0];
    if type_id == 4 {
        while read_bits(bytes, cursor, 5)[0] >> 4 != 0 {}
    } else {
        let length_type_id = read_bits(bytes, cursor, 1)[0];
        if length_type_id == 0 {
            let length = read_bits(bytes, cursor, 15);
            let length = u16::from_be_bytes([length[0] >> 1, (length[0] << 7) | length[1]]);
            let end = *cursor + length as usize;
            while *cursor < end {
                version_sum += read_packet(bytes, cursor);
            }
        } else {
            let length = read_bits(bytes, cursor, 11);
            let length = u16::from_be_bytes([length[0] >> 4, (length[0] << 3) | length[1]]);
            for _ in 0..length {
                version_sum += read_packet(bytes, cursor);
            }
        }
    }
    version_sum
}

fn main() {
    let input: Vec<_> = read_to_string("../input")
        .expect("failed to open input file")
        .lines()
        .next()
        .expect("input file was empty")
        .chars()
        .map(|hex| hex.to_digit(16).expect("failed to parse hex digit"))
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|chunk| (chunk[0] << 4 | chunk[1]) as u8)
        .collect();

    let version_sum = read_packet(&input, &mut 0);

    println!("The sum of all the version packets is {}!", version_sum);
}
