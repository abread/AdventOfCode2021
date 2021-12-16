use bitvec::prelude::*;
use std::io::{self, BufRead};

#[derive(Debug)]
enum Packet {
    Literal {
        version: u8,
        _parts: Vec<u8>,
    },
    Operator {
        version: u8,
        typ: OperatorType,
        subpackets: Vec<Packet>,
    },
}

#[derive(Debug)]
enum OperatorType {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
}

/*
Header Format:
|0  ...  2|3  ...  5|6 ..
| Version | Type ID |

Payload format (TypeId = 4):
|6 .. 10|11 .. 15|16 .. 20|
|   A   |   B    |   C    |

Payload format (for TypeID != 4, length type 0):
|          6         |7      ..      21|22 .. (22+subpacket_length-1)|
| Length Type ID = 0 |Sub-packet length|         Sub-packets         |

Payload format (for TypeID != 4, length type 1):
|          6         |7      ..      17|18 ..                        |
| Length Type ID = 1 |Sub-packet  count|     <count> Sub-packets     |
*/

fn parse_packet(line: impl AsRef<str>) -> Packet {
    let line = line.as_ref().trim();
    let bytes = hex::decode(line).unwrap();
    let bits = bytes.view_bits::<Msb0>();
    parse_packet_from_bits(bits).0
}

fn parse_packet_from_bits<S: BitStore>(bits: &BitSlice<Msb0, S>) -> (Packet, usize) {
    let version = bits[0..=2].load_be();
    let type_id: u8 = bits[3..=5].load_be();

    let (packet, size) = match type_id {
        4 => parse_literal_packet(version, bits),
        _ => parse_operator_packet(type_id, version, bits),
    };

    (packet, size)
}

fn parse_literal_packet<S: BitStore>(version: u8, bits: &BitSlice<Msb0, S>) -> (Packet, usize) {
    let mut bit_it = &bits[6..];
    let mut parts = Vec::new();

    loop {
        // bit 0 is the "do I have more" flag
        // bits 1-4 are part of the number
        parts.push(bit_it[1..5].load_be());

        if !bit_it[0] {
            // this was the last section
            break;
        }
        bit_it = &bit_it[5..];
    }

    let size = 6 + parts.len() * 5;

    // aligned to nibs, account for padding
    //size += size % 4;

    (
        Packet::Literal {
            version,
            _parts: parts,
        },
        size,
    )
}

fn parse_operator_packet<S: BitStore>(
    type_id: u8,
    version: u8,
    bits: &BitSlice<Msb0, S>,
) -> (Packet, usize) {
    debug_assert_ne!(type_id, 4);

    let length_type_id = bits[6];
    let mut size = 7;

    let (subpackets, sub_size) = if length_type_id {
        let count: u16 = bits[7..=17].load_be();
        size += 11;
        parse_subpackets_until_count(count as usize, &bits[18..])
    } else {
        let len: u16 = bits[7..=21].load_be();
        size += 15;
        parse_subpackets_until_len(len as usize, &bits[22..])
    };
    size += sub_size;

    let typ = match type_id {
        0 => OperatorType::Sum,
        1 => OperatorType::Product,
        2 => OperatorType::Minimum,
        3 => OperatorType::Maximum,
        5 => OperatorType::GreaterThan,
        6 => OperatorType::LessThan,
        7 => OperatorType::EqualTo,
        _ => unreachable!("unknown operator type"),
    };

    (
        Packet::Operator {
            version,
            typ,
            subpackets,
        },
        size,
    )
}

fn parse_subpackets_until_len<S: BitStore>(
    len: usize,
    mut bits: &BitSlice<Msb0, S>,
) -> (Vec<Packet>, usize) {
    let mut packets = Vec::new();
    let mut n_read = 0_usize;

    while n_read < len {
        let (sub, sub_len) = parse_packet_from_bits(bits);

        packets.push(sub);
        bits = &bits[sub_len..];
        n_read += sub_len;
    }
    debug_assert_eq!(n_read, len);

    packets.shrink_to_fit();
    (packets, n_read)
}

fn parse_subpackets_until_count<S: BitStore>(
    count: usize,
    mut bits: &BitSlice<Msb0, S>,
) -> (Vec<Packet>, usize) {
    let mut packets = Vec::with_capacity(count);
    let mut size = 0;

    while packets.len() != count {
        let (sub, sub_len) = parse_packet_from_bits(bits);

        packets.push(sub);
        bits = &bits[sub_len..];
        size += sub_len;
    }

    (packets, size)
}

fn main() {
    let packets: Vec<Packet> = io::stdin()
        .lock()
        .lines()
        .map(Result::unwrap)
        .map(parse_packet)
        //.inspect(|p| { dbg!(p); })
        .collect();

    fn packet_version_sum(packet: &Packet) -> u64 {
        match packet {
            Packet::Literal { version, .. } => *version as u64,
            Packet::Operator {
                version,
                subpackets,
                ..
            } => *version as u64 + subpackets.iter().map(packet_version_sum).sum::<u64>(),
        }
    }
    dbg!(packet_version_sum(&packets[0]));

    fn compute_value(packet: &Packet) -> u64 {
        match packet {
            Packet::Literal { _parts, .. } => {
                _parts.iter().fold(0_u64, |acc, &x| acc * 16 + x as u64)
            }
            Packet::Operator {
                typ, subpackets, ..
            } => {
                use OperatorType::*;
                match typ {
                    Sum => subpackets.iter().map(compute_value).sum::<u64>(),
                    Product => subpackets.iter().map(compute_value).product::<u64>(),
                    Minimum => subpackets.iter().map(compute_value).min().unwrap(),
                    Maximum => subpackets.iter().map(compute_value).max().unwrap(),
                    GreaterThan if subpackets.len() == 2 => {
                        if compute_value(&subpackets[0]) > compute_value(&subpackets[1]) {
                            1
                        } else {
                            0
                        }
                    }
                    LessThan if subpackets.len() == 2 => {
                        if compute_value(&subpackets[0]) < compute_value(&subpackets[1]) {
                            1
                        } else {
                            0
                        }
                    }
                    EqualTo if subpackets.len() == 2 => {
                        if compute_value(&subpackets[0]) == compute_value(&subpackets[1]) {
                            1
                        } else {
                            0
                        }
                    }
                    GreaterThan | LessThan | EqualTo => {
                        unreachable!("invalid operand count for comparator")
                    }
                }
            }
        }
    }
    dbg!(compute_value(&packets[0]));
}
