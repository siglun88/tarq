//! # Bollinger Bands (BBands) Indicator
//!
//! The **Bollinger Bands (BBands)** indicator is a widely used technical analysis tool 
//! that measures market volatility and identifies potential overbought or oversold conditions.
//! It consists of three bands:
//! - **Upper Band**: Represents potential overbought levels.
//! - **Middle Band**: A moving average of the price (configurable).
//! - **Lower Band**: Represents potential oversold levels.
//!
//! ## Overview
//! This implementation of Bollinger Bands allows for dynamic selection of the moving average 
//! used for the middle band via the [`MovingAverage`] enum. 
//! Supported moving averages include SMA, EMA, WMA, VWMA, DEMA, and TEMA.
//!
//! ## Calculation
//! - The **middle band** is computed using the selected moving average.
//! - The **standard deviation** of the price data over the period is used to calculate the 
//!   **upper and lower bands**:
//!   - `Upper Band = Middle Band + (Standard Deviation × Multiplier)`
//!   - `Lower Band = Middle Band - (Standard Deviation × Multiplier)`
//!
//! ## Performance Considerations
//! - Uses a **rolling sum of squares** for an optimized standard deviation calculation.
//! - Uses an **iterator-based approach**, making it efficient for streaming data analysis.
//!
//! ## Example Usage
//! ```rust
//! use tarq::*;
//! use tarq::indicators::{sma::Sma, bbands::BBands};
//! use tarq::enums::MovingAverage;
//!
//! let period: usize = 3;
//! let price_data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0];
//! let ma_type = MovingAverage::SMA(Sma::new(&price_data, period).unwrap());
//! let mut bbands = BBands::new(&price_data, period, 2.0, ma_type).unwrap();
//!
//! let (upper_band, middle_band, lower_band) = bbands.calculate().unwrap();
//!
//! println!("Upper Band: {:?}", upper_band);
//! println!("Middle Band: {:?}", middle_band);
//! println!("Lower Band: {:?}", lower_band);
//! ```
//!
//! ## Struct Definition

use crate::Indicator;
use crate::enums::MovingAverage;
use crate::indicators::sma::Sma;


/// **The Bollinger Bands Indicator**
///
/// The `BBands` struct calculates Bollinger Bands using a configurable moving average 
/// type for the middle band. It efficiently computes standard deviation using a rolling 
/// sum of squares approach.
#[derive(Clone, Debug)]
pub struct BBands<'a> {
    /// Reference to the input price data.
    data: &'a [f64],
    /// The lookback period for computing the Bollinger Bands.
    period: usize,
    /// The standard deviation multiplier.
    std_dev: f64,
    /// The type of moving average used for the middle band.
    ma_type: MovingAverage<'a>,
    /// Current index in the iteration process.
    index: usize,
    /// Rolling sum of squared values used for standard deviation calculation.
    rolling_sq_sum: f64,
    /// Simple Moving Average (SMA) instance used for initial mean calculation.
    sma: Sma<'a>,
}

impl<'a> BBands<'a> {
    /// Creates a new instance of the Bollinger Bands (BBands) indicator.
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
    /// use tarq::indicators::{sma::Sma, bbands::BBands};
    /// use tarq::enums::MovingAverage;
    ///
    /// let period: usize = 3;
    /// let price_data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    /// let ma_type = MovingAverage::SMA(Sma::new(&price_data, period).unwrap());
    /// let bbands = BBands::new(&price_data, period, 2.0, ma_type);
    ///
    /// assert!(bbands.is_ok());
    /// ```
    pub fn new(data: &'a [f64], period: usize, std_dev: f64, ma_type: MovingAverage<'a>) -> Result<Self, String> {
        if period == 0 {
            return Err("Period must be set to a number greater than 0".to_string());
        }
        if data.len() < period {
            return Err("Period cannot be greater than input data length.".to_string());
        }

        let sma = Sma::new(data, period)?;

        Ok(Self {
            data,
            period,
            std_dev,
            ma_type,
            index: 0,
            rolling_sq_sum: 0.0,
            sma,
        })
    }
}

impl Iterator for BBands<'_> {
    type Item = (f64, f64, f64); // (upper_band, middle_band, lower_band)

    fn next(&mut self) -> Option<Self::Item> {
        if self.index + self.period > self.data.len() {
            return None;
        }

        if self.index == 0 {
            // Initialize rolling sum of squares on the first iteration
            self.rolling_sq_sum = self.data[..self.period].iter().map(|&x| x * x).sum::<f64>();
        } else if self.index + self.period <= self.data.len() {
            // Efficient rolling update
            let outgoing_index = self.index - 1;
            let incoming_index = self.index + self.period - 1;

            self.rolling_sq_sum += self.data[incoming_index] * self.data[incoming_index]
                - self.data[outgoing_index] * self.data[outgoing_index];
        }

        let mean = self.sma.next().unwrap();

        // Compute the moving average using the selected ma_type for the middle band
        let middle_band = match &mut self.ma_type {
            MovingAverage::SMA(_) => mean, // Use stored SMA instance
            MovingAverage::EMA(ema) => ema.next().unwrap(),
            MovingAverage::WMA(wma) => wma.next().unwrap(),
            MovingAverage::DEMA(dema) => dema.next().unwrap(),
            MovingAverage::TEMA(tema) => tema.next().unwrap(),
            MovingAverage::VWMA(vwma) => vwma.next().unwrap(),
            MovingAverage::KAMA(kama) => kama.next().unwrap()
        };

        // Rolling variance calculation
        let variance = (self.rolling_sq_sum / self.period as f64) - (mean * mean);
        let std_dev = variance.sqrt();

        // Compute Bollinger Bands
        let upper = middle_band + self.std_dev * std_dev;
        let lower = middle_band - self.std_dev * std_dev;

        // Slide the window forward
        self.index += 1;

        Some((upper, middle_band, lower))
    }
}

impl<'a> Indicator<'a> for BBands<'a> {
    type Output = (Vec<f64>, Vec<f64>, Vec<f64>); // (upper_band, middle_band, lower_band)

    /// Computes the Bollinger Bands for the given data.
    ///
    /// Returns three vectors representing the upper, middle, and lower bands.
    ///
    /// # Example
    /// ```rust
    /// use tarq::*;
    /// use tarq::indicators::{sma::Sma, bbands::BBands};
    /// use tarq::enums::MovingAverage;
    ///
    /// let period: usize = 3;
    /// let price_data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    /// let ma_type = MovingAverage::SMA(Sma::new(&price_data, period).unwrap());
    /// let mut bbands = BBands::new(&price_data, period, 2.0, ma_type).unwrap();
    ///
    /// let (upper, middle, lower) = bbands.calculate().unwrap();
    /// println!("Upper Band: {:?}", upper);
    /// println!("Middle Band: {:?}", middle);
    /// println!("Lower Band: {:?}", lower);
    /// ```
    fn calculate(&mut self) -> Result<Self::Output, String> {
        let len = self.data.len() - self.period + 1;
    
        let mut upper_band = Vec::with_capacity(len);
        let mut middle_band = Vec::with_capacity(len);
        let mut lower_band = Vec::with_capacity(len);
    
        self.by_ref().for_each(|(upper, middle, lower)| {
            upper_band.push(upper);
            middle_band.push(middle);
            lower_band.push(lower);
        });
    
        Ok((upper_band, middle_band, lower_band))
    }
    
    
}





#[cfg(test)]
mod tests {
    use super::*;
    use crate::enums::MovingAverage;
    use crate::indicators::sma::Sma;
    use crate::indicators::ema::Ema;
    use crate::indicators::vwma::Vwma;


    #[test]
    fn test_bollinger_bands_valid() {
        let price_data = vec![
            5.29411352124624, 12.669143122046927, 9.869522455185985, 8.162828597722068,
            2.4970385976631873, 2.496729860303394, 1.243470235752953, 11.58705466591917,
            8.194272150313072, 9.563328995789382, 0.7634815269862714, 12.914846107673528,
            11.155265802245399, 3.217940616681935, 2.827359580250888, 2.8475777261239528,
            4.394300709882083, 7.216882324892644, 6.028896238619082, 4.227732994534937,
            8.331717052446457, 2.2855214163461355, 4.239451501250793, 5.189431594159254,
            6.337695797978061, 10.550252305830575, 3.055824411627005, 7.0822008116942285,
            8.082906481434144, 1.0945652828159709
        ];

        let expected_upper_band = vec![
            14.7680417, 15.24756354, 11.79663943, 13.18944967, 13.20510606, 14.7025897,
            15.14309588, 17.09194853, 16.89045466, 16.93160388, 15.95183105, 15.55381522,
            11.25848222, 7.41957931, 8.1415686, 7.98323375, 9.21855717, 9.91957083,
            9.09202468, 8.81194058, 9.32541666, 11.23278794, 11.02544128, 11.36922189,
            11.90247524, 12.84001576
        ];

        let expected_middle_band = vec![
            7.69852926, 7.13905253, 4.85391795, 5.19742439, 5.2037131, 6.61697118,
            6.27032151, 8.60459669, 8.51823892, 7.52297261, 6.17577873, 6.59259797,
            4.88848889, 4.10081219, 4.66300332, 4.943078, 6.03990586, 5.61815001,
            5.02266384, 4.85477091, 5.27676347, 5.72047052, 5.87453112, 6.44308098,
            7.02177596, 5.97314986
        ];

        let expected_lower_band = vec![
            0.62901682, -0.96945848, -2.08880353, -2.79460088, -2.79767986, -1.46864734,
            -2.60245285, 0.11724485, 0.14602318, -1.88565866, -3.60027359, -2.36861928,
            -1.48150444, 0.78204507, 1.18443803, 1.90292225, 2.86125456, 1.31672918,
            0.953303, 0.89760124, 1.22811028, 0.2081531, 0.72362096, 1.51694007,
            2.14107668, -0.89371604
        ];

        let mut bb = BBands::new(&price_data, 5, 2.0, MovingAverage::SMA(Sma::new(&price_data, 5).unwrap())).unwrap();

        let (upper_band, middle_band, lower_band) = bb.calculate().unwrap();

        // Assertions
        assert_eq!(upper_band.len(), expected_upper_band.len(), "Upper band length mismatch");
        assert_eq!(middle_band.len(), expected_middle_band.len(), "Middle band length mismatch");
        assert_eq!(lower_band.len(), expected_lower_band.len(), "Lower band length mismatch");

        for (i, &value) in upper_band.iter().enumerate() {
            assert!((value - expected_upper_band[i]).abs() < 1e-6, "Upper band value mismatch at index {}", i);
        }

        for (i, &value) in middle_band.iter().enumerate() {
            assert!((value - expected_middle_band[i]).abs() < 1e-6, "Middle band value mismatch at index {}", i);
        }

        for (i, &value) in lower_band.iter().enumerate() {
            assert!((value - expected_lower_band[i]).abs() < 1e-6, "Lower band value mismatch at index {}", i);
        }
    }

    #[test]
    fn test_bollinger_bands_valid_ema_expected_values() {
        let price_data = vec![
            5.29411352124624, 12.669143122046927, 9.869522455185985, 8.162828597722068,
            2.4970385976631873, 2.496729860303394, 1.243470235752953, 11.58705466591917,
            8.194272150313072, 9.563328995789382, 0.7634815269862714, 12.914846107673528,
            11.155265802245399, 3.217940616681935, 2.827359580250888, 2.8475777261239528,
            4.394300709882083, 7.216882324892644, 6.028896238619082, 4.227732994534937,
            8.331717052446457, 2.2855214163461355, 4.239451501250793, 5.189431594159254,
            6.337695797978061, 10.550252305830575, 3.055824411627005, 7.0822008116942285,
            8.082906481434144, 1.0945652828159709,
        ];
    
        // Placeholder expected values for EMA-based Bollinger Bands.
        // Replace these with the actual values when available.
        let expected_upper_band = vec![
            14.768041697365513, 14.073107137129, 11.333608973578087, 14.781635161240914, 15.259223601731236,
            16.111948610085772, 14.478154934951007, 16.529220918825985, 17.45188372952203, 16.534390136284628,
            15.46901142515431, 13.705715895304959, 10.997759328844392, 8.809571893047384, 9.148733881341972,
            8.229512480636702, 9.415461476100898, 9.221134743882953, 8.762320620524445, 8.81562005319302,
            9.400185376987789, 12.596756313743306, 10.89247756095654, 11.114586114623073, 11.700631576053016,
            11.778342522525119,
        ];
        let expected_middle_band = vec![
            7.698529258772881, 5.964596125949718, 4.39088749588413, 6.7896098858958105, 7.257830640701564,
            8.026330092397503, 5.605380570593759, 8.041869082953681, 9.079667989384253, 7.125758865150147,
            5.692959103517061, 4.744498644386025, 4.627765999551378, 5.490804774665134, 5.6701685959831165,
            5.189356728833723, 6.236810170037968, 4.919713918807357, 4.692959779621836, 4.858450384467642,
            5.351532188971115, 7.084438894590935, 5.741567400269625, 6.188445204077826, 6.819932296529932,
            4.911476625291945,
        ];
        let expected_lower_band = vec![
            0.6290168201802491, -2.143914885229563, -2.5518339818098257, -1.2024153894492935, -0.7435623203281088,
            -0.05928842529076661, -3.267393793763489, -0.44548275291862005, 0.7074522492464759, -2.2828724059843344,
            -4.083093218120189, -4.2167186065329085, -1.7422273297416373, 2.1720376562828836, 2.191603310624261,
            2.1492009770307443, 3.0581588639750383, 0.6182930937317614, 0.6235989387192253, 0.901280715742264,
            1.30287900095444, 1.572121475438565, 0.5906572395827103, 1.262304293532579, 1.9392330170068472,
            -1.955389271941229,
        ];
    
        // Create EMA instance.
        let ema = Ema::new(&price_data, 5).unwrap();
        let mut bb = BBands::new(&price_data, 5, 2.0, MovingAverage::EMA(ema)).unwrap();
    
        let (upper_band, middle_band, lower_band) = bb.calculate().unwrap();
    
        // Print the resulting upper band for debugging purposes
        println!("EMA: Resulting upper band: {:?}", upper_band);
        println!("EMA: Expected upper band: {:?}", expected_upper_band);
    
        // Check that lengths match.
        assert_eq!(upper_band.len(), expected_upper_band.len(), "EMA: Upper band length mismatch");
        assert_eq!(middle_band.len(), expected_middle_band.len(), "EMA: Middle band length mismatch");
        assert_eq!(lower_band.len(), expected_lower_band.len(), "EMA: Lower band length mismatch");
    
        // Compare each band value against the expected placeholder values.
        for (i, &value) in upper_band.iter().enumerate() {
            assert!((value - expected_upper_band[i]).abs() < 1e-6, "EMA: Upper band value mismatch at index {}", i);
        }
        for (i, &value) in middle_band.iter().enumerate() {
            assert!((value - expected_middle_band[i]).abs() < 1e-6, "EMA: Middle band value mismatch at index {}", i);
        }
        for (i, &value) in lower_band.iter().enumerate() {
            assert!((value - expected_lower_band[i]).abs() < 1e-6, "EMA: Lower band value mismatch at index {}", i);
        }
    }

    #[test]
    fn test_bollinger_bands_valid_vwma_expected_values() {
        let price_data = vec![
            5.29411352124624, 12.669143122046927, 9.869522455185985, 8.162828597722068,
            2.4970385976631873, 2.496729860303394, 1.243470235752953, 11.58705466591917,
            8.194272150313072, 9.563328995789382, 0.7634815269862714, 12.914846107673528,
            11.155265802245399, 3.217940616681935, 2.827359580250888, 2.8475777261239528,
            4.394300709882083, 7.216882324892644, 6.028896238619082, 4.227732994534937,
            8.331717052446457, 2.2855214163461355, 4.239451501250793, 5.189431594159254,
            6.337695797978061, 10.550252305830575, 3.055824411627005, 7.0822008116942285,
            8.082906481434144, 1.0945652828159709
        ];

        let volume_data = vec![
            137.41506534163324, 82.56857195127418, 106.85847335913176, 140.35779817937657, 157.0233000165165,
            96.60203235342382, 91.07613689753174, 73.68294404898585, 137.44995543023836, 156.97394872931952,
            188.46552229665835, 82.12694392430107, 12.035494687686104, 65.82881410149886, 91.63893514289319,
            42.10079912956817, 107.33236283643453, 155.65874329777574, 88.99637196017784, 32.94671562659742,
            89.6089198575782, 157.49408516683113, 49.88676866266892, 141.40584720639913, 160.16763490226415,
            49.050590186061434, 138.54686101167783, 44.70239139442573, 66.36109747195077, 113.58610109018836
            ];

        let vwma = Vwma::new(&price_data, &volume_data, 5).unwrap();
        let mut bb = BBands::new(&price_data, 5, 2.0, MovingAverage::VWMA(vwma)).unwrap();

        // Placeholder expected values for VWMA-based Bollinger Bands.
        // Replace these with the actual values when available.
        let expected_upper_band = vec![
            14.063831038592632, 14.758573671179285, 11.92127074769396, 12.906667085345106, 12.906817451029674,
            14.987034857688272, 14.645048254357249, 15.823841535872303, 15.245604470137774, 15.446714271134487,
            13.888192751637252, 15.040694040918936, 10.122228989293017, 8.04429002838225, 8.647213155358854,
            8.638635171802974, 9.537309556062928, 9.839320825075593, 8.828797970902606, 8.605915438725372,
            9.172513518016675, 10.598033199152368, 10.53303016068691, 10.557386890545242, 11.002695029523078,
            11.51985318723317
            ];
        let expected_middle_band = vec![
            6.9943186 , 6.65006266, 4.97854927, 4.91464181, 4.90542449,
            6.90141634, 5.77227389, 7.3364897 , 6.87338873, 6.038083  ,
            4.11214043, 6.07947679, 3.75223566, 4.72552291, 5.16864787,
            5.59847942, 6.35865825, 5.5379    , 4.75943713, 4.64874577,
            5.12386033, 5.08571578, 5.38212   , 5.63124598, 6.12199575,
            4.65298729
            ];
        let expected_lower_band = vec![
            -0.0751938385926314, -1.4584483511792854, -1.9641722076939594, -3.0773834653451067, -3.095968471029675,
            -1.1842021776882712, -3.1005004743572497, -1.1508621358723037, -1.4988270101377736, -3.3705482711344867,
            -5.663911891637252, -2.8817404609189348, -2.6177576692930167, 1.4067557916177504, 1.690082584641147,
            2.558323668197027, 3.180006943937073, 1.2364791749244066, 0.6900762890973944, 0.6915761012746264,
            1.0752071419833262, -0.42660163915236904, 0.23120983931308903, 0.7051050694547571, 1.2412964704769225,
            -2.213878607233169
            ];

        let (upper_band, middle_band, lower_band) = bb.calculate().unwrap();

        // Check that lengths match.
        assert_eq!(upper_band.len(), expected_upper_band.len(), "VWMA: Upper band length mismatch");
        assert_eq!(middle_band.len(), expected_middle_band.len(), "VWMA: Middle band length mismatch");
        assert_eq!(lower_band.len(), expected_lower_band.len(), "VWMA: Lower band length mismatch");

        // Compare each band value against the expected placeholder values.
        for (i, &value) in upper_band.iter().enumerate() {
            assert!((value - expected_upper_band[i]).abs() < 1e-6, "Expected {}, got {}, at index {}", expected_upper_band[i], value, i);
        }
        for (i, &value) in middle_band.iter().enumerate() {
            assert!((value - expected_middle_band[i]).abs() < 1e-6, "VWMA: Middle band value mismatch at index {}", i);
        }
        for (i, &value) in lower_band.iter().enumerate() {
            assert!((value - expected_lower_band[i]).abs() < 1e-6, "VWMA: Lower band value mismatch at index {}", i);
        }
    }

    

    #[test]
    fn test_bollinger_bands_invalid_input() {
        let price_data = vec![];

        let result = BBands::new(&price_data, 5, 2.0, MovingAverage::SMA(Sma::new(&price_data, 5).unwrap_or_else(|_| Sma::new(&[0.0], 1).unwrap())));

        assert!(
            result.is_err(),
            "Bollinger Bands should return an error for empty input."
        );
    }

    #[test]
    fn test_bollinger_bands_short_data() {
        let price_data = vec![1.0, 2.0, 3.0, 4.0];

        let result = BBands::new(&price_data, 5, 2.0, MovingAverage::SMA(Sma::new(&price_data, 5).unwrap_or_else(|_| Sma::new(&[0.0], 1).unwrap())));

        assert!(
            result.is_err(),
            "Bollinger Bands should return an error when data is shorter than the period."
        );
    }


}
