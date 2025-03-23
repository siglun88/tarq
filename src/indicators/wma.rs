//! # Weighted Moving Average (WMA) Indicator
//!
//! The **Weighted Moving Average (WMA)** is a moving average that assigns more weight to recent 
//! prices compared to older prices, making it more responsive to recent price changes.
//!
//! Unlike the Simple Moving Average (SMA), which gives equal weight to all prices in the period, 
//! WMA applies a linearly increasing weight to each data point, with the most recent price 
//! receiving the highest weight.
//!
//! ## Formula
//! The WMA is computed as:
//!
//! ```text
//! WMA = (Σ (Price × Weight)) / Σ Weights
//! ```
//!
//! Where:
//! - **Weight** = The position of the price in the period (1 for the oldest, period for the newest).
//! - **Σ Weights** = `(period * (period + 1)) / 2` (sum of the first `period` natural numbers).
//!
//! ## Advantages of WMA
//! - **More responsive to recent price changes** than SMA.
//! - **Smooths out price fluctuations** without lagging as much as SMA.
//! - **Useful for short-term trend detection** and crossover strategies.
//!
//! ## Performance Considerations
//! - **Computationally efficient** → Uses a rolling sum approach for fast updates.
//! - **Iterator-based approach** → Efficient memory usage for large datasets.
//!
//! ## Example Usage
//! ```rust
//! use tarq::*;
//! use tarq::indicators::wma::Wma;
//!
//! let price_data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0];
//! let period = 3;
//!
//! let mut wma = Wma::new(&price_data, period).unwrap();
//!
//! let wma_values = wma.calculate().unwrap();
//!
//! println!("WMA Values: {:?}", wma_values);
//! ```
//!
//! ## Struct Definition

use crate::Indicator;

/// **The Weighted Moving Average (WMA) Indicator**
///
/// WMA is a moving average that gives more weight to recent prices, making it more 
/// responsive to short-term price movements. It assigns weights linearly, with 
/// the most recent price having the highest weight.
#[derive(Clone, Debug)]
pub struct Wma<'a> {
    /// Reference to the input price data.
    data: &'a [f64],
    /// The lookback period for computing the WMA.
    period: usize,
    /// Current index in the iteration process.
    index: usize,
    /// Rolling weighted sum of prices.
    period_sum: f64,
    /// Rolling sum of the unweighted values (used for adjustments).
    period_sub: f64,
    /// Precomputed sum of weights for normalization.
    weight_total: f64,
}

impl<'a> Wma<'a> {
    /// Creates a new instance of the Weighted Moving Average (WMA).
    ///
    /// # Arguments
    /// - `data`: A reference to the input price data.
    /// - `period`: The lookback period for calculating the WMA.
    ///
    /// # Errors
    /// Returns an error if:
    /// - The `period` is zero.
    /// - The `data` length is shorter than the `period`.
    ///
    /// # Example
    /// ```rust
    /// use tarq::*;
    /// use tarq::indicators::wma::Wma;
    ///
    /// let price_data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    /// let wma = Wma::new(&price_data, 3);
    ///
    /// assert!(wma.is_ok());
    /// ```
    pub fn new(data: &'a [f64], period: usize) -> Result<Self, String> {
        if period == 0 {
            return Err("Period must be greater than 0.".to_string());
        }
        if data.len() < period {
            return Err("Period cannot be greater than input data length.".to_string());
        }

        let weight_total = (period * (period + 1) / 2) as f64;

        Ok(Self {
            data,
            period,
            index: 0,
            period_sum: 0.0, // Will be computed when next() is first called
            period_sub: 0.0,
            weight_total,
        })
    }
}

impl Iterator for Wma<'_> {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index + self.period > self.data.len() {
            return None;
        }

        if self.index == 0 {
            // Compute the initial weighted sum and period sum
            self.period_sum = self
                .data
                .iter()
                .take(self.period)
                .enumerate()
                .map(|(i, &price)| price * (i + 1) as f64)
                .sum::<f64>();

            self.period_sub = self.data.iter().take(self.period).sum::<f64>();

            let wma_value = self.period_sum / self.weight_total;

            self.index += 1;
            Some(wma_value)
        } else {
            // Compute periodSub: remove the contribution of the outgoing value
            let incoming_sum = self.data[self.index + self.period - 1] * self.period as f64;

            self.period_sum += incoming_sum;
            self.period_sum -= self.period_sub;
            let wma_value = self.period_sum / self.weight_total;

            self.period_sub += self.data[self.index + self.period - 1];
            self.period_sub -= self.data[self.index - 1];

            self.index += 1;
            Some(wma_value)
        }
    }
}

impl<'a> Indicator<'a> for Wma<'a> {
    type Output = Vec<f64>;

    /// Computes the Weighted Moving Average (WMA) for the given data.
    ///
    /// Returns a vector containing the WMA values over the dataset.
    ///
    /// # Example
    /// ```rust
    /// use tarq::*;
    /// use tarq::indicators::wma::Wma;
    ///
    /// let price_data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    /// let mut wma = Wma::new(&price_data, 3).unwrap();
    ///
    /// let wma_values = wma.calculate().unwrap();
    ///
    /// println!("WMA Values: {:?}", wma_values);
    /// ```
    fn calculate(&mut self) -> Result<Self::Output, String> {
        Ok(self.collect())
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wma() {
        let data = vec![
            5.29411352124624, 12.669143122046927, 9.869522455185985, 8.162828597722068,
            2.4970385976631873, 2.496729860303394, 1.243470235752953, 11.58705466591917,
            8.194272150313072, 9.563328995789382, 0.7634815269862714, 12.914846107673528,
            11.155265802245399, 3.217940616681935, 2.827359580250888, 2.8475777261239528,
            4.394300709882083, 7.216882324892644, 6.028896238619082, 4.227732994534937,
            8.331717052446457, 2.2855214163461355, 4.239451501250793, 5.189431594159254,
            6.337695797978061, 10.550252305830575, 3.055824411627005, 7.0822008116942285,
            8.082906481434144, 1.0945652828159709
        ];
        let expected = vec![
            7.0251649673401495, 5.291231834516989, 3.326037737573202, 5.57041664310442,
            6.5693658960513925, 8.022571193984401, 6.071407975774628, 8.286249506681747,
            9.136472544318117, 7.369706444344919, 5.804502101136781, 4.6951017675888975,
            3.9623360153512124, 4.738467161303144, 5.381161843654072, 5.236071736514475,
            6.365618087726449, 5.114156605150147, 4.654590437111128, 4.710179688284387,
            5.204487983694569, 6.962317594826048, 6.074102224330729, 6.476658787505759,
            7.023267286564533, 5.047530393598922
        ];

        let period = 5;
        let mut wma = Wma::new(&data, period).unwrap();
        let result = wma.calculate().unwrap();

        // Check length
        assert_eq!(result.len(), expected.len(), "WMA output length mismatch.");

        // Check values with a small floating-point tolerance
        for (i, (&actual, &exp)) in result.iter().zip(expected.iter()).enumerate() {
            assert!((actual - exp).abs() < 1e-6, "Value at index {} differs: expected {}, got {}", i, exp, actual);
        }
    }
}