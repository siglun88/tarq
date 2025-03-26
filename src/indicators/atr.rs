use crate::Indicator;

#[derive(Clone, Debug)]
pub struct Atr<'a> {
    high: &'a [f64],
    low: &'a [f64],
    close: &'a [f64],
    period: usize,
    index: usize,
    previous_tr: f64,
    len: usize,
}

impl<'a> Atr<'a> {
    pub fn new(high: &'a [f64], low: &'a [f64], close: &'a [f64], period: usize) -> Result<Self, String> {
        if period == 0 {
            return Err("Period must be greater than zero.".to_string());
        }
        if high.len() < period || low.len() < period || close.len() < period {
            return Err("Insufficient data for ATR calculation.".to_string());
        }
        if high.len() != low.len() || low.len() != close.len() || high.len() != close.len() {
            return Err("All inputs must have the same length.".to_string());
        }

        Ok(Self {
            high,
            low,
            close,
            period,
            index: 0,
            previous_tr: 0.0,
            len: close.len(),
        })
    }

}

impl Iterator for Atr<'_> {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        // Stop if the index goes out of bounds
        if self.index + self.period >= self.len {
            return None;
        }

        if self.index == 0 {
            // calculate SMA of True range for the first period and save it to previous_tr. Return previous_tr
            let mut sum = 0.0;
            for i in 1..self.period + 1 {

                let tr1 = self.high[i] - self.low[i];
                let tr2 = (self.high[i] - self.close[i - 1]).abs();
                let tr3 = (self.low[i] - self.close[i - 1]).abs();
            
                sum += tr1.max(tr2).max(tr3);

            }
            self.previous_tr = sum / self.period as f64;
            self.index += 1;
            return Some(self.previous_tr);
        }

        // Calculate ATR for remaining periods using Wilders approach.

        let tr1 = self.high[self.index + self.period] - self.low[self.index + self.period];
        let tr2 = (self.high[self.index + self.period] - self.close[self.index + self.period - 1]).abs();
        let tr3 = (self.low[self.index + self.period] - self.close[self.index + self.period - 1]).abs();
    
        let tr = tr1.max(tr2).max(tr3);

        self.previous_tr *= (self.period - 1) as f64;
        self.previous_tr += tr;
        self.previous_tr /= self.period as f64;

        self.index += 1;

        Some(self.previous_tr)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.len.saturating_sub(self.period + self.index) + 1;
        (remaining, Some(remaining))
    }
}

impl<'a> Indicator<'a> for Atr<'a> {
    type Output = Vec<f64>;

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
    fn test_atr_basic() {
        let high = vec![
            6.10162623, 14.56635718, 11.35078849,  9.39745112,  2.88643542,
            2.88638959,  1.45035886, 13.32573468,  9.43211852, 11.0053379 ,
            0.89911054, 14.85282872, 12.83164102,  3.71344431,  3.26546744,
            3.28846857,  5.06806026,  8.30703722,  6.94069554,  4.87333537,
            9.5925205 ,  2.64503834,  4.88507483,  5.9760875 ,  7.29653297,
            12.14183433,  3.52938019,  8.15704543,  9.30558768,  1.27825578,
            3.93433522,  3.54399584
        ];
        let low = vec![
            5.06640197, 10.75711748,  7.92529914,  6.52404504,  1.81123384,
            2.20442298, -0.16004275, 10.64371026,  7.85812735,  8.51916387,
            0.59466181, 11.08229124, 10.55831803,  1.85120766,  2.12774896,
            1.75187076,  3.25125047,  6.75877653,  4.07778921,  2.64724764,
            6.44495201,  0.47982803,  2.99399004,  3.32843895,  6.06186425,
            10.07763256,  2.86406815,  6.36187237,  7.24151315,  0.47443684,
            1.00539873,  2.34216868
        ];
        let close = vec![
            5.29, 12.66, 9.86, 8.16, 2.49, 2.49, 1.24, 11.58,
            8.19, 9.56, 0.76, 12.91, 11.15, 3.21, 2.82, 2.84,
            4.39, 7.21, 6.02, 4.22, 8.33, 2.28, 4.23, 5.18,
            6.33, 10.55, 3.05, 7.08, 8.08, 1.09, 2.68, 3.12
        ];
        let period = 5;

        let mut atr = Atr::new(&high, &low, &close, period).unwrap();
        let result = atr.calculate().unwrap();

        // For the given data and period, you can calculate expected ATR values manually
        // Example expected values: these would depend on your calculations for TR and ATR.
        let expected = vec![
            4.875549154, 4.4304478732, 5.961505234560001, 5.5135787176480004, 4.973930554118401,
            5.772212081294721, 7.436335409035777, 6.419404721228622, 6.9952822449828975, 5.823769491986318,
            4.9663351555890545, 4.418680176471243, 4.318351585176995, 4.0811234261415965, 3.939449212913277,
            4.2260634703306215, 4.950885170264497, 4.481723102211598, 4.1149081917692785, 3.715233147415423,
            4.134553383932338, 4.84482907714587, 4.8972723477166955, 4.362935414173356, 5.011460963338685,
            4.594956068670948, 3.916330286936758
        ];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_atr_insufficient_data() {
        let high = vec![1.1, 1.2];
        let low = vec![1.0, 1.1];
        let close = vec![1.05, 1.15];
        let period = 3;

        let atr = Atr::new(&high, &low, &close, period);
        assert!(atr.is_err());
        assert_eq!(atr.err().unwrap(), "Insufficient data for ATR calculation.");
    }

    #[test]
    fn test_atr_zero_period() {
        let high = vec![1.1, 1.2, 1.3, 1.4];
        let low = vec![1.0, 1.1, 1.2, 1.3];
        let close = vec![1.05, 1.15, 1.25, 1.35];
        let period = 0;

        let atr = Atr::new(&high, &low, &close, period);
        assert!(atr.is_err());
        assert_eq!(atr.err().unwrap(), "Period must be greater than zero.");
    }
}
