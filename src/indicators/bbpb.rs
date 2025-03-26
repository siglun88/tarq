//! # Bollinger Bands %b (Percent Bandwidth) Indicator
//!
//! The **Bollinger Bands %b (Bbpb)** indicator measures where the current price is relative 
//! to the upper and lower Bollinger Bands. It helps identify whether a price is closer 
//! to the upper or lower band and can indicate potential overbought or oversold conditions.
//!
//! ## Formula
//! The %b value is computed as:
//! ```text
//! %b = (Price - Lower Band) / (Upper Band - Lower Band)
//! ```
//! Where:
//! - **Upper Band** = `Middle Band + (Standard Deviation × Multiplier)`
//! - **Lower Band** = `Middle Band - (Standard Deviation × Multiplier)`
//! - **Middle Band** = Configurable moving average (SMA, EMA, WMA, etc.)
//!
//! ## Interpretation
//! - **%b > 1** → Price is above the upper band (potentially overbought).
//! - **%b < 0** → Price is below the lower band (potentially oversold).
//! - **%b ≈ 0.5** → Price is near the middle band.
//!
//! ## Performance Considerations
//! - Uses a **rolling iterator-based approach**, making it efficient for streaming data analysis.
//! - **Relies on Bollinger Bands (`BBands`)** for band calculations.
//!
//! ## Example Usage
//! ```rust
//! use tarq::*;
//! use tarq::indicators::{sma::Sma, bbands::BBands, bbpb::Bbpb};
//! use tarq::enums::MovingAverage;
//!
//! let price_data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0];
//! let period = 3;
//! let std_dev_mul = 2.0;
//! let ma_type = MovingAverage::SMA(Sma::new(&price_data, period).unwrap());
//!
//! let mut bbpb = Bbpb::new(&price_data, period, std_dev_mul, ma_type).unwrap();
//!
//! let bb_percent = bbpb.calculate().unwrap();
//!
//! println!("Bollinger %b Values: {:?}", bb_percent);
//! ```
//!
//! ## Struct Definition

use crate::Indicator;
use crate::enums::MovingAverage;
use crate::indicators::bbands::BBands;

/// **The Bollinger Bands %b (Percent Bandwidth) Indicator**
///
/// The `Bbpb` struct calculates the Bollinger %b value, which measures where 
/// the current price is relative to the upper and lower Bollinger Bands.
///
/// This indicator is useful for identifying overbought and oversold conditions.
///
/// It internally relies on the [`BBands`] struct 
/// for Bollinger Band calculations.
#[derive(Clone, Debug)]
pub struct Bbpb<'a> {
    /// Reference to the input price data.
    data: &'a [f64],
    /// The lookback period for computing the Bollinger Bands.
    period: usize,
    /// Current index in the iteration process.
    index: usize,
    /// Bollinger Bands instance used for upper and lower band calculations.
    bbands: BBands<'a>,
    /// Length of the iterator when initialized.
    len: usize,
}

impl<'a> Bbpb<'a> {
    /// Creates a new instance of the Bollinger Bands %b (Bbpb) indicator.
    ///
    /// # Arguments
    /// - `data`: A reference to the input price data.
    /// - `period`: The lookback period for calculating Bollinger Bands.
    /// - `std_dev`: The standard deviation multiplier.
    /// - `ma_type`: The moving average type for the middle band.
    ///
    /// # Errors
    /// Returns an error if:
    /// - The `period` is zero.
    /// - The `data` length is shorter than the `period`.
    ///
    /// # Example
    /// ```rust
    /// use tarq::*;
    /// use tarq::indicators::{sma::Sma, bbpb::Bbpb};
    /// use tarq::enums::MovingAverage;
    ///
    /// let period = 3;
    /// let std_dev_mul = 2.0;
    /// let price_data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    /// let ma_type = MovingAverage::SMA(Sma::new(&price_data, period).unwrap());
    /// let bbpb = Bbpb::new(&price_data, period, std_dev_mul, ma_type);
    ///
    /// assert!(bbpb.is_ok());
    /// ```
    pub fn new(data: &'a [f64], period: usize, std_dev: f64, ma_type: MovingAverage<'a>) -> Result<Self, String> {
        if period == 0 {
            return Err("Period must be set to a number greater than 0".to_string());
        }
        if data.len() < period {
            return Err("Period cannot be greater than input data length.".to_string());
        }

        let bbands = BBands::new(data, period, std_dev, ma_type.clone()).unwrap();

        Ok(Self {
            data,
            period,
            index: 0,
            bbands,
            len: data.len(),
        })
    }
}

impl Iterator for Bbpb<'_> {
    type Item = f64; 

    fn next(&mut self) -> Option<Self::Item> {
        if self.index + self.period > self.data.len() {
            return None;
        }

        // Get the Bollinger Bands values
        let (upperband, _, lowerband) = self.bbands.next().unwrap();

        // Compute %b using the latest price and Bollinger Bands
        let bandwith = (self.data[self.index + self.period - 1] - lowerband) / (upperband - lowerband);

        self.index += 1;
        Some(bandwith)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.len.saturating_sub(self.period + self.index) + 1;
        (remaining, Some(remaining))
    }
}

impl<'a> Indicator<'a> for Bbpb<'a> {
    type Output = Vec<f64>;

    /// Computes the Bollinger %b values for the given data.
    ///
    /// Returns a vector containing the %b values over the dataset.
    ///
    /// # Example
    /// ```rust
    /// use tarq::*;
    /// use tarq::indicators::{sma::Sma, bbpb::Bbpb};
    /// use tarq::enums::MovingAverage;
    ///
    /// let period = 3;
    /// let std_dev_mul = 2.0;
    /// let price_data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    /// let ma_type = MovingAverage::SMA(Sma::new(&price_data, period).unwrap());
    /// let mut bbpb = Bbpb::new(&price_data, period, std_dev_mul, ma_type).unwrap();
    ///
    /// let bb_percent = bbpb.calculate().unwrap();
    ///
    /// println!("Bollinger %b Values: {:?}", bb_percent);
    /// ```
    fn calculate(&mut self) -> Result<Self::Output, String> {
        let mut result = Vec::with_capacity(self.len);
        result.extend(self);

        Ok(result)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::enums::MovingAverage;
    use crate::indicators::sma::Sma;

    #[test]
    fn test_bbpb_collect() {
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
            0.13211814773005873, 0.21373766034969283, 0.23998325259248446, 0.8997503795538926,
            0.6868774014029846, 0.682197429159456, 0.1896776722911279, 0.7539219241577635,
            0.6574867972525796, 0.27121900789113007, 0.3287437998308535, 0.2910428831480928,
            0.4612096785971347, 0.9694620053433108, 0.6963299249282948, 0.38235059933173426,
            0.8605005657588232, 0.11261304990274404, 0.4037671553335432, 0.5422853592880611,
            0.6310228706032542, 0.9380899552279673, 0.2263875099147299, 0.5648702340272342,
            0.6087068121747742, 0.1447735656356627
        ];
        
        let period = 5;
        let std_dev = 2.0;
        let ma_type = MovingAverage::SMA(Sma::new(&data, period).unwrap()); // Assuming SMA is a variant

        let mut bbpb = Bbpb::new(&data, period, std_dev, ma_type).unwrap();
        let result = bbpb.calculate().unwrap();

        assert_eq!(result.len(), expected.len(), "Bbpb output length did not match expected length.");
        for (i, (&actual, &exp)) in result.iter().zip(expected.iter()).enumerate() {
            assert!((actual - exp).abs() < 1e-6, "Value at index {} differs: expected {}, got {}", i, exp, actual);
        }
    }
}