//! # Kaufman Adaptive Moving Average (KAMA) Indicator
//!
//! The **Kaufman Adaptive Moving Average (KAMA)** is a trend-following moving average that dynamically adjusts its smoothing factor
//! based on market conditions. It reduces lag during strong trends and filters out noise during sideways movements, making it
//! an effective tool for trend identification and risk management.
//!
//! ## Calculation Steps
//! 1. **Compute the Efficiency Ratio (ER):**
//!    ```text
//!    ER = |Price_t - Price_(t - period)| / Σ |Price_i - Price_(i-1)|
//!    ```
//!    - Measures the smoothness of price movement over `period`.
//!    - **Range:** `0` (choppy market) to `1` (strong trend).
//!
//! 2. **Compute the Smoothing Constant (SC):**
//!    ```text
//!    SC = (ER × (fast_SC - slow_SC) + slow_SC)²
//!    ```
//!    - Dynamically adjusts sensitivity based on `ER`.
//!    - `fast_SC = 2 / (fast + 1)`, `slow_SC = 2 / (slow + 1)`.
//!
//! 3. **Calculate KAMA using an EMA-like formula:**
//!    ```text
//!    KAMA_t = KAMA_(t-1) + SC × (Price_t - KAMA_(t-1))
//!    ```
//!
//! ## Advantages of KAMA
//! - **Adapts to market conditions**: Slows down during noise, accelerates in trends.
//! - **Reduces lag compared to SMA & EMA**.
//!
//! ## Performance Optimizations
//! - **Rolling sum for absolute price changes (sum_roc)** reduces redundant calculations.
//! - **Single-pass computation** makes it significantly faster than naive implementations.
//! - **Efficient memory usage** with minimal allocations and updates in-place.
//!
//! ## Example Usage
//! ```rust
//! use tarq::*;
//! use tarq::indicators::kama::Kama;
//!
//! let price_data = vec![
//!     5.29, 12.66, 9.86, 8.16, 2.49, 2.49, 1.24, 11.58,
//!     8.19, 9.56, 0.76, 12.91, 11.15, 3.21, 2.82, 2.84,
//!     4.39, 7.21, 6.02, 4.22, 8.33, 2.28, 4.23, 5.18,
//!     6.33, 10.55, 3.05, 7.08, 8.08, 1.09, 2.68, 3.12
//! ];
//! let period = 10;
//! let fast = 2;
//! let slow = 30;
//!
//! let mut kama = Kama::new(&price_data, period, fast, slow).unwrap();
//! let kama_values = kama.calculate().unwrap();
//!
//! println!("KAMA Values: {:?}", kama_values);
//! ```
//!
//! ## Struct Definition

use crate::Indicator;

/// **Kaufman Adaptive Moving Average (KAMA) Indicator**
///
/// The KAMA indicator dynamically adjusts its smoothing factor based on market conditions, 
/// reducing lag in trends while filtering noise in sideways movements.
#[derive(Clone, Debug)]
pub struct Kama<'a> {
    /// Reference to the input price data.
    data: &'a [f64],
    /// The lookback period for calculating the Efficiency Ratio (ER).
    period: usize,
    /// The fastest smoothing factor.
    fast: f64,
    /// The slowest smoothing factor.
    slow: f64,
    /// Current index in the iteration process.
    index: usize,
    /// The previously computed KAMA value.
    prev_kama: f64,
    /// Rolling sum of absolute price changes.
    sum_roc: f64,
    /// Last trailing value used for ROC calculations.
    trailing_value: f64,
}

impl<'a> Kama<'a> {
    /// Creates a new instance of the Kaufman Adaptive Moving Average (KAMA).
    ///
    /// # Arguments
    /// - `data`: A reference to the input price data.
    /// - `period`: The lookback period.
    /// - `fast`: The fastest smoothing factor.
    /// - `slow`: The slowest smoothing factor.
    ///
    /// # Returns
    /// - `Ok(Kama)` if initialization is successful.
    /// - `Err(String)` if `period` is zero or greater than the data length.
    ///
    /// # Example
    /// ```rust
    /// use tarq::*;
    /// use tarq::indicators::kama::Kama;
    ///
    /// let price_data = vec![
    ///     5.29, 12.66, 9.86, 8.16, 2.49, 2.49, 1.24, 11.58,
    ///     8.19, 9.56, 0.76, 12.91, 11.15, 3.21, 2.82, 2.84,
    ///     4.39, 7.21, 6.02, 4.22, 8.33, 2.28, 4.23, 5.18,
    ///     6.33, 10.55, 3.05, 7.08, 8.08, 1.09, 2.68, 3.12
    /// ];
    /// let kama = Kama::new(&price_data, 3, 2, 30);
    ///
    /// assert!(kama.is_ok());
    /// ```
    pub fn new(data: &'a [f64], period: usize, fast: usize, slow: usize) -> Result<Self, String> {
        if period == 0 {
            return Err("Period must be greater than 0.".to_string());
        }
        if data.len() < period {
            return Err("Period cannot be greater than input data length.".to_string());
        }
        if data.len() <= 1 {
            return Err("Data length must be greater than 1.".to_string());
        }

        let fast_sc = 2.0 / (fast as f64 + 1.0);
        let slow_sc = 2.0 / (slow as f64 + 1.0);

        // Initialize sum of absolute price changes
        let sum_roc = data[1..period].iter()
            .zip(data[..period - 1].iter())
            .map(|(curr, prev)| (curr - prev).abs())
            .sum();

        let prev_kama = data[period - 1]; // Initialize KAMA with last value in period

        Ok(Self {
            data,
            period,
            fast: fast_sc,
            slow: slow_sc,
            index: period,
            prev_kama,
            sum_roc,
            trailing_value: data[0],
        })
    }

    /// Computes the Efficiency Ratio (ER) with a rolling sum update.
    fn calculate_er(&mut self, start_index: usize) -> f64 {
        let price_change = (self.data[start_index] - self.data[start_index - self.period]).abs();

        // Update rolling sum of absolute price changes
        self.sum_roc -= (self.data[start_index - self.period] - self.trailing_value).abs();
        self.sum_roc += (self.data[start_index] - self.data[start_index - 1]).abs();
        self.trailing_value = self.data[start_index - self.period];

        if self.sum_roc == 0.0 {
            return 0.0;
        }

        price_change / self.sum_roc
    }

    /// Computes the smoothing constant (SC).
    fn calculate_sc(&self, er: f64) -> f64 {
        let sc = er * (self.fast - self.slow) + self.slow;
        sc * sc // Square for increased sensitivity
    }
}

impl Iterator for Kama<'_> {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.data.len() {
            return None;
        }

        // Compute Efficiency Ratio (ER)
        let er = self.calculate_er(self.index);
        let sc = self.calculate_sc(er);

        // Compute KAMA using an EMA-like formula
        let kama = self.prev_kama + sc * (self.data[self.index] - self.prev_kama);

        self.prev_kama = kama;
        self.index += 1;

        Some(kama)
    }
}

impl<'a> Indicator<'a> for Kama<'a> {
    type Output = Vec<f64>;

    /// Computes the Kaufman Adaptive Moving Average (KAMA) for the given data.
    ///
    /// # Example
    /// ```rust
    /// use tarq::*;
    /// use tarq::indicators::kama::Kama;
    ///
    /// let price_data = vec![
    ///     5.29, 12.66, 9.86, 8.16, 2.49, 2.49, 1.24, 11.58,
    ///     8.19, 9.56, 0.76, 12.91, 11.15, 3.21, 2.82, 2.84,
    ///     4.39, 7.21, 6.02, 4.22, 8.33, 2.28, 4.23, 5.18,
    ///     6.33, 10.55, 3.05, 7.08, 8.08, 1.09, 2.68, 3.12
    /// ];
    /// 
    /// let mut kama = Kama::new(&price_data, 10, 2, 30).unwrap();
    /// let kama_values = kama.calculate().unwrap();
    /// println!("KAMA Values: {:?}", kama_values);
    /// ```
    fn calculate(&mut self) -> Result<Self::Output, String> {
        Ok(self.collect())
    }
}





#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kama_valid() {
        let price_data = vec![
            5.29, 12.66, 9.86, 8.16, 2.49, 2.49, 1.24, 11.58, 8.19, 9.56, 0.76, 12.91,
            11.15, 3.21, 2.82, 2.84, 4.39, 7.21, 6.02, 4.22, 8.33, 2.28, 4.23, 5.18,
            6.33, 10.55, 3.05, 7.08, 8.08, 1.09, 2.68, 3.12,
        ];
        let expected_results = vec![
            2.49, 1.9344444444444444, 2.071360412420171, 2.0975235497376548, 2.8852412878752443,
            2.8613907122058375, 3.5377668906133284, 3.5793859996805817, 3.57014511197127,
            3.5415427951382417, 3.531308523681771, 3.749871114695233, 3.967917085216985,
            4.216347445182803, 4.216464143785599, 4.728262228909424, 4.677442941689665,
            4.662400966403684, 4.667428374682707, 4.706820880808867, 4.852443206333075,
            4.836544841355116, 4.894450510796723, 4.978125620047757, 4.8265787936555915,
            4.647290259376047, 4.640328209291386,
        ];

        let period = 5;
        let fast = 2;
        let slow = 30;

        let mut kama = Kama::new(&price_data, period, fast, slow).unwrap();
        let kama_values = kama.calculate().unwrap();

        // Ensure the calculated length matches expected length
        assert_eq!(
            kama_values.len(),
            expected_results.len(),
            "Mismatch in output length"
        );

        // Compare each value within a tolerance range
        let tolerance = 1e-6;
        for (i, &value) in kama_values.iter().enumerate() {
            assert!(
                (value - expected_results[i]).abs() < tolerance,
                "Mismatch at index {}: expected {}, got {}",
                i,
                expected_results[i],
                value
            );
        }
    }

    #[test]
    fn test_kama_invalid_period() {
        let price_data = vec![5.29, 12.66, 9.86, 8.16, 2.49];

        let kama = Kama::new(&price_data, 0, 2, 30);
        assert!(kama.is_err(), "KAMA should return an error for period = 0");

        let kama = Kama::new(&price_data, 6, 2, 30);
        assert!(
            kama.is_err(),
            "KAMA should return an error when period > data length"
        );
    }

    #[test]
    fn test_kama_single_data_point() {
        let price_data = vec![10.0];

        let kama = Kama::new(&price_data, 1, 2, 30);
        assert!(
            kama.is_err(),
            "KAMA should return an error when data length is too short"
        );
    }

    #[test]
    fn test_kama_constant_values() {
        let price_data = vec![5.0; 20]; // All values are the same

        let mut kama = Kama::new(&price_data, 5, 2, 30).unwrap();
        let kama_values = kama.calculate().unwrap();

        for value in kama_values.iter() {
            assert!(
                (*value - 5.0).abs() < 1e-6,
                "KAMA should remain constant if all prices are the same"
            );
        }
    }
}