use std::{cmp::Ordering, fs::read_to_string};

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

fn read_packet(bytes: &[u8], cursor: &mut usize) -> u64 {
    read_bits(bytes, cursor, 3);
    let type_id = read_bits(bytes, cursor, 3)[0];
    if type_id == 4 {
        let mut literal = 0;
        let mut more_bits = 1;
        while more_bits != 0 {
            let literal_bits = read_bits(bytes, cursor, 5)[0];
            literal <<= 4;
            literal |= literal_bits as u64 & 0b1111;
            more_bits = literal_bits >> 4;
        }
        literal
    } else {
        let length_type_id = read_bits(bytes, cursor, 1)[0];
        let mut sub_packets: Box<dyn Iterator<Item = _>> = if length_type_id == 0 {
            let length = read_bits(bytes, cursor, 15);
            let length = u16::from_be_bytes([length[0] >> 1, (length[0] << 7) | length[1]]);
            let end = *cursor + length as usize;
            Box::new(std::iter::from_fn(move || {
                if *cursor < end {
                    Some(read_packet(bytes, cursor))
                } else {
                    None
                }
            }))
        } else {
            let length = read_bits(bytes, cursor, 11);
            let length = u16::from_be_bytes([length[0] >> 4, (length[0] << 3) | length[1]]);
            Box::new((0..length).map(|_| read_packet(bytes, cursor)))
        };

        fn compare_subpackets(
            sub_packets: &mut dyn Iterator<Item = u64>,
            desired_order: Ordering,
        ) -> u64 {
            let first = sub_packets
                .next()
                .expect("no sub-packets for compare operation");
            let second = sub_packets
                .next()
                .expect("only one sub-packet for compare operation");

            let order = first.cmp(&second);
            if order == desired_order {
                1
            } else {
                0
            }
        }

        match type_id {
            0 => sub_packets.sum(),
            1 => sub_packets.product(),
            2 => sub_packets.min().expect("sub-packets were empty"),
            3 => sub_packets.max().expect("sub-packets were empty"),
            5 => compare_subpackets(&mut sub_packets, Ordering::Greater),
            6 => compare_subpackets(&mut sub_packets, Ordering::Less),
            7 => compare_subpackets(&mut sub_packets, Ordering::Equal),
            _ => panic!("unexpected packet type {}", type_id),
        }
    }
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

    let evaluated = read_packet(&input, &mut 0);

    println!(
        "The evaluated value of the BITS transmission expression is {}!",
        evaluated
    );
}
