//! # Tarq - A Technical Analysis Library
//!
//! Tarq is a Rust-based technical analysis library designed for analyzing price charts. 
//! It provides a variety of technical indicators with a focus on efficient computation, 
//! cross-language compatibility (especially Python), and ease of use. The library prioritizes 
//! memory efficiency while maintaining reasonable performance in a safe Rust environment.
//!
//! ## Features
//! - **Multiple Indicators**: Implements various moving averages, Bollinger Bands, and more.
//! - **Optimized for Memory**: Uses efficient algorithms that generally consume less memory than TA-Lib.
//! - **Safe & Reliable**: Written in Rust to ensure memory safety without requiring unsafe code.
//! - **Python Compatibility**: Designed to integrate seamlessly with Python through the `pytarq` wrapper.
//!
//! ## Performance Considerations
//! - Tarq generally **uses less memory** than TA-Lib due to Rust's efficient memory management.
//! - However, due to Rust’s strict safety features, it is typically **slower than TA-Lib** in raw execution speed.
//! - Despite this, Tarq remains a robust and modern alternative, eliminating C-based dependency complexities.
//!
//! ## Getting Started
//! To use Tarq, add it as a dependency in your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! tarq = "0.1.0"
//! ```
//!
//! For Python compatibility, install the `pytarq` wrapper, which exposes Tarq’s functionality via PyO3.
//!
//! ## Example Usage
//! ```rust
//! use tarq::*;
//! use tarq::indicators::sma::Sma;
//! 
//!
//! let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
//! let mut sma = Sma::new(&data, 3).unwrap();
//! let result = sma.calculate();
//!
//! match result {
//!     Ok(values) => println!("SMA: {:?}", values),
//!     Err(err) => eprintln!("Error: {}", err),
//! }
//! ```
//!
//! ## Library Modules
//! - [`indicators`]: Contains implementations of various technical indicators.
//! - [`enums`]: Defines enums for handling different indicator types.
//!
//! ## Indicator Trait
//! The `Indicator` trait defines a common interface for all technical indicators in the library.
//! Each indicator implements this trait, allowing it to be iterated over and computed dynamically.
//!
//! ```rust
//! pub trait Indicator<'a>: Iterator {
//!     type Output;
//!
//!     /// Calculates the indicator values based on the input data.
//!     fn calculate(&mut self) -> Result<Self::Output, String>;
//! }
//! ```
//!
//! ## License
//! Tarq is licensed under the MIT license, making it free to use and modify.


pub mod indicators;
pub mod enums;
mod python_api;

pub trait Indicator<'a>: Iterator {
    type Output;

    fn calculate(&mut self) -> Result<Self::Output, String>;

}


