//! # Double Exponential Moving Average (DEMA) Indicator
//!
//! The **Double Exponential Moving Average (DEMA)** is a technical indicator that reduces lag
//! compared to a traditional Exponential Moving Average (EMA). It achieves this by 
//! applying EMA twice and using the following formula:
//!
//! ```text
//! DEMA = 2 * EMA1 - EMA2
//! ```
//!
//! Where:
//! - **EMA1** is the first EMA applied to the price data.
//! - **EMA2** is the second EMA applied to **EMA1**.
//!
//! ## Advantages of DEMA
//! - **Faster reaction** to price changes compared to a standard EMA.
//! - **Reduces lag**, making it useful for trend-following strategies.
//! - **Can be used in crossover strategies** for better signal accuracy.
//!
//! ## Performance Considerations
//! - **Computational Complexity**: Requires two EMA calculations but remains efficient.
//! - **Rolling Iterator Approach**: Uses an iterator for sequential computation, making it efficient for streaming data analysis.
//!
//! ## Example Usage
//! ```rust
//! use tarq::*;
//! use tarq::indicators::dema::Dema;
//!
//! let price_data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0];
//! let period = 3;
//!
//! let mut dema = Dema::new(&price_data, period).unwrap();
//!
//! let dema_values = dema.calculate().unwrap();
//!
//! println!("DEMA Values: {:?}", dema_values);
//! ```
//!
//! ## Struct Definition

use crate::Indicator;

/// **The Double Exponential Moving Average (DEMA) Indicator**
///
/// DEMA is a modified Exponential Moving Average (EMA) that smooths the price series 
/// while reducing lag. It calculates two EMAs and applies the formula:
///
/// `DEMA = 2 * EMA1 - EMA2`
#[derive(Clone, Debug)]
pub struct Dema<'a> {
    /// Reference to the input price data.
    data: &'a [f64],
    /// Current index in the iteration process.
    index: usize,
    /// The lookback period for calculating the DEMA.
    period: usize,
    /// First EMA applied to the price data.
    prev_ema1: f64,
    /// Second EMA applied to `ema1`.
    prev_ema2: f64,
    /// The smoothing factor used in the EMA formula.
    smoothing: f64,
}

impl<'a> Dema<'a> {
    /// Creates a new instance of the Double Exponential Moving Average (DEMA).
    ///
    /// # Arguments
    /// - `data`: A reference to the input price data.
    /// - `period`: The lookback period for calculating the DEMA.
    ///
    /// # Errors
    /// Returns an error if:
    /// - The `period` is zero.
    /// - The `data` length is shorter than the `period`.
    ///
    /// # Example
    /// ```rust
    /// use tarq::*;
    /// use tarq::indicators::dema::Dema;
    ///
    /// let period = 3;
    /// let price_data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    /// let dema = Dema::new(&price_data, period);
    ///
    /// assert!(dema.is_ok());
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
            index: 0,
            period,
            prev_ema1: 0.0,
            prev_ema2: 0.0,
            smoothing: 2.0 / (period as f64 + 1.0),
        })
    }
}

impl Iterator for Dema<'_> {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index + (2 * self.period - 2) >= self.data.len() {
            return None;
        }

        if self.index == 0 {
            // Step 1: Compute the first EMA1 as SMA of the first `period` values
            let sum: f64 = self.data[..self.period].iter().sum();
            self.prev_ema1 = sum / self.period as f64;

            // Step 2: Collect `period` EMA1 values
            let mut ema1_values = Vec::with_capacity(self.period);
            ema1_values.push(self.prev_ema1);

            for i in self.period..(2 * self.period - 1) {
                let ema1 = (self.data[i] - self.prev_ema1) * self.smoothing + self.prev_ema1;

                self.prev_ema1 = ema1;
                ema1_values.push(ema1);
            }

            // Step 3: Compute the first EMA2 as SMA of `period` EMA1 values
            let sum_ema1: f64 = ema1_values.iter().sum();
            self.prev_ema2 = sum_ema1 / self.period as f64;

            // Step 4: Compute the first DEMA value
            let dema = 2.0 * self.prev_ema1 - self.prev_ema2;
            self.index += 1;
            return Some(dema);
        }

        // Offset index to start after initialization phase
        let price_index = self.index + (2 * self.period - 2);


        let price = self.data[price_index];

        // Compute EMA1
        self.prev_ema1 = (price - self.prev_ema1) * self.smoothing + self.prev_ema1;

        // Compute EMA2
        self.prev_ema2 = (self.prev_ema1 - self.prev_ema2) * self.smoothing + self.prev_ema2;

        // Compute DEMA
        let dema = 2.0 * self.prev_ema1 - self.prev_ema2;

        self.index += 1;
        Some(dema)
    }
}


impl<'a> Indicator<'a> for Dema<'a> {
    type Output = Vec<f64>;

    /// Computes the Double Exponential Moving Average (DEMA) for the given data.
    ///
    /// Returns a vector containing the DEMA values over the dataset.
    ///
    /// # Example
    /// ```rust
    /// use tarq::*;
    /// use tarq::indicators::dema::Dema;
    ///
    /// let period = 3;
    /// let price_data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    /// let mut dema = Dema::new(&price_data, period).unwrap();
    ///
    /// let dema_values = dema.calculate().unwrap();
    ///
    /// println!("DEMA Values: {:?}", dema_values);
    /// ```
    fn calculate(&mut self) -> Result<Self::Output, String> {
        let result: Vec<f64> = self.collect();
        Ok(result)
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dema() {
        let data = [
            5.29411352124624, 12.669143122046927, 9.869522455185985, 8.162828597722068,
            2.4970385976631873, 2.496729860303394, 1.243470235752953, 11.58705466591917,
            8.194272150313072, 9.563328995789382, 0.7634815269862714, 12.914846107673528,
            11.155265802245399, 3.217940616681935, 2.827359580250888, 2.8475777261239528,
            4.394300709882083, 7.216882324892644, 6.028896238619082, 4.227732994534937,
            8.331717052446457, 2.2855214163461355, 4.239451501250793, 5.189431594159254,
            6.337695797978061, 10.550252305830575, 3.055824411627005, 7.0822008116942285,
            8.082906481434144, 1.0945652828159709
        ];
        let expected = [
            8.095370599962308, 9.097023033035292, 4.705209516483122, 9.066080721786538,
            10.454341686226538, 6.7396019135556005, 4.480321294698639, 3.3037664657530526,
            3.5894561172396315, 5.373957369866473, 5.7118462069959985, 4.896600558076049,
            6.739941683669015, 4.377070760407648, 4.180028247898348, 4.626823099882522,
            5.525835202250017, 8.355912040523418, 5.69396850134374, 6.454631140666037,
            7.418380982556809, 4.038138635151205
        ];
        
        let mut dema = Dema::new(&data, 5).expect("Failed to create Dema");
        let result: Vec<f64> = dema.calculate().expect("Calculation failed");


        println!("Result: {:?}", result);
        
        assert_eq!(result.len(), expected.len());
        for (r, e) in result.iter().zip(expected.iter()) {
            assert!((r - e).abs() < 1e-6, "Expected {}, got {}", e, r);
        }
    }
}
