use crate::Indicator;

#[derive(Clone, Debug)]
pub struct Sma<'a> {
    data: &'a [f64],
    period: usize,
    index: usize,
    sum: f64,
    len: usize,
    inv_period: f64,
}

impl<'a> Sma<'a> {

    pub fn new(data: &'a [f64], period: usize) -> Result<Self, String> {
        if period == 0 {
            return Err("Period must be greater than zero.".to_string());
        }
        if data.len() < period {
            return Err("Insufficient data for SMA calculation.".to_string());
        }

        assert!(period <= data.len());
        

        let sum = data.iter().take(period).sum();
        let inv_period = 1.0 / period as f64;


        Ok(Self {
            data,
            period,
            index: 0,
            sum,
            len: data.len(),
            inv_period,
        })
    }
}

impl Iterator for Sma<'_> {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {

        if self.index > self.len - self.period {
            return None;
        }

        let result = self.sum * self.inv_period;

        if let Some(&next_value) = self.data.get(self.index + self.period) {
            self.sum += next_value;
            self.sum -= self.data[self.index];
        }

        self.index += 1;
        Some(result)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.len.saturating_sub(self.period + self.index) + 1;
        (remaining, Some(remaining))
    }

}

impl<'a> Indicator<'a> for Sma<'a> {
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
    fn test_sma_valid() {
        // Given input data
        let input_data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        let expected_output = vec![2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];

        let mut sma = Sma::new(&input_data, 3).unwrap();
        let output = sma.calculate().unwrap();
        


        // Assert the output
        assert_eq!(&output, &expected_output, "SMA calculation is incorrect!");
    }

    #[test]
    fn test_sma_invalid_input() {
        // Given empty input data
        let input_data = vec![];


        // Ensure calculation fails due to invalid input
        assert!(Sma::new(&input_data, 3).is_err(), "SMA should return an error for empty input.");
    }

    #[test]
    fn test_sma_short_data() {
        // Given input data shorter than the period
        let input_data = vec![1.0, 2.0];

        // Ensure calculation fails due to insufficient data
        assert!(Sma::new(&input_data, 3).is_err(), "SMA should return an error when data is shorter than the period.");
    }
}




