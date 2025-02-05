// Copyright 2024 Felipe Torres González

//! Finance Library implementation for Ibex indexes and companies.
//!
//! The [Finance Library][financelib] defines an API that needs to be implemented for
//! a particular use case. This library implements such API for [Ibex indexes][ibexindexes]
//! and companies that are included in these indexes.
//!
//! [financelib]: https://github.com/felipet/finance_api
//! [ibexindexes]: https://www.bolsasymercados.es/bme-exchange/en/Indices/Ibex
mod ibex35_market;
mod ibex_company;
pub use ibex35_market::Ibex35Market;
pub use ibex_company::IbexCompany;

use finance_api::{Company, Market};
use log::{debug, error, info};
use std::collections::HashMap;
use std::fs::read_to_string;
use toml::Table;

/// Helper function to build an [Ibex35Market] object from a file.
///
/// # Description
///
/// This function parses a TOML file with descriptors for companies, and builds
/// a HashMap with the tickers as keys, and [IbexCompany] as values. This collection
/// can be fed straight to [Ibex35Market::new].
///
/// An example of descriptor would be:
///
/// ```toml
/// [<BME TICKER>]
/// full_name = <Full name of the company (legal name)>
/// name = <Most used contraction of the name>
/// isin = <ISIN>
/// ticker = <BME TICKER>
/// extra_id = <NIF>
/// ```
///
/// ## Arguments
///
/// - _path_: a string that points to the TOML file.
///
/// ## Returns
///
/// An `enum` `Result<T, &str>` in which `T` implements the [Market] trait, and
/// the `str` indicates an error message.
pub fn load_ibex35_companies(path: &str) -> Result<Box<dyn Market>, &'static str> {
    info!("File {path} will be parsed to find stock descriptors.");

    let toml_parsed = match read_to_string(path) {
        Ok(data) => data,
        Err(_) => return Err("Error opening the input file"),
    };

    let table = match toml_parsed.parse::<Table>() {
        Ok(data) => data,
        Err(_) => return Err("Could not parse the file as a TOML table"),
    };

    let mut map: HashMap<String, Box<dyn Company>> = HashMap::new();

    for key in table.keys() {
        debug!("Found company descriptor for {key}");
        let fname = table[key]["full_name"].as_str().unwrap();
        let sname = table[key]["full_name"].as_str().unwrap();
        let ticker = table[key]["ticker"].as_str().unwrap();
        let isin = table[key]["isin"].as_str().unwrap();
        let nif = table[key]["extra_id"].as_str().unwrap();

        let company = IbexCompany::new(Some(fname), sname, ticker, isin, Some(nif));

        map.insert(String::from(ticker), Box::new(company));
    }

    Ok(Ibex35Market::new(map))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_FILE_PATH: &str = "./tests/data/ibex35.toml";
    const TEST_FILE_COMPANIES: usize = 35;

    /// Test case to load a TOML file and build an Ibex35Market.
    #[test]
    fn load_from_file() -> Result<(), &'static str> {
        let market = load_ibex35_companies(TEST_FILE_PATH)?;
        println!("Parsed companies:");
        println!("{:#?}", market.get_companies());
        assert_eq!(market.list_tickers().len(), TEST_FILE_COMPANIES);

        Ok(())
    }
}
