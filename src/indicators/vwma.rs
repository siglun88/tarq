//! # Volume Weighted Moving Average (VWMA) Indicator
//!
//! The **Volume Weighted Moving Average (VWMA)** is a moving average that accounts for trading volume.
//! Unlike a Simple Moving Average (SMA), which gives equal weight to all prices, VWMA assigns more 
//! weight to prices with higher trading volume, making it useful for analyzing market trends and 
//! strength.
//!
//! ## Formula
//! The VWMA is calculated using the following formula:
//!
//! ```text
//! VWMA = (Σ(Price × Volume) over Period) / (Σ(Volume) over Period)
//! ```
//!
//! Where:
//! - **Price** = Closing price of the asset.
//! - **Volume** = Corresponding trading volume.
//! - **Period** = Number of data points considered for calculation.
//!
//! ## Advantages of VWMA
//! - **Emphasizes high-volume price movements**, filtering out low-volume fluctuations.
//! - **More responsive** to significant trading activity compared to SMA or EMA.
//! - **Useful for trend confirmation**, detecting accumulation/distribution phases.
//!
//! ## Performance Considerations
//! - **Computationally efficient** → Uses a rolling sum approach to update VWMA iteratively.
//! - **Memory efficient** → Uses iterator-based computation instead of storing intermediate results.
//!
//! ## Example Usage
//! ```rust
//! use tarq::*;
//! use tarq::indicators::vwma::Vwma;
//!
//! let price_data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0];
//! let volume_data = vec![100.0, 200.0, 150.0, 250.0, 300.0, 350.0, 400.0];
//! let period = 3;
//!
//! let mut vwma = Vwma::new(&price_data, &volume_data, period).unwrap();
//!
//! let vwma_values = vwma.calculate().unwrap();
//!
//! println!("VWMA Values: {:?}", vwma_values);
//! ```
//!
//! ## Struct Definition

use crate::Indicator;

/// **The Volume Weighted Moving Average (VWMA) Indicator**
///
/// VWMA is a moving average that incorporates trading volume, assigning more weight 
/// to prices with higher trading activity. It is commonly used to confirm trends and 
/// analyze market strength.
#[derive(Clone, Debug)]
pub struct Vwma<'a> {
    /// Reference to the input price data.
    data: &'a [f64],
    /// Reference to the corresponding volume data.
    volume: &'a [f64],
    /// The lookback period for computing the VWMA.
    period: usize,
    /// Current index in the iteration process.
    index: usize,
    /// Rolling sum of price × volume values.
    rolling_sum: f64,
    /// Rolling sum of volume values.
    rolling_sum_vol: f64,
}

impl<'a> Vwma<'a> {
    /// Creates a new instance of the Volume Weighted Moving Average (VWMA).
    ///
    /// # Arguments
    /// - `data`: A reference to the input price data.
    /// - `volume`: A reference to the corresponding volume data.
    /// - `period`: The lookback period for calculating the VWMA.
    ///
    /// # Errors
    /// Returns an error if:
    /// - The `period` is zero.
    /// - The `data` length is shorter than the `period`.
    /// - The `data` and `volume` arrays have different lengths.
    ///
    /// # Example
    /// ```rust
    /// use tarq::*;
    /// use tarq::indicators::vwma::Vwma;
    ///
    /// let price_data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    /// let volume_data = vec![100.0, 200.0, 150.0, 250.0, 300.0];
    /// let vwma = Vwma::new(&price_data, &volume_data, 3);
    ///
    /// assert!(vwma.is_ok());
    /// ```
    pub fn new(data: &'a [f64], volume: &'a [f64], period: usize) -> Result<Self, String> {
        if period == 0 {
            return Err("Period must be set to a number greater than 0".to_string());
        }
        if data.len() < period {
            return Err("Period cannot be greater than input data length.".to_string());
        }
        if data.len() != volume.len() {
            return Err("Data and volume must have the same length.".to_string());
        }

        Ok(Self {
            data,
            volume,
            period,
            index: 0,
            rolling_sum: 0.0,
            rolling_sum_vol: 0.0,
        })
    }
}

impl Iterator for Vwma<'_> {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index + self.period > self.data.len() {
            return None;
        }

        if self.index == 0 {
            // Compute initial rolling sums
            self.rolling_sum = self.data[..self.period]
                .iter()
                .zip(self.volume[..self.period].iter())
                .map(|(c, v)| c * v)
                .sum();
            self.rolling_sum_vol = self.volume[..self.period].iter().sum();
            self.index += 1;

            return Some(self.rolling_sum / self.rolling_sum_vol);
        }

        if self.index + self.period <= self.data.len() {
            // Slide the window forward
            let outgoing_index = self.index - 1;
            let incoming_index = self.index + self.period - 1;

            self.rolling_sum += self.data[incoming_index] * self.volume[incoming_index]
                - self.data[outgoing_index] * self.volume[outgoing_index];
            self.rolling_sum_vol += self.volume[incoming_index] - self.volume[outgoing_index];
        }

        let vwma = self.rolling_sum / self.rolling_sum_vol;
        self.index += 1;

        Some(vwma)
    }
}

impl<'a> Indicator<'a> for Vwma<'a> {
    type Output = Vec<f64>;

    /// Computes the Volume Weighted Moving Average (VWMA) for the given data.
    ///
    /// Returns a vector containing the VWMA values over the dataset.
    ///
    /// # Example
    /// ```rust
    /// use tarq::*;
    /// use tarq::indicators::vwma::Vwma;
    ///
    /// let price_data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    /// let volume_data = vec![100.0, 200.0, 150.0, 250.0, 300.0];
    /// let mut vwma = Vwma::new(&price_data, &volume_data, 3).unwrap();
    ///
    /// let vwma_values = vwma.calculate().unwrap();
    ///
    /// println!("VWMA Values: {:?}", vwma_values);
    /// ```
    fn calculate(&mut self) -> Result<Self::Output, String> {
        Ok(self.collect()) // Collect all VWMA values
    }
}






#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vwma_valid() {
        let price_data = vec![
            5.29411352124624, 12.669143122046927, 9.869522455185985, 8.162828597722068, 2.4970385976631873,
            2.496729860303394, 1.243470235752953, 11.58705466591917, 8.194272150313072, 9.563328995789382,
            0.7634815269862714, 12.914846107673528, 11.155265802245399, 3.217940616681935, 2.827359580250888,
            2.8475777261239528, 4.394300709882083, 7.216882324892644, 6.028896238619082, 4.227732994534937,
            8.331717052446457, 2.2855214163461355, 4.239451501250793, 5.189431594159254, 6.337695797978061,
            10.550252305830575, 3.055824411627005, 7.0822008116942285, 8.082906481434144, 1.0945652828159709
        ];
        let volume_data = vec![
            137.41506534163324, 82.56857195127418, 106.85847335913176, 140.35779817937657, 157.0233000165165,
            96.60203235342382, 91.07613689753174, 73.68294404898585, 137.44995543023836, 156.97394872931952,
            188.46552229665835, 82.12694392430107, 12.035494687686104, 65.82881410149886, 91.63893514289319,
            42.10079912956817, 107.33236283643453, 155.65874329777574, 88.99637196017784, 32.94671562659742,
            89.6089198575782, 157.49408516683113, 49.88676866266892, 141.40584720639913, 160.16763490226415,
            49.050590186061434, 138.54686101167783, 44.70239139442573, 66.36109747195077, 113.58610109018836
            ];
        let expected_output = [
            6.99431860447832, 6.6500626605796676, 4.9785492698023655, 4.9146418116823645, 4.905424494834933,
            6.901416341964701, 5.772273888363902, 7.336489704800355, 6.873388730781582, 6.038082997128731,
            4.112140434956763, 6.079476790655422, 3.752235655999037, 4.725522911091717, 5.168647872141773,
            5.598479417806917, 6.358658253783518, 5.537900002941467, 4.759437131142186, 4.648745772573681,
            5.123860334913925, 5.085715784991328, 5.382119995192083, 5.631245978310521, 6.121995751101593,
            4.652987289438866
            ]; // Modify for actual output

    

        let output = Vwma::new(&price_data, &volume_data, 5).unwrap().calculate().unwrap();
    
        assert_eq!(output.len(), expected_output.len(), "VWMA output length mismatch");
    
        for (i, (o, e)) in output.iter().zip(expected_output.iter()).enumerate() {
            assert!((o - e).abs() < 1e-6, "VWMA value mismatch at index {}: expected {}, got {}", i, e, o);
        }

    }

    #[test]
    fn test_vwma_invalid_input() {
        let price_data = vec![];
        let volume_data = vec![];

        assert!(
            Vwma::new(&price_data, &volume_data, 3).is_err(),
            "VWMA should return an error for empty input."
        );
    }

    #[test]
    fn test_vwma_short_data() {
        let price_data = vec![1.0, 2.0];
        let volume_data = vec![100.0, 200.0];

        assert!(
            Vwma::new(&price_data, &volume_data, 3).is_err(),
            "VWMA should return an error when data is shorter than the period."
        );
    }
}

