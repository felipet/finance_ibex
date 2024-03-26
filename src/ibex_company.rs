// Copyright 2024 Felipe Torres González

use finance_api::Company;
use std::fmt;

/// An implementation of the [Company][company] trait for a company that is included
/// in some index of the Ibex family.
///
/// A Company has several attributes:
/// - _Full name_: this is the full legal name of the company. Optional.
/// - _Short name_: This is the usual name for the company, usually some sort of contraction
///   of the _full name_.
/// - _Ticker_: the identifier of the company in the market.
/// - _ISIN_: the _International Securities Identification Number_.
/// - _NIF_: a local identifier for Spanish companies. This is optional as some companies,
///   which are included in an Ibex index, might be registered in another country.
///
/// [company]: https://docs.rs/finance_api/0.1.0/finance_api/trait.Company.html
pub struct IbexCompany {
    full_name: Option<String>,
    short_name: String,
    ticker: String,
    isin: String,
    nif: Option<String>,
}

impl IbexCompany {
    /// Constructor of the [IbexCompany] object.
    ///
    /// # Description
    ///
    /// The constructor shall receive all the information related to a stock. The only
    /// optional argument is _nif_ as it is only applicable to Spanish companies.
    ///
    /// Input values are not checked to ensure those comply with the expected format.
    pub fn new(
        fname: Option<&str>,
        sname: &str,
        ticker: &str,
        isin: &str,
        nif: Option<&str>,
    ) -> IbexCompany {
        IbexCompany {
            full_name: fname.map_or(None, |x| Some(String::from(x))),
            short_name: String::from(sname),
            ticker: String::from(ticker),
            isin: String::from(isin),
            nif: nif.map_or(None, |x| Some(String::from(x))),
        }
    }
}

impl Company for IbexCompany {
    /// Get the most common name of the stock.
    fn name(&self) -> &str {
        &self.short_name
    }

    /// Get the legal or full name of the stock.
    ///
    /// # Description
    ///
    /// This method might return `None` if a full name was not provided for a
    /// particular stock. This is common in cases in which the short name is equal
    fn full_name(&self) -> Option<&String> {
        self.full_name.as_ref()
    }

    /// Get the [ISIN](https://en.wikipedia.org/wiki/International_Securities_Identification_Number)
    /// of a stock.
    fn isin(&self) -> &str {
        &self.isin
    }

    /// Get the ticker of a stock.
    fn ticker(&self) -> &str {
        &self.ticker
    }

    /// Get the NIF of a stock.
    ///
    /// # Description
    ///
    /// Some countries add extra identity numbers to the companies, and these are useful for
    /// checking information related to the stock in national registries. As example, companies
    /// whose headquarters are registered in Spain, have an ID number called `NIF`. The property
    /// `extra_id` allows storing this information.
    ///
    /// ## Returns
    ///
    /// `None` when no special ID is linked to the stock. An ID otherwise.
    fn extra_id(&self) -> Option<&String> {
        self.nif.as_ref()
    }
}

impl fmt::Display for IbexCompany {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.ticker(), self.name())
    }
}

impl fmt::Debug for IbexCompany {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("")
            .field(&self.full_name())
            .field(&self.name())
            .field(&self.ticker())
            .field(&self.isin())
            .field(&self.extra_id())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::{fixture, rstest};
    use std::fmt;

    // Fixture that builds a Company that belongs to the Ibex35 and is Spanish
    // (i.e. has NIF).
    #[fixture]
    fn spanish_company() -> IbexCompany {
        IbexCompany::new(
            Some("Banco Santander"),
            "SANTANDER",
            "SAN",
            "ES0113900J37",
            Some("A39000013"),
        )
    }

    // Fixture that builds a Company that belongs to the Ibex35 but it is not
    // registered in Spain (i.e. has no NIF).
    #[fixture]
    fn foreign_company() -> IbexCompany {
        IbexCompany::new(
            Some("Ferrovial S.E."),
            "FERROVIAL",
            "FER",
            "NL0015001FS8",
            None,
        )
    }

    #[rstest]
    fn test1_trait_impl<C>(spanish_company: C)
    where
        C: Company + std::fmt::Display,
    {
        println!("Test1 -- Test expects values for a Spanish company of the Ibex35");
        println!("Company -> {spanish_company}");
        assert_eq!("Banco Santander", spanish_company.full_name().unwrap());
        assert_eq!("SANTANDER", spanish_company.name());
        assert_eq!("ES0113900J37", spanish_company.isin());
        assert_eq!("A39000013", spanish_company.extra_id().unwrap());
    }

    #[rstest]
    fn test2_trait_impl<C>(foreign_company: C)
    where
        C: Company + fmt::Display,
    {
        println!("Test2 -- Test expects values for a non-Spanish company of the Ibex35");
        println!("Company -> {foreign_company}");
        assert_eq!(None, foreign_company.extra_id());
    }
}
