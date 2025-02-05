// Copyright 2024 Felipe Torres González

use finance_api::{Company, Market};
use std::{collections::HashMap, fmt};

/// An implementation of the [Market][market] trait for the Ibex35 index.
///
/// The Ibex35 index includes the 35 values whose negotiation is the highest for all
/// the Spanish exchanges. This usually means that the 35 most capitalized companies
/// are included in this index, however this might be not true, as the index consider
/// several aspects to include/exclude companies from it.
///
/// This implementation is mainly a container for [IbexCompany][super::IbexCompany]
/// whose key trait is that these companies must be included in the Ibex35 index.
///
/// [market]: https://docs.rs/finance_api/0.1.0/finance_api/trait.Market.html
pub struct Ibex35Market {
    name: String,
    open_time: String,
    close_time: String,
    currency: String,
    company_map: HashMap<String, Box<dyn Company>>,
}

impl Ibex35Market {
    /// Constructor of the [Ibex35Market] object.
    ///
    /// # Description
    ///
    /// The constructor shall receive a collection of companies that are part of
    /// the Ibex35 at the moment of the instantiation.
    ///
    /// Each entry of the collection is identified by the company's ticker and
    /// an object that implements the [Company] trait as value.
    ///
    /// The constructor has no logic to check whether the input companies are compliant
    /// with the invariant of the [Ibex35Market], this means that valid companies must
    /// be input at instantiation time, and external logic must ensure an instantiation
    /// of this object complies with the invariant (for example, if there's a change in
    /// the composition of the index).
    pub fn new(company_map: HashMap<String, Box<dyn Company>>) -> Box<dyn Market> {
        Box::new(Ibex35Market {
            name: String::from("BME Ibex35 Index"),
            open_time: String::from("08:00:00"),
            close_time: String::from("16:30:00"),
            currency: String::from("euro"),
            company_map,
        })
    }
}

impl Market for Ibex35Market {
    /// Get the name of the Market, for example: _NASDAQ100_ or _IBEX35_.
    fn market_name(&self) -> &str {
        &self.name
    }

    /// Get a list of the stocks included in the market.
    ///
    /// # Description
    ///
    /// This method should build a list with the ticker identifier for each stock
    /// that is included in the market.
    ///
    /// ## Returns
    ///
    /// A vector with references to the tickers.
    fn list_tickers(&self) -> Vec<&String> {
        let mut tickers = Vec::new();
        self.company_map.keys().for_each(|c| tickers.push(c));

        tickers
    }

    /// Get a reference to a [Company] object included in the market.
    ///
    /// # Description
    ///
    /// This method searches for stocks identified by `name` in the market. The given
    /// name is applied in a regular expression. This means that if the `name` is too
    /// ambiguous, multiple stocks might match it. For example, if **Bank** is given as
    /// `name`, multiple stocks might match such string.
    ///
    /// ## Returns
    ///
    /// A wrapped vector with a list of references to stock descriptors (objects that
    /// implement the [Company] trait) that match `name`. `None` is returned when no
    /// stocks have been found matching `name` with their respective names.
    fn stock_by_name(&self, name: &str) -> Option<Vec<&Box<dyn Company>>> {
        let mut stocks = Vec::new();

        for stock in self.company_map.values() {
            let stock_lowercase = stock.name().to_ascii_lowercase();
            if stock_lowercase.contains(&name.to_ascii_lowercase()) {
                stocks.push(stock);
            }
        }

        if stocks.len() > 0 {
            Some(stocks)
        } else {
            None
        }
    }

    /// Get a reference to a [Company] object included in the market.
    ///
    /// # Description
    ///
    /// This method searches for a stock whose ticker is equal to `ticker`. An
    /// exhaustive match is applied between `ticker` and the ticker of a Company.
    /// This means that partial tickers won't produce a match.
    ///
    /// ## Returns
    ///
    /// In contrast to the method [stock_by_name](Market::stock_by_name), this method will
    /// return a wrapped reference to an object that implements the `Company` trait
    /// whose ticker is equal to `ticker`, otherwise `None` will be returned.
    fn stock_by_ticker(&self, ticker: &str) -> Option<&Box<dyn Company>> {
        if let Some(stock) = self.company_map.get(ticker) {
            Some(stock)
        } else {
            None
        }
    }

    /// Get the open time of the market (UTC).
    ///
    /// # Description
    ///
    /// Ibex35 opens at 8:00:00 GMT
    fn open_time(&self) -> &str {
        &self.open_time
    }

    /// Get the close time of the market (UTC).
    ///
    /// # Description
    ///
    /// Ibex35 closes at 16:30:00 GMT
    fn close_time(&self) -> &str {
        &self.close_time
    }

    /// Get the currency code (ISO 4217) of the market.
    ///
    /// # Description
    ///
    /// Ibex35's currency is Euro
    fn currency(&self) -> &str {
        &self.currency
    }

    /// Get a reference to a [Company] object included in the market.
    ///
    /// # Description
    ///
    /// This method searches for stocks identified by `name` in the market. The given
    /// name is applied in a regular expression. This means that if the `name` is too
    /// ambiguous, multiple stocks might match it. For example, if **Bank** is given as
    /// `name`, multiple stocks might match such string.
    ///
    /// ## Returns
    ///
    /// A wrapped vector with a list of references to stock descriptors (objects that
    /// implement the [Company] trait) that match `name`. `None` is returned when no
    /// stocks have been found matching `name` with their respective names.
    fn get_companies(&self) -> Vec<&Box<dyn Company>> {
        self.company_map.values().collect()
    }
}

impl fmt::Display for Ibex35Market {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.market_name())
    }
}

impl fmt::Debug for Ibex35Market {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("")
            .field(&self.market_name())
            .field(&self.open_time())
            .field(&self.close_time())
            .field(&self.currency())
            .field(&self.get_companies())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ibex_company::IbexCompany;
    use finance_api::Company;
    use rstest::{fixture, rstest};
    use std::collections::HashMap;

    #[fixture]
    fn ibex35_companies() -> HashMap<String, Box<dyn Company>> {
        let mut companies = HashMap::<String, Box<dyn Company>>::new();

        companies.insert(
            String::from("AENA"),
            Box::new(IbexCompany::new(
                Some("AENA S.A."),
                "AENA",
                "AENA",
                "ES0105046009",
                Some("A86212420"),
            )),
        );

        companies.insert(
            String::from("AMS"),
            Box::new(IbexCompany::new(
                Some("Amadeus IT Holding S.A."),
                "AMADEUS",
                "AMS",
                "ES0109067019",
                Some("A-84236934"),
            )),
        );

        companies.insert(
            String::from("CLNX"),
            Box::new(IbexCompany::new(
                Some("Cellnex Telecom S.A."),
                "CELLNEX",
                "CLNX",
                "ES0105066007",
                Some("A64907306"),
            )),
        );

        companies
    }

    // Test case for the creation of a IbexMarket object.
    #[rstest]
    fn new(ibex35_companies: HashMap<String, Box<dyn Company>>) {
        let market = Ibex35Market::new(ibex35_companies);

        assert_eq!(market.get_companies().len(), 3);
    }

    // Test case for the implementation of the Market trait.
    #[rstest]
    fn interface(ibex35_companies: HashMap<String, Box<dyn Company>>) {
        let market = Ibex35Market::new(ibex35_companies);

        // Let's check that we get the same amount of companies using these methods:
        assert_eq!(market.get_companies().len(), market.list_tickers().len());
        // Check for the company search.
        assert!(market.stock_by_name("CELLNEX").is_some());
        assert!(market.stock_by_name("cell").is_some());
        assert!(market.stock_by_name("Grifols").is_none());
        // Check for companies by ticker.
        assert!(market.stock_by_ticker("SAN").is_none());
        assert!(market.stock_by_ticker("AENA").is_some());
        assert!(market.stock_by_ticker("CLNX").is_some());
    }
}
