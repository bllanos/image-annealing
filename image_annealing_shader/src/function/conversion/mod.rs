use std::io::Write;

pub fn u16_to_i32<W: Write>(mut writer: W) -> std::io::Result<()> {
    writeln!(
        writer,
        "// Reinterpret the two values as the first and second bytes of a big-endian
// 16-bit two's-complement value
fn u16_to_i32(x1 : u32, x2 : u32) -> i32 {{
  let x : u32 = (x1 << 8u) | x2;
  if (x & (1u << 15u)) == 0u {{
    return i32(x);
  }} else {{
    // This is a negative i16 number that must be represented as a negative i32 number
    return i32(x | (~u32(0u) << 16u));
  }}
}}"
    )
}

pub fn i32_to_u16<W: Write>(mut writer: W) -> std::io::Result<()> {
    writeln!(
        writer,
        "// Store a 16-bit two's-complement value, represented as a 32-bit two's complement value,
// in two bytes in big-endian order
fn i32_to_u16(x_signed: i32) -> vec2<u32> {{
  let x : u32 = u32(x_signed);
  return vec2<u32>((x >> 8u) & 255u, x & 255u);
}}"
    )
}
