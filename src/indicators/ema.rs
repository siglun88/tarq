//! # Exponential Moving Average (EMA) Indicator
//!
//! The **Exponential Moving Average (EMA)** is a widely used technical indicator that gives more 
//! weight to recent prices, making it more responsive to price changes than a Simple Moving 
//! Average (SMA). EMA is used for trend-following and smoothing out price data.
//!
//! ## Formula
//! The EMA is calculated using the following formula:
//!
//! ```text
//! EMA = (Current Price - Previous EMA) * Smoothing Factor + Previous EMA
//! ```
//!
//! Where:
//! - **Smoothing Factor** = `2 / (Period + 1)`
//! - **Previous EMA** is either the first calculated SMA or the last EMA value.
//!
//! ## Advantages of EMA
//! - **More weight on recent prices** → faster reaction to trends.
//! - **Reduces lag** compared to SMA, making it useful for trading signals.
//! - **Used in combination with other indicators** (e.g., MACD, Bollinger Bands, DEMA).
//!
//! ## Performance Considerations
//! - **Computationally efficient** due to the recursive nature of the EMA formula.
//! - **Iterator-based approach** ensures memory efficiency in large datasets.
//!
//! ## Example Usage
//! ```rust
//! use tarq::*;
//! use tarq::indicators::ema::Ema;
//!
//! let price_data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0];
//! let period = 3;
//!
//! let mut ema = Ema::new(&price_data, period).unwrap();
//!
//! let ema_values = ema.calculate().unwrap();
//!
//! println!("EMA Values: {:?}", ema_values);
//! ```
//!
//! ## Struct Definition

use crate::Indicator;

/// **The Exponential Moving Average (EMA) Indicator**
///
/// EMA is a moving average that gives higher weight to recent prices, making it 
/// more responsive to price changes than a Simple Moving Average (SMA). It is commonly 
/// used in trading strategies to detect trends.
#[derive(Clone, Debug)]
pub struct Ema<'a> {
    /// Reference to the input price data.
    data: &'a [f64],
    /// The lookback period for computing the EMA.
    period: usize,
    /// Current index in the iteration process.
    index: usize,
    /// The previously computed EMA value.
    prev_ema: f64,
    /// The smoothing factor used in the EMA formula.
    smoothing: f64,
}

impl<'a> Ema<'a> {
    /// Creates a new instance of the Exponential Moving Average (EMA).
    ///
    /// # Arguments
    /// - `data`: A reference to the input price data.
    /// - `period`: The lookback period for calculating the EMA.
    ///
    /// # Errors
    /// Returns an error if:
    /// - The `period` is zero.
    /// - The `data` length is shorter than the `period`.
    ///
    /// # Example
    /// ```rust
    /// use tarq::*;
    /// use tarq::indicators::ema::Ema;
    ///
    /// let price_data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    /// let ema = Ema::new(&price_data, 3);
    ///
    /// assert!(ema.is_ok());
    /// ```
    pub fn new(data: &'a [f64], period: usize) -> Result<Self, String> {
        if period == 0 {
            return Err("Period must be greater than 0".to_string());
        }
        if data.len() < period {
            return Err("Period cannot be greater than input data length".to_string());
        }

        Ok(Self {
            data,
            period,
            index: 0,
            prev_ema: 0.0,
            smoothing: 2.0 / (period as f64 + 1.0),
        })
    }
}

impl Iterator for Ema<'_> {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index + self.period > self.data.len() {
            return None;
        }

        if self.index == 0 {
            // Calculate initial EMA as the SMA of the first `period` values
            self.prev_ema = self.data[..self.period].iter().sum::<f64>() / self.period as f64;
            self.index += 1;
            return Some(self.prev_ema);
        }

        if self.index < self.data.len() {
            // Apply the EMA formula
            self.prev_ema = (self.data[self.index + self.period - 1] - self.prev_ema) * self.smoothing + self.prev_ema;
            self.index += 1;
            return Some(self.prev_ema);
        }

        None
    }
}

impl<'a> Indicator<'a> for Ema<'a> {
    type Output = Vec<f64>;

    /// Computes the Exponential Moving Average (EMA) for the given data.
    ///
    /// Returns a vector containing the EMA values over the dataset.
    ///
    /// # Example
    /// ```rust
    /// use tarq::*;
    /// use tarq::indicators::ema::Ema;
    ///
    /// let price_data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    /// let mut ema = Ema::new(&price_data, 3).unwrap();
    ///
    /// let ema_values = ema.calculate().unwrap();
    ///
    /// println!("EMA Values: {:?}", ema_values);
    /// ```
    fn calculate(&mut self) -> Result<Self::Output, String> {
        Ok(self.collect()) // Collect all EMA values
    }
}





#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ema_valid() {
        let input_data = vec![
            5.2, 12.6, 9.8, 8.1, 2.4, 2.5, 1.2, 11.5, 8.1, 9.5, 
            0.7, 12.9, 11.1, 3.2, 2.8, 2.8, 4.3, 7.2, 6.0, 4.2, 
            8.3, 2.2, 4.2, 5.1, 6.3, 10.5, 3.0, 7.0, 8.0, 1.0
        ];
        let expected_output = vec![
            7.62, 5.913333333333334, 4.342222222222222, 6.728148148148148, 7.185432098765432,
            7.956954732510288, 5.537969821673525, 7.991979881115683, 9.027986587410455,
            7.08532439160697, 5.65688292773798, 4.704588618491987, 4.569725745661325, 5.446483830440883,
            5.630989220293922, 5.153992813529282, 6.2026618756861875, 4.868441250457458, 4.645627500304972,
            4.797085000203315, 5.29805666680221, 7.0320377778681395, 5.688025185245427, 6.125350123496951,
            6.750233415664634, 4.833488943776423
        ]; // Expected EMA values

        let mut indicator = Ema::new(&input_data, 5).unwrap();

        let output = indicator.calculate().unwrap();

        assert_eq!(output, expected_output, "EMA calculation is incorrect!");
    }

    #[test]
    fn test_ema_invalid_input() {
        let input_data = vec![];

        assert!(
            Ema::new(&input_data, 3).is_err(),
            "EMA should return an error for empty input."
        );
    }

    #[test]
    fn test_ema_short_data() {
        let input_data = vec![1.0, 2.0];

        assert!(
            Ema::new(&input_data, 3).is_err(),
            "EMA should return an error when data is shorter than the period."
        );
    }
}
