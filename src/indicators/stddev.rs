//! # Standard Deviation (StdDev)
//!
//! The **Standard Deviation (StdDev)** measures the dispersion of a dataset relative 
//! to its mean. In technical analysis, it is often used to assess volatility by analyzing 
//! how much prices deviate from the average over a given period.
//!
//! ## Formula
//! The standard deviation is computed as follows:
//!
//! ```text
//! Variance = (Σ(x²) / (N - ddof)) - (Mean²)
//! StdDev = √Variance
//! ```
//!
//! Where:
//! - **x** = individual data points
//! - **N** = number of observations (period)
//! - **Mean** = Simple Moving Average (SMA) of the dataset
//! - **ddof (Delta Degrees of Freedom)** = Optional degrees of freedom (default is 0)
//!
//! ## Advantages of StdDev in Technical Analysis
//! - **Measures market volatility** → Higher values indicate more volatility.
//! - **Used in Bollinger Bands** → Determines the width of the bands.
//! - **Risk Management** → Helps identify stable or turbulent periods.
//!
//! ## Performance Considerations
//! - Uses a **rolling sum of squares approach**, optimizing standard deviation calculations.
//! - Uses **iterator-based computation**, making it efficient for real-time data processing.
//!
//! ## Example Usage
//! ```rust
//! use tarq::*;
//! use tarq::indicators::stddev::StdDev;
//!
//! let price_data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0];
//! let period = 3;
//!
//! let mut stddev = StdDev::new(&price_data, period, None).unwrap();
//!
//! let stddev_values = stddev.calculate().unwrap();
//!
//! println!("Standard Deviation Values: {:?}", stddev_values);
//! ```
//!
//! ## Struct Definition

use crate::Indicator;
use crate::indicators::sma::Sma; // Assuming Sma is implemented

/// **The Standard Deviation (StdDev) Indicator**
///
/// The `StdDev` struct calculates the rolling standard deviation of a dataset over 
/// a specified period. It measures the price dispersion around the mean and is often 
/// used in volatility analysis and indicators like Bollinger Bands.
#[derive(Clone, Debug)]
pub struct StdDev<'a> {
    /// Reference to the input price data.
    data: &'a [f64],
    /// The lookback period for computing standard deviation.
    period: usize,
    /// Current index in the iteration process.
    index: usize,
    /// Simple Moving Average (SMA) instance used for mean calculation.
    sma: Sma<'a>,
    /// Rolling sum of squared values used for variance calculation.
    sum_sq: f64,
    /// Degrees of freedom adjustment (default is 0).
    ddof: usize,
}

impl<'a> StdDev<'a> {
    /// Creates a new instance of the Standard Deviation (StdDev) indicator.
    ///
    /// # Arguments
    /// - `data`: A reference to the input price data.
    /// - `period`: The lookback period for calculating standard deviation.
    /// - `ddof`: Optional degrees of freedom (default is `0`).
    ///
    /// # Errors
    /// Returns an error if:
    /// - The `period` is zero.
    /// - The `data` length is shorter than the `period`.
    ///
    /// # Example
    /// ```rust
    /// use tarq::*;
    /// use tarq::indicators::stddev::StdDev;
    ///
    /// let price_data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    /// let stddev = StdDev::new(&price_data, 3, None);
    ///
    /// assert!(stddev.is_ok());
    /// ```
    pub fn new(data: &'a [f64], period: usize, ddof: usize) -> Result<Self, String> {
        if data.len() < period {
            return Err("Insufficient data for the given period".to_string());
        }
        if period == 0 {
            return Err("Period must be set to a number greater than 0".to_string());
        }

        let sma = Sma::new(data, period).unwrap();

        Ok(Self {
            data,
            period,
            index: 0,
            sma,
            sum_sq: 0.0,
            ddof,
        })
    }
}

impl Iterator for StdDev<'_> {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index + self.period > self.data.len() {
            return None;
        }

        if self.index == 0 {
            // Compute the initial sum of squares for the first `period` values
            self.sum_sq = self.data[..self.period].iter().map(|&x| x * x).sum::<f64>();
        } else if self.index + self.period <= self.data.len() {
            // Rolling update: Remove outgoing value and add new incoming value
            let outgoing_index = self.index - 1;
            let incoming_index = self.index + self.period - 1;

            self.sum_sq += self.data[incoming_index] * self.data[incoming_index]
                - self.data[outgoing_index] * self.data[outgoing_index];
        }

        let mean = self.sma.next()?;

        // Compute variance and standard deviation
        let variance = (self.sum_sq / (self.period - self.ddof) as f64) - (mean * mean);
        let std_dev = variance.sqrt();

        self.index += 1;
        Some(std_dev)
    }
}

impl<'a> Indicator<'a> for StdDev<'a> {
    type Output = Vec<f64>;

    /// Computes the Standard Deviation for the given data.
    ///
    /// Returns a vector containing the standard deviation values over the dataset.
    ///
    /// # Example
    /// ```rust
    /// use tarq::*;
    /// use tarq::indicators::stddev::StdDev;
    ///
    /// let price_data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    /// let mut stddev = StdDev::new(&price_data, 3, None).unwrap();
    ///
    /// let stddev_values = stddev.calculate().unwrap();
    ///
    /// println!("Standard Deviation Values: {:?}", stddev_values);
    /// ```
    fn calculate(&mut self) -> Result<Self::Output, String> {
        Ok(self.collect())
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stddev_valid_output() {
        let data = vec![10.0, 12.0, 23.0, 23.0, 16.0, 20.0, 25.0, 30.0, 28.0, 26.0];
        let period = 3;
        let mut std_dev = StdDev::new(&data, period, 0).unwrap();

        let result = std_dev.calculate().unwrap();

        let expected = [5.715476066494082, 5.185449728701348, 3.2998316455372225, 2.867441755680877, 3.6817870057290882, 4.082482904638632, 2.054804667656329, 1.6329931618554558];

        assert_eq!(result.len(), expected.len());
        for (i, (r, e)) in result.iter().zip(expected.iter()).enumerate() {
            assert!((r - e).abs() < 1e-4, "Expected {}, got {}, at index {}", e, r, i);
        }
    }

    #[test]
    fn test_stddev_invalid_input() {
        let data = vec![10.0, 12.0, 23.0, 23.0];
        let period = 5; // Period larger than dataset
        let std_dev = StdDev::new(&data, period, 0);

        assert!(std_dev.is_err(), "Expected error for invalid input");
    }

    #[test]
    fn test_stddev_too_short_data() {
        let data = vec![10.0, 12.0]; // Too short for std dev calculation
        let period = 3;
        let std_dev = StdDev::new(&data, period, 0);

        assert!(std_dev.is_err(), "Expected error for too short data");
    }
}