use crate::utils::SolverResult;
use std::fs::read_to_string;

#[derive(Debug, PartialEq, Eq)]
enum Packet {
    Literal {
        version: usize,
        type_id: usize,
        value: usize,
    },
    Operator {
        version: usize,
        type_id: usize,
        length_type_id: usize,
        packets: Vec<Packet>,
    },
}

fn parse_bin(bin: &str) -> usize {
    usize::from_str_radix(bin, 2).unwrap()
}

fn parse(bin: &str) -> Packet {
    let mut pointer = 0;
    _parse(bin, &mut pointer)
}

fn _parse(bin: &str, pointer: &mut usize) -> Packet {
    let version = parse_bin(&bin[*pointer..*pointer + 3]);
    *pointer += 3;
    let type_id = parse_bin(&bin[*pointer..*pointer + 3]);
    *pointer += 3;

    match type_id {
        4 => {
            let mut value_bits: Vec<char> = vec![];

            let mut another = true;
            while another {
                another = &bin[*pointer..*pointer + 1] == "1";
                *pointer += 1;

                bin[*pointer..*pointer + 4]
                    .chars()
                    .for_each(|c| value_bits.push(c));
                *pointer += 4;
            }

            let value: String = value_bits.into_iter().collect();

            Packet::Literal {
                version,
                type_id,
                value: parse_bin(&value),
            }
        }
        _ => {
            let length_type_id = parse_bin(&bin[*pointer..*pointer + 1]);
            *pointer += 1;

            match length_type_id {
                0 => {
                    let num_bits = parse_bin(&bin[*pointer..*pointer + 15]);
                    *pointer += 15;

                    let current_pointer = *pointer;

                    let mut packets = vec![];

                    while *pointer != current_pointer + num_bits {
                        packets.push(_parse(bin, pointer));
                    }

                    Packet::Operator {
                        version,
                        type_id,
                        length_type_id,
                        packets,
                    }
                }
                1 => {
                    let num_packets = parse_bin(&bin[*pointer..*pointer + 11]);
                    *pointer += 11;

                    let packets = (0..num_packets).map(|_| _parse(bin, pointer)).collect();

                    Packet::Operator {
                        version,
                        type_id,
                        length_type_id,
                        packets,
                    }
                }
                _ => unreachable!("Length type id was not in binary"),
            }
        }
    }
}

fn sum_versions(packet: &Packet) -> usize {
    match packet {
        Packet::Literal { version, .. } => *version,
        Packet::Operator {
            version, packets, ..
        } => version + packets.iter().map(sum_versions).sum::<usize>(),
    }
}

fn evaluate(packet: &Packet) -> usize {
    match packet {
        Packet::Literal { value, .. } => *value,
        Packet::Operator {
            type_id, packets, ..
        } => {
            let mut values = packets.iter().map(evaluate);
            match type_id {
                0 => values.sum(),
                1 => values.product(),
                2 => values.min().unwrap(),
                3 => values.max().unwrap(),
                5 => {
                    if values.next().unwrap() > values.next().unwrap() {
                        1
                    } else {
                        0
                    }
                }
                6 => {
                    if values.next().unwrap() < values.next().unwrap() {
                        1
                    } else {
                        0
                    }
                }
                7 => {
                    if values.next().unwrap() == values.next().unwrap() {
                        1
                    } else {
                        0
                    }
                }
                _ => unreachable!("Unknown operator type id {}", type_id),
            }
        }
    }
}

fn hex_to_bin(hex: &str) -> String {
    hex.chars()
        .map(|c| match c {
            '0' => "0000",
            '1' => "0001",
            '2' => "0010",
            '3' => "0011",
            '4' => "0100",
            '5' => "0101",
            '6' => "0110",
            '7' => "0111",
            '8' => "1000",
            '9' => "1001",
            'A' => "1010",
            'B' => "1011",
            'C' => "1100",
            'D' => "1101",
            'E' => "1110",
            'F' => "1111",
            _ => "",
        })
        .collect()
}

pub fn solve() -> SolverResult {
    let bin = hex_to_bin(&read_to_string("data/day_16.txt")?);

    let bin_argument = &bin;
    println!("Part 1: {}", sum_versions(&parse(bin_argument)));
    let bin_argument = &bin;
    println!("Part 2: {}", evaluate(&parse(bin_argument)));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_example_1() {
        let bin = hex_to_bin("D2FE28");
        assert_eq!(bin, "110100101111111000101000");
        assert_eq!(
            parse(&bin),
            Packet::Literal {
                version: 6,
                type_id: 4,
                value: 2021
            }
        );
    }
    #[test]
    fn part_1_example_2() {
        let bin = hex_to_bin("38006F45291200");
        assert_eq!(
            bin,
            "00111000000000000110111101000101001010010001001000000000"
        );
        assert_eq!(
            parse(&bin),
            Packet::Operator {
                version: 1,
                type_id: 6,
                length_type_id: 0,
                packets: vec![
                    Packet::Literal {
                        version: 6,
                        type_id: 4,
                        value: 10
                    },
                    Packet::Literal {
                        version: 2,
                        type_id: 4,
                        value: 20
                    }
                ],
            }
        );
    }

    #[test]
    fn part_1_example_3() {
        let bin = hex_to_bin("EE00D40C823060");
        assert_eq!(
            bin,
            "11101110000000001101010000001100100000100011000001100000"
        );
        assert_eq!(
            parse(&bin),
            Packet::Operator {
                version: 7,
                type_id: 3,
                length_type_id: 1,
                packets: vec![
                    Packet::Literal {
                        version: 2,
                        type_id: 4,
                        value: 1
                    },
                    Packet::Literal {
                        version: 4,
                        type_id: 4,
                        value: 2
                    },
                    Packet::Literal {
                        version: 1,
                        type_id: 4,
                        value: 3
                    }
                ],
            }
        );
    }

    #[test]
    fn part_1_examples() {
        assert_eq!(sum_versions(&parse(&hex_to_bin("8A004A801A8002F478"))), 16);
        assert_eq!(
            sum_versions(&parse(&hex_to_bin("620080001611562C8802118E34"))),
            12
        );
        assert_eq!(
            sum_versions(&parse(&hex_to_bin("C0015000016115A2E0802F182340"))),
            23
        );
        assert_eq!(
            sum_versions(&parse(&hex_to_bin("A0016C880162017C3686B18A3D4780"))),
            31
        );
    }

    #[test]
    fn part_2_examples() {
        assert_eq!(evaluate(&parse(&hex_to_bin("C200B40A82"))), 3);
        assert_eq!(evaluate(&parse(&hex_to_bin("04005AC33890"))), 54);
        assert_eq!(evaluate(&parse(&hex_to_bin("880086C3E88112"))), 7);
        assert_eq!(evaluate(&parse(&hex_to_bin("CE00C43D881120"))), 9);
        assert_eq!(evaluate(&parse(&hex_to_bin("D8005AC2A8F0"))), 1);
        assert_eq!(evaluate(&parse(&hex_to_bin("F600BC2D8F"))), 0);
        assert_eq!(evaluate(&parse(&hex_to_bin("9C005AC2F8F0"))), 0);
        assert_eq!(
            evaluate(&parse(&hex_to_bin("9C0141080250320F1802104A08"))),
            1
        );
    }
}
