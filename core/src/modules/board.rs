use crate::{Info, Module};

pub struct Board;

impl Module for Board {
    fn name(&self) -> &'static str {
        "board"
    }

    fn detect(&self) -> Option<Info> {
        let vendor = std::fs::read_to_string("/sys/class/dmi/id/board_vendor").ok()?;
        let name = std::fs::read_to_string("/sys/class/dmi/id/board_name").ok()?;

        let vendor = vendor.trim();
        let name = name.trim();

        if vendor.is_empty() && name.is_empty() {
            return None;
        }

        let value = if vendor.is_empty() || name.starts_with(vendor) {
            name.to_string()
        } else if name.is_empty() {
            vendor.to_string()
        } else {
            format!("{vendor} {name}")
        };

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
