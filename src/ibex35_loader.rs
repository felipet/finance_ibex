// Copyright 2024 Felipe Torres González

use crate::ibex35_market::Ibex35Market;
use crate::ibex_company::IbexCompany;
use finance_api::company::Company;
use std::collections::HashMap;
use std::fs::read_to_string;
use toml::Table;

pub fn load_ibex35_desc(path: &str) -> Ibex35Market {
    let toml_parsed = read_to_string(path).ok().unwrap();
    let table = toml_parsed.parse::<Table>().unwrap();
    let mut map: HashMap<String, Box<dyn Company>> = HashMap::new();

    for key in table.keys() {
        println!("key: {key}");
        let fname = table[key]["full_name"].as_str().unwrap();
        let sname = table[key]["full_name"].as_str().unwrap();
        let ticker = table[key]["ticker"].as_str().unwrap();
        let isin = table[key]["isin"].as_str().unwrap();
        let nif = table[key]["extra_id"].as_str();

        let company = IbexCompany::new(fname, sname, ticker, isin, nif);

        map.insert(String::from(ticker), Box::new(company));
    }

    println!("{:#?}", map);
    let ib = Ibex35Market::new(map);
    ib
}

#[cfg(test)]
mod tests {
    use super::*;
    use finance_api::market::Market;

    #[test]
    fn test_1() {
        let market = load_ibex35_desc("./tests/data/ibex35.toml");

        let tickers = market.list_tickers();
        println!("{market}");

        println!("{:#?}", tickers);
    }
}
