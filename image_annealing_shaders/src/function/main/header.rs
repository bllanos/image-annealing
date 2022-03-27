use std::io::Write;

pub const SHADER_ENTRY_POINT: &str = "main";

const GLOBAL_INVOCATION_ID: &str = "[[builtin(global_invocation_id)]] global_id: vec3<u32>";

const LOCAL_INVOCATION_ID: &str = "[[builtin(local_invocation_index)]] local_id: u32";

const NUM_WORKGROUPS: &str = "[[builtin(num_workgroups)]] num_workgroups: vec3<u32>";

const WORKGROUP_ID: &str = "[[builtin(workgroup_id)]] workgroup_id: vec3<u32>";

pub fn global_invocation_id_header<W: Write>(mut writer: W) -> std::io::Result<()> {
    writeln!(
        writer,
        "fn {}({}) {{",
        SHADER_ENTRY_POINT, GLOBAL_INVOCATION_ID
    )
}

pub fn count_swap_header<W: Write>(mut writer: W) -> std::io::Result<()> {
    writeln!(
        writer,
        "fn {}({}, {}, {}) {{",
        SHADER_ENTRY_POINT, WORKGROUP_ID, LOCAL_INVOCATION_ID, NUM_WORKGROUPS
    )
}

pub fn swap_header<W: Write>(mut writer: W) -> std::io::Result<()> {
    writeln!(
        writer,
        "fn {}({}, {}, {}, {}) {{",
        SHADER_ENTRY_POINT, WORKGROUP_ID, LOCAL_INVOCATION_ID, GLOBAL_INVOCATION_ID, NUM_WORKGROUPS
    )
}
