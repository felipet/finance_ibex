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
