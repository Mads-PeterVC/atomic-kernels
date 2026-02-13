use once_cell::sync::Lazy;
use std::num::{ParseFloatError, ParseIntError};

use crate::AtomicNumber;

const DATA: &str = include_str!("atom_info.csv");

pub struct AtomInfo {
    pub number: AtomicNumber,
    pub symbol: String,
    pub covalent_radius: f64,
    pub vdw_radius: f64,
}

pub struct PeriodicTable {
    info_arr: [AtomInfo; 110],
}

impl PeriodicTable {
    pub fn from_data() -> Result<Self, String> {
        let mut info = Vec::new();

        for (i, line) in DATA.lines().enumerate() {
            if i == 0 || line.trim().is_empty() {
                continue; // skip header/empty
            }
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() != 4 {
                return Err(format!("bad row {}: {}", i + 1, line));
            }
            let number_u8: u8 = parts[0]
                .parse::<u8>()
                .map_err(|e: ParseIntError| e.to_string())?;
            let number: AtomicNumber = AtomicNumber::new(number_u8).expect("Invalid atomic number");
            let symbol = parts[1].to_string();
            let covalent_radius: f64 = parts[2]
                .parse()
                .map_err(|e: ParseFloatError| e.to_string())?;
            let vdw_radius: f64 = parts[3]
                .parse()
                .map_err(|e: ParseFloatError| e.to_string())?;

            info.push(AtomInfo {
                number,
                symbol,
                covalent_radius,
                vdw_radius,
            });
        }

        let info_arr: [AtomInfo; 110] = info
            .try_into()
            .map_err(|v: Vec<AtomInfo>| format!("expected 110 rows, got {}", v.len()))?;

        Ok(Self { info_arr })
    }

    pub fn get(&self, z: AtomicNumber) -> &AtomInfo {
        self.info_arr
            .get((z.get() - 1) as usize)
            .expect("Mismatched index due")
    }

    pub fn get_by_symbol(&self, symbol: &str) -> &AtomInfo {
        self.info_arr
            .iter()
            .find(|f| f.symbol == symbol)
            .expect("Unknown element symbol")
    }
}

pub static PERIODIC_TABLE: Lazy<PeriodicTable> =
    Lazy::new(|| PeriodicTable::from_data().expect("Failed to build periodic table"));

#[cfg(test)]
mod test {

    use crate::AtomicNumber;
    use crate::utils::atom_info::PERIODIC_TABLE;

    #[test]
    fn get_from_table() {
        let hydrogen_data = PERIODIC_TABLE.get(AtomicNumber::new(1).unwrap());
        assert_eq!(hydrogen_data.number.get(), 1);
        assert_eq!(hydrogen_data.symbol, String::from("H"));
    }

    #[test]
    fn get_by_symbol() {
        let hydrogen_data = PERIODIC_TABLE.get_by_symbol("H");
        assert_eq!(hydrogen_data.number.get(), 1);
        assert_eq!(hydrogen_data.symbol, String::from("H"));
    }
}
