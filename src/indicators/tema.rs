use crate::Indicator;

/// **The Triple Exponential Moving Average (TEMA) Indicator**
///
/// TEMA is a smoothed version of the Exponential Moving Average (EMA), reducing lag 
/// significantly by applying EMA three times and using the formula:
///
/// `TEMA = (3 * EMA1) - (3 * EMA2) + EMA3`
#[derive(Clone, Debug)]
pub struct Tema<'a> {
    /// Reference to the input price data.
    data: &'a [f64],
    /// Current index in the iteration process.
    index: usize,
    /// The lookback period for calculating the TEMA.
    period: usize,
    /// First EMA applied to the price data.
    prev_ema1: f64,
    /// Second EMA applied to `ema1`.
    prev_ema2: f64,
    /// Third EMA applied to `ema2`.
    prev_ema3: f64,
    /// The smoothing factor used in the EMA formula.
    smoothing: f64,
    /// Lenght of iterator when initialized.
    len: usize,
}

impl<'a> Tema<'a> {
    /// Creates a new instance of the Triple Exponential Moving Average (TEMA).
    ///
    /// # Arguments
    /// - `data`: A reference to the input price data.
    /// - `period`: The lookback period for calculating the TEMA.
    ///
    /// # Errors
    /// Returns an error if:
    /// - The `period` is zero.
    /// - The `data` length is shorter than the `period`.
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
            prev_ema3: 0.0,
            smoothing: 2.0 / (period as f64 + 1.0),
            len: data.len(),
        })
    }
}

impl Iterator for Tema<'_> {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index + (3 * self.period - 3) >= self.data.len() {
            return None;
        }

        if self.index == 0 {
            // Step 1: Compute the first EMA1 as SMA of the first `period` values
            let sum: f64 = self.data[..self.period].iter().sum();
            self.prev_ema1 = sum / self.period as f64;

            // Step 2: Collect `period` EMA1 values
            let mut ema1_values = Vec::with_capacity(self.period);
            ema1_values.push(self.prev_ema1);

            for data in self.data.iter().take(3 * self.period - 2).skip(self.period) {
                let ema1 = (data - self.prev_ema1) * self.smoothing + self.prev_ema1;
                self.prev_ema1 = ema1;
                ema1_values.push(ema1);
            }

            // Step 3: Compute the first EMA2 as SMA of `period` EMA1 values
            let sum_ema1: f64 = ema1_values.iter().take(self.period).sum::<f64>();
            self.prev_ema2 = sum_ema1 / self.period as f64;

            // Step 4: Collect `period` EMA2 values
            let mut ema2_values = Vec::with_capacity(self.period);
            ema2_values.push(self.prev_ema2);

            for ema1 in ema1_values.iter().take(2 * self.period - 1).skip(self.period) {
                let ema2 = (ema1 - self.prev_ema2) * self.smoothing + self.prev_ema2;
                self.prev_ema2 = ema2;
                ema2_values.push(ema2);
            }

            // Step 5: Compute the first EMA3 as SMA of `period` EMA2 values
            let sum_ema2: f64 = ema2_values.iter().sum();
            self.prev_ema3 = sum_ema2 / self.period as f64;

            // Step 6: Compute the first TEMA value
            let tema = (3.0 * self.prev_ema1) - (3.0 * self.prev_ema2) + self.prev_ema3;
            self.index += 1;
            return Some(tema);
        }

        // Offset index to start after initialization phase
        let price_index = self.index + (3 * self.period - 3);
        let price = self.data[price_index];

        // Compute EMA1
        self.prev_ema1 = (price - self.prev_ema1) * self.smoothing + self.prev_ema1;

        // Compute EMA2
        self.prev_ema2 = (self.prev_ema1 - self.prev_ema2) * self.smoothing + self.prev_ema2;

        // Compute EMA3
        self.prev_ema3 = (self.prev_ema2 - self.prev_ema3) * self.smoothing + self.prev_ema3;

        // Compute TEMA
        let tema = (3.0 * self.prev_ema1) - (3.0 * self.prev_ema2) + self.prev_ema3;

        self.index += 1;
        Some(tema)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.len.saturating_sub(self.index + (3 * self.period - 3)) + 1;
        (remaining, Some(remaining))
    }
}

impl<'a> Indicator<'a> for Tema<'a> {
    type Output = Vec<f64>;

    /// Computes the Triple Exponential Moving Average (TEMA) for the given data.
    ///
    /// Returns a vector containing the TEMA values over the dataset.
    fn calculate(&mut self) -> Result<Self::Output, String> {
        let mut result = Vec::with_capacity(self.len);
        result.extend(self);

        Ok(result)
    }
}





#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tema() {
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
            11.044847329440401, 5.959385243406953, 3.409189609783623, 2.437615762600011,
            3.280303846018417, 5.782164174061058, 6.089667420333415, 4.925525512453958,
            7.2898167761801, 4.046471040727867, 3.979436185895974, 4.680631223306519,
            5.832327483108697, 9.291686982864922, 5.438437099665834, 6.493466763223496,
            7.665779897220894, 3.2218801274821836
        ];
        
        let mut tema = Tema::new(&data, 5).expect("Failed to create Tema");
        let result: Vec<f64> = tema.calculate().expect("Calculation failed");

        println!("Result: {:?}", result);
        
        assert_eq!(result.len(), expected.len());
        for (r, e) in result.iter().zip(expected.iter()) {
            assert!((r - e).abs() < 1e-6, "Expected {}, got {}", e, r);
        }
    }
}
