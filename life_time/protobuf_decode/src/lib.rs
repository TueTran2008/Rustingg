use core::panic;

pub enum WireType {
    /// The Varint WireType indicates the value is a single VARINT.
    Varint,
    /// The I64 WireType indicates that the value is precisely 8 bytes in
    /// little-endian order containing a 64-bit signed integer or double type.
    //I64,  -- not needed for this exercise
    /// The Len WireType indicates that the value is a length represented as a
    /// VARINT followed by exactly that number of bytes
    Len,
    // The I32 WireType indicates that the value is precisely 4 bytes in
    // little-endian order containing a 32-bit signed integer or float type.
    //I32,  -- not needed for this exercise    // The I32 WireType indicates that the value is precisely 4 bytes in
    // little-endian order containing a 32-bit signed integer or float type.
    //I32,  -- not needed for this exercise
}

#[derive(Debug)]
pub enum FieldValue<'a> {
    Varint(u64),
    //I64(i64),  -- not needed for this exercise
    Len(&'a [u8]),
    //I32(i32),  -- not needed for this exercise
}

#[derive(Debug)]
pub struct Field<'a> {
    field_num: u64,
    value: FieldValue<'a>,
}

trait ProtoMessage<'a>: Default {
    fn add_field(&mut self, field: Field<'a>);
}

impl From<u64> for WireType {
    fn from(value: u64) -> Self {
        match value {
            0 => WireType::Varint,
            //1 => WireType::I64,
            2 => WireType::Len,
            //5 => WireType::I32
            _ => panic!("Not a valid type"),
        }
    }
}

impl<'a> FieldValue<'a> {
    fn as_str(&self) -> &'a str {
        let FieldValue::Len(data) = self else {
            panic!("Expect string to be len field");
        };
        std::str::from_utf8(data).unwrap()
    }
    fn as_bytes(&self) -> &'a [u8] {
        let FieldValue::Len(data) = self else {
            panic!("Expect bytes to be LEN field");
        };
        data
    }
    fn as_u64(&self) -> u64 {
        let FieldValue::Varint(data) = self else {
            panic!("Expect u64 to be in the Variants field ")
        };
        *data
    }
}
/// Parse a VARINT, returning the parsed value and the remaining bytes.
fn parse_variant(data: &[u8]) -> (u64, &[u8]) {
    for index in 0..7 {
        let Some(b) = data.get(index) else {
            panic!("Not enough bytes for varint");
        };
        if b & 0x80 == 0 {
            // This is the LAST BYTE of variant
            let mut value: u64 = 0;
            for b in data[..=index].iter().rev() {
                value = (value << 7) | (b & 0x7f) as u64;
            }
            return (value, &data[index + 1..]);
        }
    }
    panic!("Too many byte for varint");
}

fn unpack_tag(tag: u64) -> (u64, WireType) {
    let field_num = tag >> 3;
    let wire_type = WireType::from(tag & 0x7);
    (field_num, wire_type)
}

fn parse_field(data: &[u8]) -> (Field, &[u8]) {
    let (tag, remainder) = parse_variant(data);
    let (field_num, wire_type) = unpack_tag(tag);
    let (field_value, last_remainder) = match wire_type {
        WireType::Varint => {
            let (value, remainder) = parse_variant(remainder);
            (FieldValue::Varint(value), remainder)
        }
        WireType::Len => {
            let (len, remainder) = parse_variant(remainder);
            let len: usize = len.try_into().unwrap();
            if remainder.len() < len {
                panic!("Unxpected EOF {} < {}", remainder.len(), len);
            }
            (FieldValue::Len(&remainder[..len]), &remainder[len + 1..])
        }
    };
    (
        Field {
            field_num,
            value: field_value,
        },
        last_remainder,
    )
}

/// Parse a message in the given data, calling `T::add_field` for each field in
/// the message.
///
/// The entire input is consumed.
fn parse_message<'a, T: ProtoMessage<'a>>(mut data: &'a [u8]) -> T {
    let mut result = T::default();
    while !data.is_empty() {
        let parsed = parse_field(data);
        result.add_field(parsed.0);
        data = parsed.1;
    }
    result
}
/*
message PhoneNumber {
  optional string number = 1;
  optional string type = 2;
}

message Person {
  optional string name = 1;
  optional int32 id = 2;
  repeated PhoneNumber phones = 3;
}
*/
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Person<'a> {
    name: &'a str,
    id: u64,
    phone: Vec<PhoneNumber<'a>>,
}
// TODO: Implement ProtoMessage for Person and PhoneNumber.

#[derive(Debug, Default, Ord, Eq, PartialEq, PartialOrd)]

pub struct PhoneNumber<'a> {
    number: &'a str,
    number_type: &'a str,
}

impl<'a> ProtoMessage<'a> for Person<'a> {
    fn add_field(&mut self, field: Field<'a>) {
        match field.field_num {
            1 => self.name = field.value.as_str(),
            2 => self.id = field.value.as_u64(),
            3 => self.phone.push(parse_message(field.value.as_bytes())),
            _ => {} // skip everything else
        }
    }
}

impl<'a> ProtoMessage<'a> for PhoneNumber<'a> {
    fn add_field(&mut self, field: Field<'a>) {
        match field.field_num {
            1 => self.number = field.value.as_str(),
            2 => self.number_type = field.value.as_str(),
            _ => {} // skip everything else
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn it_works() {
    //     let person: Person = parse_message(&[
    //         0x0a, 0x07, 0x6d, 0x61, 0x78, 0x77, 0x65, 0x6c, 0x6c, 0x10, 0x2a, 0x1a, 0x16, 0x0a,
    //         0x0e, 0x2b, 0x31, 0x32, 0x30, 0x32, 0x2d, 0x35, 0x35, 0x35, 0x2d, 0x31, 0x32, 0x31,
    //         0x32, 0x12, 0x04, 0x68, 0x6f, 0x6d, 0x65, 0x1a, 0x18, 0x0a, 0x0e, 0x2b, 0x31, 0x38,
    //         0x30, 0x30, 0x2d, 0x38, 0x36, 0x37, 0x2d, 0x35, 0x33, 0x30, 0x38, 0x12, 0x06, 0x6d,
    //         0x6f, 0x62, 0x69, 0x6c, 0x65,
    //     ]);
    //     println!("{:#?}", person);
    // }
    #[test]
    fn test_id() {
        let person_id: Person = parse_message(&[0x10, 0x2a]);
        assert_eq!(
            person_id,
            Person {
                name: "",
                id: 42,
                phone: vec![]
            }
        );
    }

    #[test]
    fn test_name() {
        let person_name: Person = parse_message(&[
            0x0a, 0x0e, 0x62, 0x65, 0x61, 0x75, 0x74, 0x69, 0x66, 0x75, 0x6c, 0x20, 0x6e, 0x61,
            0x6d, 0x65,
        ]);
        assert_eq!(
            person_name,
            Person {
                name: "beautiful name",
                id: 0,
                phone: vec![]
            }
        );
    }
    #[test]
    fn test_just_person() {
        let person_name_id: Person =
            parse_message(&[0x0a, 0x04, 0x45, 0x76, 0x61, 0x6e, 0x10, 0x16]);
        assert_eq!(
            person_name_id,
            Person {
                name: "Evan",
                id: 22,
                phone: vec![]
            }
        );
    }

    #[test]
    fn test_phone() {
        let phone: Person = parse_message(&[
            0x0a, 0x00, 0x10, 0x00, 0x1a, 0x16, 0x0a, 0x0e, 0x2b, 0x31, 0x32, 0x33, 0x34, 0x2d,
            0x37, 0x37, 0x37, 0x2d, 0x39, 0x30, 0x39, 0x30, 0x12, 0x04, 0x68, 0x6f, 0x6d, 0x65,
        ]);
        assert_eq!(
            phone,
            Person {
                name: "",
                id: 0,
                phone: vec![PhoneNumber {
                    number: "+1234-777-9090",
                    number_type: "home"
                },],
            }
        );
    }
}
