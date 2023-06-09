use std::cmp::max;
use std::collections::HashMap;
use std::fmt::Debug;
use std::str::FromStr;
use std::{
    io::Write,
    process::{Command, Stdio},
};

use anyhow::{anyhow, Context};
use serde::{Deserialize, Deserializer};
use which::which;

pub mod parse_error;
pub mod parse_odata;

pub fn default_true() -> bool {
    true
}
pub fn default_false() -> bool {
    false
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Find longest keyname in hashmap
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
pub fn longest(m: &HashMap<&str, &str>) -> usize {
    m.iter().fold(0, |max_len, e| max(max_len, e.0.len()))
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Append entity definition to output buffer
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
pub fn write_entity<T: Debug>(out_buf: &mut Vec<u8>, maybe_entity: Option<&Vec<T>>) {
    match maybe_entity {
        Some(entity) => {
            if entity.len() > 0 {
                for e in entity {
                    out_buf.append(&mut format!("{:#?}\n", e).as_bytes().to_vec());
                }
                out_buf.append(&mut vec![10]); // Add line feed
            }
        },
        None => {},
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Deserialize string to Boolean
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
pub fn de_str_to_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    bool::from_str(&s).or_else(|_| Ok(false))
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Pass the generated source code through rustfmt
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
pub fn run_rustfmt(buffer: &Vec<u8>) -> Result<Vec<u8>, anyhow::Error> {
    let rustfmt_path = which("rustfmt").with_context(|| "Cannot find `rustfmt` in the path.  Is it installed?")?;

    let mut fmt_proc = Command::new(rustfmt_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .with_context(|| "Failed to spawn `rustfmt` process")?;

    {
        let mut stdin = fmt_proc.stdin.take().unwrap();
        stdin.write(buffer.into_iter().as_slice())?;
    }

    let rustfmt_output = fmt_proc.wait_with_output()?;

    if rustfmt_output.status.success() {
        Ok(rustfmt_output.stdout)
    } else {
        let rustfmt_err_out = std::str::from_utf8(&rustfmt_output.stderr).unwrap();

        Err(anyhow!("Syntax error in generated source code:\n{}", rustfmt_err_out))
    }
}
