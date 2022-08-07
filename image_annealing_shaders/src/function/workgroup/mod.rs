use std::io::Write;

pub fn reduce_partial_sum<W: Write>(mut writer: W, n_elements: u32) -> std::io::Result<()> {
    assert!(n_elements.is_power_of_two());
    writeln!(
        writer,
        "fn reduce_partial_sum(id : u32) {{
  // Unrolled loop for reducing `partial_sum`
  // Note: Synchronization operations must only be executed in uniform control flow"
    )?;
    let mut i = n_elements / 2u32;
    while i > 0 {
        writeln!(
            writer,
            "  if id < {i}u {{
    partial_sum[id] += partial_sum[id + {i}u];
  }}
  workgroupBarrier();",
            i = i
        )?;
        i /= 2u32;
    }
    writeln!(writer, "}}")
}
