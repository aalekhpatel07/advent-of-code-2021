use nom::{combinator::map, sequence::*, IResult, multi::{many1, count, many0}, character::complete::one_of, branch::alt};
use nom::bytes::complete::*;
use crate::*;

/// Anything complex that can be parsed with parser combinators must implement this trait.
pub trait Parse {
    fn parse(s: &str) -> IResult<&str, Self> where Self: Sized;
}

#[derive(Debug, PartialEq, Clone, Eq)]
pub struct Header {
    pub version: u8,
    pub type_id: u8,
}

impl Parse for Header {

    /// Parse 6 bits of data into a header.
    fn parse(s: &str) -> IResult<&str, Self> {
        let (rest, version) = map(
            take_while_m_n(3, 3, is_binary), 
            |hex_str: &str| u8::from_str_radix(hex_str, 2).unwrap()
        )(s)?;

        let (rest, type_id) = map(
            take_while_m_n(3, 3, is_binary), 
            |hex_str: &str| u8::from_str_radix(hex_str, 2).unwrap()
        )(rest)?;

        Ok((rest, Self { version, type_id }))
    }
}


impl From<u8> for Header {
    fn from(s: u8) -> Self {
        Self {
            version: (s >> 5) & 0b111,
            type_id: (s >> 2) & 0b111,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PacketLiteral {
    pub header: Header,
    // pub extra_end_bits: Option<String>,
    pub value: usize
}


#[derive(Debug, Clone)]
pub enum Packet {
    Literal(PacketLiteral),
    Operator(PacketOperator),
}

impl Packet {
    pub fn sum_version_numbers(&self) -> usize {
        match self {
            Packet::Literal(l) => l.header.version as usize,
            Packet::Operator(o) => o.header.version as usize + o.subpackets.iter().map(|c| c.sum_version_numbers()).sum::<usize>(),
        }
    }
}

impl Parse for Packet {
    fn parse(s: &str) -> IResult<&str, Self> where Self: Sized {

        let (_, header) = Header::parse(s)?;

        if header.type_id == 4 {
            map(PacketLiteral::parse, Packet::Literal)(s)
        } else {
            map(PacketOperator::parse, Packet::Operator)(s)
        }
    }
}


impl Parse for PacketLiteral {
    fn parse(s: &str) -> IResult<&str, Self> where Self: Sized {

        let (rest, header) = Header::parse(s)?;
        let (rest, mut bits): (&str, String) = map(
            many0(
                preceded(
                    tag("1"),
                    take_while_m_n(4, 4, is_binary)
                )
            ),
            |bytes: Vec<&str>| {
                bytes.join("")
            }
        )(rest)?;

        let (rest, _) = map(
            preceded(
                tag("0"),
                take_while_m_n(4, 4, is_binary)
            ),
            |byte: &str| {
                bits.extend(byte.chars());
            }
        )(rest)?;

        // let (rest, extra_bits) = take_while(is_zero)(rest)?;
        let value = usize::from_str_radix(&bits, 2).unwrap();
        
        // let extra_bits = if extra_bits.len() > 0 {
        //     Some(extra_bits.to_string())
        // } else {
        //     None
        // };

        Ok((rest, PacketLiteral { header, value }))


    }
}


#[derive(Debug, Clone)]
pub struct PacketOperator {
    pub header: Header,
    // pub extra_end_bits: Option<String>,
    pub length_type_id: u8,
    pub subpackets: Vec<Packet>
}

impl Parse for PacketOperator {
    fn parse(s: &str) -> IResult<&str, Self> where Self: Sized {
        let (rest, header) = Header::parse(s)?;
        if header.type_id == 4 {
            return Err(nom::Err::Error(nom::error::Error::new(s, nom::error::ErrorKind::Tag)));
        }
        let (rest, length_type_char) = one_of("01")(rest)?;

        let length_type_id: u8 = if is_zero(length_type_char) {
            0
        } else {
            1
        };

        let (rest, child_packets) = match length_type_id == 0 {
            true => {
                let (rest, length_as_bits) = take_while_m_n(15, 15, is_binary)(rest)?;
                let length = u16::from_str_radix(length_as_bits, 2).unwrap();

                let (rest, all_subpackets) = take(length)(rest)?;
                let (_, child_packets) = many1(
                    Packet::parse
                )(all_subpackets)?;
                (rest, child_packets)
            },
            false => {
                let (rest, count_as_bits) = take_while_m_n(11, 11, is_binary)(rest)?;
                let packet_count = u16::from_str_radix(count_as_bits, 2).unwrap();
                count(Packet::parse, packet_count.into())(rest)?
            }
        };

        // let (rest, extra_bits) = take_while(is_zero)(rest)?;

        Ok((
            rest,
            PacketOperator {
                header,
                // extra_end_bits: { if extra_bits.len() > 0 {Some(extra_bits.to_string())} else {None} },
                length_type_id,
                subpackets: child_packets
            }
        ))
    }
}


#[cfg(test)]
mod tests {

    use super::*;
    use test_case::test_case;

    #[test_case("110100101111111000101000", PacketLiteral { header: Header { version: 6, type_id: 4 }, value: 0b11111100101})]
    fn parse_packet_literal(raw_bin: &str, packet_literal: PacketLiteral) {
        let (_, lit) = PacketLiteral::parse(raw_bin).unwrap();
        assert_eq!(lit.value, packet_literal.value);
        assert_eq!(lit.header.version, packet_literal.header.version);
        assert_eq!(lit.header.type_id, packet_literal.header.type_id);
    }

    #[test_case("0052E4A00905271049796FB8872A0D25B9FB746893847236200B4F0BCE5194401C9B9E3F9C63992C8931A65A1CCC0D222100511A00BCBA647D98BE29A397005E55064A9DFEEC86600BD002AF2343A91A1CCE773C26600D126B69D15A6793BFCE2775D9E4A9002AB86339B5F9AB411A15CCAF10055B3EFFC00BCCE730112FA6620076268CE5CDA1FCEB69005A3800D24F4DB66E53F074F811802729733E0040E5C5E5C5C8015F9613937B83F23B278724068018014A00588014005519801EC04B220116CC0402000EAEC03519801A402B30801A802138801400170A0046A800C10001AB37FD8EB805D1C266963E95A4D1A5FF9719FEF7FDB4FB2DB29008CD2BAFA3D005CD31EB4EF2EBE4F4235DF78C66009E80293AE9310D3FCBFBCA440144580273BAEE17E55B66508803C2E0087E630F72BCD5E71B32CCFBBE2800017A2C2803D272BCBCD12BD599BC874B939004B5400964AE84A6C1E7538004CD300623AC6C882600E4328F710CC01C82D1B228980292ECD600B48E0526E506F700760CCC468012E68402324F9668028200C41E8A30E00010D8B11E62F98029801AB88039116344340004323EC48873233E72A36402504CB75006EA00084C7B895198001098D91AE2190065933AA6EB41AD0042626A93135681A400804CB54C0318032200E47B8F71C0001098810D61D8002111B228468000E5269324AD1ECF7C519B86309F35A46200A1660A280150968A4CB45365A03F3DDBAE980233407E00A80021719A1B4181006E1547D87C6008E0043337EC434C32BDE487A4AE08800D34BC3DEA974F35C20100BE723F1197F59E662FDB45824AA1D2DDCDFA2D29EBB69005072E5F2EDF3C0B244F30E0600AE00203229D229B342CC007EC95F5D6E200202615D000FB92CE7A7A402354EE0DAC0141007E20C5E87A200F4318EB0C", 986)]
    #[test_case("A0016C880162017C3686B18A3D4780", 31)]
    #[test_case("C0015000016115A2E0802F182340", 23)]
    #[test_case("620080001611562C8802118E34", 12)]
    #[test_case("38006F45291200", 9)]
    fn parse_packet_from_hex(raw: &str, sum_version_number: usize) {
        let bin = raw.chars().map(|c| format!("{:04b}", c.to_digit(16).unwrap())).collect::<String>();
        let (_, packet) = Packet::parse(&bin).unwrap();
        assert_eq!(packet.sum_version_numbers(), sum_version_number);
    }

    #[test]
    fn parse_packet() {
        let raw: &str = "00111000000000000110111101000101001010010001001000000000";
        let (_, packet) = Packet::parse(raw).unwrap();
        assert!(matches!(packet, Packet::Operator(_)));

        let raw = "11101110000000001101010000001100100000100011000001100000";
        let (_, packet) = Packet::parse(raw).unwrap();
        assert!(matches!(packet, Packet::Operator(_)));

    }
}
