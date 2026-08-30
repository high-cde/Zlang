use std::error::Error;
use std::fmt;

pub const ZLB2_HEADER: [u8; 6] = *b"ZLB2\x02\x05";
pub const ZLB2_EMIT: u8 = 0x01;
pub const ZLB2_HALT: u8 = 0xff;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompileError {
    message: String,
}

impl CompileError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for CompileError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl Error for CompileError {}

#[derive(Default)]
pub struct Compiler;

impl Compiler {
    pub fn new() -> Self {
        Self
    }

    /// Compile the portable bootstrap-safe ZLB2 profile.
    ///
    /// Only `emit <text>` is accepted. This is intentionally identical to the
    /// currently executable ZDOS x86_64 profile: unsupported constructs fail
    /// at compile time rather than producing bytecode the runtime would ignore.
    pub fn compile(&self, source: &str) -> Result<Vec<u8>, CompileError> {
        let mut bytecode = Vec::from(ZLB2_HEADER);

        for (index, raw_line) in source.lines().enumerate() {
            let line_no = index + 1;
            let line = raw_line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            let text = line.strip_prefix("emit ").ok_or_else(|| {
                CompileError::new(format!(
                    "riga {line_no}: istruzione non supportata dal profilo zdos-x86_64: {line:?}"
                ))
            })?;
            let payload = text.trim().trim_matches(['\"', '\'']).as_bytes();
            let payload_len = u16::try_from(payload.len()).map_err(|_| {
                CompileError::new(format!(
                    "riga {line_no}: payload EMIT oltre il limite di {} byte",
                    u16::MAX
                ))
            })?;
            bytecode.push(ZLB2_EMIT);
            bytecode.extend_from_slice(&payload_len.to_le_bytes());
            bytecode.extend_from_slice(payload);
        }

        bytecode.extend_from_slice(&[ZLB2_HALT, 0x00, 0x00]);
        Ok(bytecode)
    }
}

#[cfg(test)]
mod tests {
    use super::{Compiler, ZLB2_HEADER};

    #[test]
    fn compiles_emit_to_zlb2() {
        let bytecode = Compiler::new()
            .compile("emit ciao\n")
            .expect("valid source");
        assert_eq!(&bytecode[..6], ZLB2_HEADER);
        assert!(bytecode.windows(4).any(|window| window == b"ciao"));
        assert_eq!(&bytecode[bytecode.len() - 3..], &[0xff, 0x00, 0x00]);
    }

    #[test]
    fn rejects_unimplemented_constructs() {
        let error = Compiler::new()
            .compile("let value = 1\n")
            .expect_err("must fail closed");
        assert!(error.to_string().contains("non supportata"));
    }
}
