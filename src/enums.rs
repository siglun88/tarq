//! # Moving Average Enum
//!
//! This module defines the `MovingAverage` enum, which provides a unified representation 
//! for different types of moving averages used in technical analysis.
//!
//! ## Overview
//! Moving averages are fundamental in financial analysis, helping smooth out price data 
//! and identify trends. The `MovingAverage` enum allows for dynamic selection of various 
//! moving average implementations within the `tarq` library.
//!
//! This enum is primarily used in indicators that support multiple types of moving averages, 
//! such as **Bollinger Bands (BBands)**. By passing a `MovingAverage` variant to BBands, 
//! the middle band will be calculated using the selected moving average type.
//!
//! ## Usage Example
//! The `MovingAverage` enum is commonly used when selecting an MA type for indicators like Bollinger Bands:
//!
//! ```rust
//! use tarq::*;
//! use tarq::indicators::{sma::Sma, bbands::BBands};
//! use tarq::enums::MovingAverage;
//!
//! let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
//! let period: usize = 3;
//! let mut sma = Sma::new(&data, period).unwrap();
//! let ma_type = MovingAverage::SMA(sma);
//!
//! let mut bbands = BBands::new(&data, period, 2.0, ma_type).unwrap();  // Selects SMA as the moving average for BBands
//!
//! let price_data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
//! let result = bbands.calculate();
//!
//! match result {
//!     Ok((lower, middle, upper)) => println!("BBands: Lower: {:?}, Middle: {:?}, Upper: {:?}", lower, middle, upper),
//!     Err(err) => eprintln!("Error: {}", err),
//! }
//! ```
//!
//! ## Enum Definition

use crate::indicators::{
    sma::Sma,
    ema::Ema,
    vwma::Vwma,
    wma::Wma,
    dema::Dema,
    tema::Tema,
    kama::Kama,
};

/// Represents different types of moving averages available in `tarq`.
///
/// This enum is primarily used in indicators that allow the user to select 
/// a specific moving average type, such as Bollinger Bands.
#[derive(Clone, Debug)]
pub enum MovingAverage<'a> {
    /// Simple Moving Average (SMA).
    SMA(Sma<'a>),
    /// Exponential Moving Average (EMA).
    EMA(Ema<'a>),
    /// Weighted Moving Average (WMA).
    WMA(Wma<'a>),
    /// Volume-Weighted Moving Average (VWMA).
    VWMA(Vwma<'a>),
    /// Double Exponential Moving Average (DEMA).
    DEMA(Dema<'a>),
    /// Triple Exponential Moving Average (TEMA).
    TEMA(Tema<'a>),
    /// Kaufman Adaptive Moving Average (KAMA).
    KAMA(Kama<'a>),
}
