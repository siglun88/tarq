use crate::{Indicator, IndicatorInput, IndicatorOutput};

/// **Input structure for [Indicator Name]**
pub struct [IndicatorName]Input<'a> {
    pub data: &'a [f64], // Modify this based on required inputs
    pub period: usize,
}

impl<'a> IndicatorInput for [IndicatorName]Input<'a> {
    fn validate(&self) -> bool {
        !self.data.is_empty() && self.period > 0 && self.data.len() >= self.period
    }
}

/// **Output structure for [Indicator Name]**
pub struct [IndicatorName]Output {
    pub result: Vec<f64>, // Modify this based on the indicator's output
}

impl IndicatorOutput for [IndicatorName]Output {
    type IndicatorReturnType = Vec<f64>;

    fn result(&self) -> &Self::IndicatorReturnType {
        &self.result
    }
}

/// **The [Indicator Name] Indicator**
pub struct [IndicatorName];

impl [IndicatorName] {
    pub fn new() -> Self {
        Self
    }

    /// **Core calculation logic for [Indicator Name]**
    fn calculate_[indicator_short_name](&self, data: &[f64], period: usize) -> Result<Vec<f64>, String> {
        if data.len() < period {
            return Err("Period can not be greater than input data".to_string());
        }

        let mut result = vec![0.0; data.len() - period + 1]; // Preallocate buffer

        // Modify logic below based on indicator formula
        result[0] = data.iter().take(period).sum::<f64>() / period as f64; // First value

        for i in period..data.len() {
            result[i - period + 1] = result[i - period] + (data[i] - data[i - period]) / period as f64;
        }

        Ok(result)
    }
}

/// **Implements the Indicator Trait for [Indicator Name]**
impl<'a> Indicator<[IndicatorName]Input<'a>, [IndicatorName]Output> for [IndicatorName] {
    fn calculate(&self, input: &[IndicatorName]Input) -> Result<[IndicatorName]Output, String> {
        if !input.validate() {
            return Err("Invalid input: Data must not be empty and period must be valid.".to_string());
        }

        let result = self.calculate_[indicator_short_name](input.data, input.period)?;
        Ok([IndicatorName]Output { result })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_[indicator_short_name]_valid() {
        let input_data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        let expected_output = vec![2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]; // Modify this for actual output

        let input = [IndicatorName]Input {
            data: &input_data,
            period: 3,
        };

        let indicator = [IndicatorName]::new();
        let output = indicator.calculate(&input).unwrap();

        assert_eq!(output.result(), &expected_output, "[Indicator Name] calculation is incorrect!");
    }

    #[test]
    fn test_[indicator_short_name]_invalid_input() {
        let input_data = vec![];

        let input = [IndicatorName]Input {
            data: &input_data,
            period: 3,
        };

        let indicator = [IndicatorName]::new();
        assert!(indicator.calculate(&input).is_err(), "[Indicator Name] should return an error for empty input.");
    }

    #[test]
    fn test_[indicator_short_name]_short_data() {
        let input_data = vec![1.0, 2.0];

        let input = [IndicatorName]Input {
            data: &input_data,
            period: 3,
        };

        let indicator = [IndicatorName]::new();
        assert!(indicator.calculate(&input).is_err(), "[Indicator Name] should return an error when data is shorter than the period.");
    }
}
