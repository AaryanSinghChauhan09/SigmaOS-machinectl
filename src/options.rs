//! Options model.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Mode { Default, List, Start(String), Stop(String), Status(String), Shell(String) }

#[derive(Debug, Clone)]
pub struct Options { pub mode: Mode }
impl Default for Options { fn default() -> Self { Self { mode: Mode::Default } } }
#[cfg(test)] mod tests { use super::*; #[test] fn o() { assert_eq!(Options::default().mode, Mode::Default); } }
