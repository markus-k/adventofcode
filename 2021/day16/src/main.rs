use bitvec::prelude::*;

fn main() {
    let input = include_str!("../input.txt");
    let bv = parse_packet_to_bitvec(input);

    let (packet, _) = Packet::from_bits(&bv);
    println!("Sum of all packet versions: {}", packet.version_sum());

    println!("Packet value: {}", packet.value());
}

#[derive(Debug)]
enum Operator {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
}

impl Operator {
    fn from_typeid(type_id: u8) -> Self {
        match type_id {
            0 => Self::Sum,
            1 => Self::Product,
            2 => Self::Minimum,
            3 => Self::Maximum,
            5 => Self::GreaterThan,
            6 => Self::LessThan,
            7 => Self::EqualTo,
            _ => panic!("{} is not an operator type_id", type_id),
        }
    }
}

#[derive(Debug)]
struct OperatorPacket {
    subpackets: Vec<Packet>,
    operator: Operator,
}

impl OperatorPacket {
    fn new(subpackets: Vec<Packet>, type_id: u8) -> Self {
        let operator = Operator::from_typeid(type_id);

        Self {
            subpackets,
            operator,
        }
    }

    fn version_sum(&self) -> u32 {
        self.subpackets
            .iter()
            .map(|packet| packet.version_sum())
            .sum()
    }

    fn values_iter(&self) -> impl Iterator<Item = u64> + '_ {
        self.subpackets.iter().map(|packet| packet.value())
    }

    fn eval(&self) -> u64 {
        match self.operator {
            Operator::Sum => self.values_iter().sum(),
            Operator::Product => self.values_iter().product(),
            Operator::Minimum => self.values_iter().min().unwrap(),
            Operator::Maximum => self.values_iter().max().unwrap(),
            Operator::GreaterThan => {
                if self.subpackets[0].value() > self.subpackets[1].value() {
                    1
                } else {
                    0
                }
            }
            Operator::LessThan => {
                if self.subpackets[0].value() < self.subpackets[1].value() {
                    1
                } else {
                    0
                }
            }
            Operator::EqualTo => {
                if self.subpackets[0].value() == self.subpackets[1].value() {
                    1
                } else {
                    0
                }
            }
        }
    }
}

#[derive(Debug)]
enum PacketType {
    Literal(BitVec<usize, Msb0>),
    Operator(OperatorPacket),
}

impl PacketType {
    fn from_bits<T: BitStore>(bits: &BitSlice<T, Msb0>) -> (Self, &BitSlice<T, Msb0>) {
        let type_id = bits[0..3].load_be();

        match type_id {
            4 => {
                let (literal, rem) = Self::parse_literal(&bits[3..]);
                (Self::Literal(literal), rem)
            }
            _ => {
                let (packets, rem) = Self::parse_operator(type_id, &bits[3..]);
                (Self::Operator(packets), rem)
            }
        }
    }

    fn parse_literal<T: BitStore>(
        mut bits: &BitSlice<T, Msb0>,
    ) -> (BitVec<usize, Msb0>, &BitSlice<T, Msb0>) {
        let mut bv = BitVec::<usize, Msb0>::new();

        loop {
            bv.extend_from_bitslice(&bits[1..5]);
            let last = bits[0];
            bits = &bits[5..];

            if !last {
                break;
            }
        }

        (bv, bits)
    }

    fn parse_operator<T: BitStore>(
        type_id: u8,
        bits: &BitSlice<T, Msb0>,
    ) -> (OperatorPacket, &BitSlice<T, Msb0>) {
        let length_type_id = bits[0];

        let (subpackets, rem_bits) = match length_type_id {
            false => {
                let total_length = bits[1..16].load_be::<usize>();
                let mut subpackets = Vec::new();
                let mut rem_bits = &bits[16..];
                let num_rem_bits = rem_bits.len();

                while !(num_rem_bits - rem_bits.len() >= total_length) {
                    let (packet, new_rem_bits) = Packet::from_bits(rem_bits);
                    rem_bits = new_rem_bits;
                    subpackets.push(packet);
                }

                (subpackets, rem_bits)
            }
            true => {
                let num_subpackets = bits[1..12].load_be::<usize>();
                let mut subpackets = Vec::with_capacity(num_subpackets);
                let mut rem_bits = &bits[12..];

                for _ in 0..num_subpackets {
                    let (packet, new_rem_bits) = Packet::from_bits(rem_bits);
                    rem_bits = new_rem_bits;
                    subpackets.push(packet);
                }

                (subpackets, rem_bits)
            }
        };

        (OperatorPacket::new(subpackets, type_id), rem_bits)
    }
}

#[derive(Debug)]
struct Packet {
    version: u8,
    inner: PacketType,
}

impl Packet {
    fn from_bits<T: BitStore>(bv: &BitSlice<T, Msb0>) -> (Self, &BitSlice<T, Msb0>) {
        let version: u8 = bv[0..3].load_be();
        let (inner, rem) = PacketType::from_bits(&bv[3..]);

        (Self { version, inner }, rem)
    }

    fn version_sum(&self) -> u32 {
        let subversions = match &self.inner {
            PacketType::Literal(_) => 0,
            PacketType::Operator(packet) => packet.version_sum(),
        };

        self.version as u32 + subversions
    }

    fn value(&self) -> u64 {
        match &self.inner {
            PacketType::Literal(literal) => literal.load_be::<u64>(),
            PacketType::Operator(operator) => operator.eval(),
        }
    }
}

fn parse_packet_to_bitvec(input: &str) -> BitVec<usize, Msb0> {
    let mut bits = BitVec::new();
    for nibble in input.trim().chars().map(|c| c.to_digit(16).unwrap()) {
        bits.extend_from_bitslice(&nibble.view_bits::<Msb0>()[28..]);
    }

    bits
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_literal_packet_input() {
        let bv = parse_packet_to_bitvec("D2FE28");
        dbg!(&bv);
        let (packet, _) = Packet::from_bits(&bv);
        dbg!(&packet);

        assert_eq!(packet.version, 6);

        let PacketType::Literal(literal) = packet.inner else {panic!("wrong packet type")};
        assert_eq!(literal.load_be::<u32>(), 2021);
    }

    #[test]
    fn test_operator_packet_lentype0_input() {
        let bv = parse_packet_to_bitvec("38006F45291200");
        dbg!(&bv);
        let (packet, _) = Packet::from_bits(&bv);
        dbg!(&packet);

        assert_eq!(packet.version, 1);

        let PacketType::Operator(operator) = packet.inner else { panic!("wrong packet type") };
        assert_eq!(operator.subpackets.len(), 2);

        let PacketType::Literal(ref literal) = &operator.subpackets[0].inner else { panic!() };
        assert_eq!(literal.load_be::<usize>(), 10);

        let PacketType::Literal(ref literal) = &operator.subpackets[1].inner else { panic!() };
        assert_eq!(literal.load_be::<usize>(), 20);
    }

    #[test]
    fn test_operator_packet_lentype1_input() {
        let bv = parse_packet_to_bitvec("EE00D40C823060");
        dbg!(&bv);
        let (packet, _) = Packet::from_bits(&bv);
        dbg!(&packet);

        assert_eq!(packet.version, 7);

        let PacketType::Operator(operator) = packet.inner else { panic!("wrong packet type") };
        assert_eq!(operator.subpackets.len(), 3);

        let PacketType::Literal(ref literal) = &operator.subpackets[0].inner else { panic!() };
        assert_eq!(literal.load_be::<usize>(), 1);

        let PacketType::Literal(ref literal) = &operator.subpackets[1].inner else { panic!() };
        assert_eq!(literal.load_be::<usize>(), 2);

        let PacketType::Literal(ref literal) = &operator.subpackets[2].inner else { panic!() };
        assert_eq!(literal.load_be::<usize>(), 3);
    }

    #[test]
    fn test_version_sums_1() {
        let bv = parse_packet_to_bitvec("8A004A801A8002F478");
        let (packet, _) = Packet::from_bits(&bv);
        assert_eq!(packet.version_sum(), 16);
    }

    #[test]
    fn test_version_sums_2() {
        let bv = parse_packet_to_bitvec("620080001611562C8802118E34");
        let (packet, _) = Packet::from_bits(&bv);
        assert_eq!(packet.version_sum(), 12);
    }

    #[test]
    fn test_version_sums_3() {
        let bv = parse_packet_to_bitvec("C0015000016115A2E0802F182340");
        let (packet, _) = Packet::from_bits(&bv);
        assert_eq!(packet.version_sum(), 23);
    }

    #[test]
    fn test_version_sums_4() {
        let bv = parse_packet_to_bitvec("A0016C880162017C3686B18A3D4780");
        let (packet, _) = Packet::from_bits(&bv);
        assert_eq!(packet.version_sum(), 31);
    }

    #[test]
    fn test_operators() {
        let testcases = [
            ("C200B40A82", 3),
            ("04005AC33890", 54),
            ("880086C3E88112", 7),
            ("CE00C43D881120", 9),
            ("D8005AC2A8F0", 1),
            ("F600BC2D8F", 0),
            ("9C005AC2F8F0", 0),
            ("9C0141080250320F1802104A08", 1),
        ];

        for (input, result) in testcases {
            let bv = parse_packet_to_bitvec(input);
            let (packet, _) = Packet::from_bits(&bv);

            let PacketType::Operator(operator) = packet.inner else { panic!("wrong packet type") };
            assert_eq!(operator.eval(), result);
        }
    }
}
