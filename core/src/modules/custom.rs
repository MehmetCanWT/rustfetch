use crate::{Info, Module};

pub struct Custom;

impl Module for Custom {
    fn name(&self) -> &'static str {
        "custom"
    }

    fn detect(&self) -> Option<Info> {
        Some(Info::new("", ""))
    }
}
