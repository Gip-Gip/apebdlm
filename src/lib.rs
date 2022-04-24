#[macro_export]
macro_rules! binary_data
{
    () =>
    {
        Vec::<u8>::new()
    };

    ( $($e: expr), *) =>
    {{
        let mut data = Vec::<u8>::new();
        $(
            data.extend_from_slice(&$e);
        )*

        data
    }};
}

#[macro_export]
macro_rules! byte
{
    ($e: expr) =>
    {
        [$e as u8]
    };
}

#[macro_export]
macro_rules! byte_signed
{
    ($e: expr) =>
    {
        [($e as i8) as u8]
    };
}

#[macro_export]
macro_rules! bytes
{
    ($($e: expr), *) =>
    {
        [$($e), *]
    };
}

#[macro_export]
macro_rules! bytes_from_vec
{
    ($e: expr) =>
    {
        $e
    };
}

#[macro_export]
macro_rules! u16_be
{
    ($e: expr) =>
    {
        ($e as u16).to_be_bytes()
    };
}

#[macro_export]
macro_rules! u32_be
{
    ($e: expr) =>
    {
        ($e as u32).to_be_bytes()
    };
}

#[macro_export]
macro_rules! u64_be
{
    ($e: expr) =>
    {
        ($e as u64).to_be_bytes()
    };
}

#[macro_export]
macro_rules! u128_be
{
    ($e: expr) =>
    {
        ($e as u128).to_be_bytes()
    };
}

#[macro_export]
macro_rules! u16_le
{
    ($e: expr) =>
    {
        ($e as u16).to_le_bytes()
    };
}

#[macro_export]
macro_rules! u32_le
{
    ($e: expr) =>
    {
        ($e as u32).to_le_bytes()
    };
}

#[macro_export]
macro_rules! u64_le
{
    ($e: expr) =>
    {
        ($e as u64).to_le_bytes()
    };
}

#[macro_export]
macro_rules! u128_le
{
    ($e: expr) =>
    {
        ($e as u128).to_le_bytes()
    };
}

#[macro_export]
macro_rules! i16_be
{
    ($e: expr) =>
    {
        ($e as i16).to_be_bytes()
    };
}

#[macro_export]
macro_rules! i32_be
{
    ($e: expr) =>
    {
        ($e as i32).to_be_bytes()
    };
}

#[macro_export]
macro_rules! i64_be
{
    ($e: expr) =>
    {
        ($e as i64).to_be_bytes()
    };
}

#[macro_export]
macro_rules! i128_be
{
    ($e: expr) =>
    {
        ($e as i128).to_be_bytes()
    };
}

#[macro_export]
macro_rules! i16_le
{
    ($e: expr) =>
    {
        ($e as i16).to_le_bytes()
    };
}

#[macro_export]
macro_rules! i32_le
{
    ($e: expr) =>
    {
        ($e as i32).to_le_bytes()
    };
}

#[macro_export]
macro_rules! i64_le
{
    ($e: expr) =>
    {
        ($e as i64).to_le_bytes()
    };
}

#[macro_export]
macro_rules! i128_le
{
    ($e: expr) =>
    {
        ($e as i128).to_le_bytes()
    };
}

#[macro_export]
macro_rules! f32_be
{
    ($e: expr) =>
    {
        ($e as f32).to_be_bytes()
    };
}

#[macro_export]
macro_rules! f64_be
{
    ($e: expr) =>
    {
        ($e as f64).to_be_bytes()
    };
}

#[macro_export]
macro_rules! f32_le
{
    ($e: expr) =>
    {
        ($e as f32).to_le_bytes()
    };
}

#[macro_export]
macro_rules! f64_le
{
    ($e: expr) =>
    {
        ($e as f64).to_le_bytes()
    };
}

#[cfg(test)]
mod tests
{
    #[test]
    fn empty_binary_data()
    {
        let data = binary_data!();

        assert_eq!(data.len(), 0);
    }

    #[test]
    fn binary_data_string()
    {
        let hello = "Hello, World!";
        let length = hello.len();

        let data = binary_data!
        (
            byte!(length), 
            bytes_from_vec!(hello.as_bytes())
        );

        let data2 = binary_data!
        (
            byte_signed!(length), 
            bytes!(b'H', b'e', b'l', b'l', b'o', b',', b' ', b'W', b'o', b'r', b'l', b'd', b'!')
        );

        assert_eq!(data, data2);
        assert_eq!(data[0], length as u8);
        assert_eq!(&data[1..], hello.as_bytes());
    }

    #[test]
    fn binary_data_ints_unsigned()
    {
        let data = binary_data!
        (
            byte!(0x12),
            u16_be!(0x1234),
            u32_be!(0x12345678),
            u64_be!(0x123456789abcdef0),
            u128_be!(0x123456789abcdef0123456789abcdef0),
            u16_le!(0x1234),
            u32_le!(0x12345678),
            u64_le!(0x123456789abcdef0),
            u128_le!(0x123456789abcdef0123456789abcdef0)
        );
        
        assert_eq!(data[0], 0x12);
        assert_eq!(data[1..3], 0x1234_u16.to_be_bytes());
        assert_eq!(data[3..7], 0x12345678_u32.to_be_bytes());
        assert_eq!(data[7..15], 0x123456789abcdef0_u64.to_be_bytes());
        assert_eq!(data[15..31], 0x123456789abcdef0123456789abcdef0_u128.to_be_bytes());
        assert_eq!(data[31..33], 0x1234_u16.to_le_bytes());
        assert_eq!(data[33..37], 0x12345678_u32.to_le_bytes());
        assert_eq!(data[37..45], 0x123456789abcdef0_u64.to_le_bytes());
        assert_eq!(data[45..61], 0x123456789abcdef0123456789abcdef0_u128.to_le_bytes());
    }

    #[test]
    fn binary_data_ints_signed()
    {
        let data = binary_data!
        (
            byte_signed!(-0x12),
            i16_be!(-0x1234),
            i32_be!(-0x12345678),
            i64_be!(-0x123456789abcdef0),
            i128_be!(-0x123456789abcdef0123456789abcdef0),
            i16_le!(-0x1234),
            i32_le!(-0x12345678),
            i64_le!(-0x123456789abcdef0),
            i128_le!(-0x123456789abcdef0123456789abcdef0)
        );

        assert_eq!(data[0], 0xee);
        assert_eq!(data[1..3], (-0x1234_i16).to_be_bytes());
        assert_eq!(data[3..7], (-0x12345678_i32).to_be_bytes());
        assert_eq!(data[7..15], (-0x123456789abcdef0_i64).to_be_bytes());
        assert_eq!(data[15..31], (-0x123456789abcdef0123456789abcdef0_i128).to_be_bytes());
        assert_eq!(data[31..33], (-0x1234_i16).to_le_bytes());
        assert_eq!(data[33..37], (-0x12345678_i32).to_le_bytes());
        assert_eq!(data[37..45], (-0x123456789abcdef0_i64).to_le_bytes());
        assert_eq!(data[45..61], (-0x123456789abcdef0123456789abcdef0_i128).to_le_bytes());
    }
    
    #[test]
    fn binary_data_floats()
    {
        let data = binary_data!
        (
            f32_be!(1.0),
            f64_be!(1.0),
            f32_le!(1.0),
            f64_le!(1.0)
        );
        
        assert_eq!(data[0..4], 1.0_f32.to_be_bytes());
        assert_eq!(data[4..12], 1.0_f64.to_be_bytes());
        assert_eq!(data[12..16], 1.0_f32.to_le_bytes());
        assert_eq!(data[16..24], 1.0_f64.to_le_bytes());
    }
}
