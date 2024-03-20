use std::fmt;
use finance_api::Company;

pub struct IbexCompany {
    full_name: String,
    short_name: String,
    ticker: String,
    isin: String,
    nif: Option<String>,
}

impl IbexCompany {
    pub fn new(fname: &str, sname: &str, ticker: &str, isin: &str, nif: Option<&str>) -> IbexCompany {
        IbexCompany {
            full_name: String::from(fname),
            short_name: String::from(sname),
            ticker: String::from(ticker),
            isin: String::from(isin),
            nif: nif.map_or(None, |x| Some(String::from(x))),
        }
    }
}

impl Company for IbexCompany {
    fn name(&self) -> &str {
        &self.short_name
    }

    fn full_name(&self) -> Option<&String> {
        Some(&self.full_name)
    }

    fn isin(&self) -> &str {
        &self.isin
    }

    fn ticker(&self) -> &str {
        &self.ticker
    }

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
    use std::fmt;
    use rstest::{fixture, rstest};
    use pretty_assertions::assert_eq;

    // Fixture that builds a Company that belongs to the Ibex35 and is Spanish
    // (i.e. has NIF).
    #[fixture]
    fn spanish_company() -> IbexCompany {
        IbexCompany::new("Banco Santander", "SANTANDER", "SAN", "ES0113900J37", Some("A39000013"))
    }

    // Fixture that builds a Company that belongs to the Ibex35 but it is not
    // registered in Spain (i.e. has no NIF).
    #[fixture]
    fn foreign_company() -> IbexCompany {
        IbexCompany::new("Ferrovial S.E.", "FERROVIAL", "FER", "NL0015001FS8", None)
    }

    #[rstest]
    fn test1_trait_impl<C>(spanish_company: C)
    where C: Company + std::fmt::Display
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
    where C: Company + fmt::Display
    {
        println!("Test2 -- Test expects values for a non-Spanish company of the Ibex35");
        println!("Company -> {foreign_company}");
        assert_eq!(None, foreign_company.extra_id());
    }
}
