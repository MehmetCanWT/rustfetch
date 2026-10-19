use crate::{Info, Module};

pub struct Board;

impl Module for Board {
    fn name(&self) -> &'static str {
        "board"
    }

    fn detect(&self) -> Option<Info> {
        let vendor = std::fs::read_to_string("/sys/devices/virtual/dmi/id/board_vendor")
            .or_else(|_| std::fs::read_to_string("/sys/class/dmi/id/board_vendor"))
            .ok();
        let name = std::fs::read_to_string("/sys/devices/virtual/dmi/id/board_name")
            .or_else(|_| std::fs::read_to_string("/sys/class/dmi/id/board_name"))
            .ok();

        let value = super::host::parse_host(
            vendor.as_deref().unwrap_or(""),
            name.as_deref().unwrap_or(""),
        )?;
        Some(Info::new("Board", value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_board_module_name() {
        assert_eq!(Board.name(), "board");
    }
}
