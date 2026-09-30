use crate::{Info, Module};

pub struct BreakLine;

impl Module for BreakLine {
    fn name(&self) -> &'static str {
        "break"
    }

    fn detect(&self) -> Option<Info> {
        Some(Info::new("", ""))
    }
}
