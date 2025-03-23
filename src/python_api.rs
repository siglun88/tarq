use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyString;
use numpy::{PyArray1, PyReadonlyArray1};
use crate::indicators::{
    bbands::BBands,
    sma::Sma,
    ema::Ema,
    vwma::Vwma,
    atr::Atr,
    stddev::StdDev,
    bbpb::Bbpb,
    wma::Wma,
    dema::Dema,
    tema::Tema,
    kama::Kama
};
use crate::Indicator;



/// Python module definition
#[pymodule]
fn tarq(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sma, m)?)?;
    m.add_function(wrap_pyfunction!(ema, m)?)?;
    m.add_function(wrap_pyfunction!(vwma, m)?)?;
    m.add_function(wrap_pyfunction!(wma, m)?)?;
    m.add_function(wrap_pyfunction!(atr, m)?)?;
    m.add_function(wrap_pyfunction!(dema, m)?)?;
    m.add_function(wrap_pyfunction!(tema, m)?)?;
    m.add_function(wrap_pyfunction!(kama, m)?)?;
    m.add_function(wrap_pyfunction!(bbands, m)?)?;
    m.add_function(wrap_pyfunction!(bbpb, m)?)?;
    m.add_function(wrap_pyfunction!(stddev, m)?)?;
    Ok(())
}

fn prepend_vec_in_place(data: &mut Vec<f64>, prepend_count: usize, value: f64) {
    data.splice(0..0, std::iter::repeat(value).take(prepend_count));
}



/// Optimized SMA function with zero-copy NumPy
#[pyfunction]
fn sma<'py>(py: Python<'py>, data: PyReadonlyArray1<'py, f64>, period: usize) -> PyResult<Bound<'py, PyArray1<f64>>> {
    let data_slice = data.as_slice()?;
    let mut sma = Sma::new(data_slice, period).map_err(|e| PyValueError::new_err(e.to_string()))?;

    let mut result = sma.calculate().map_err(|e| PyValueError::new_err(e.to_string()))?;

    let prepend_count = data_slice.len().saturating_sub(result.len());
    prepend_vec_in_place(&mut result, prepend_count, f64::NAN);

    Ok(PyArray1::from_vec(py, result))
}


/// Optimized EMA function
#[pyfunction]
fn ema<'py>(py: Python<'py>, data: PyReadonlyArray1<f64>, period: usize) -> PyResult<Bound<'py, PyArray1<f64>>> {
    let data_slice = data.as_slice()?;
    let mut ema = Ema::new(data_slice, period).unwrap();
    let mut result = ema.calculate().map_err(PyErr::new::<pyo3::exceptions::PyValueError, _>)?;

    let prepend_count = data_slice.len() - result.len();
    prepend_vec_in_place(&mut result, prepend_count, f64::NAN);

    Ok(PyArray1::from_vec(py, result))
}

/// Optimized WMA function
#[pyfunction]
fn wma<'py>(py: Python<'py>, data: PyReadonlyArray1<f64>, period: usize) -> PyResult<Bound<'py, PyArray1<f64>>> {
    let data_slice = data.as_slice()?;
    let mut wma = Wma::new(data_slice, period).unwrap();
    let mut result = wma.calculate().map_err(PyErr::new::<pyo3::exceptions::PyValueError, _>)?;

    let prepend_count = data_slice.len() - result.len();
    prepend_vec_in_place(&mut result, prepend_count, f64::NAN);

    Ok(PyArray1::from_vec(py, result))
}

#[pyfunction]
fn dema<'py>(py: Python<'py>, data: PyReadonlyArray1<f64>, period: usize) -> PyResult<Bound<'py, PyArray1<f64>>> {
    let data_slice = data.as_slice()?;
    let mut dema = Dema::new(data_slice, period).unwrap();
    let mut result = dema.calculate().map_err(PyErr::new::<pyo3::exceptions::PyValueError, _>)?;

    let prepend_count = data_slice.len() - result.len();
    prepend_vec_in_place(&mut result, prepend_count, f64::NAN);

    Ok(PyArray1::from_vec(py, result))
}

#[pyfunction]
fn tema<'py>(py: Python<'py>, data: PyReadonlyArray1<f64>, period: usize) -> PyResult<Bound<'py, PyArray1<f64>>> {
    let data_slice = data.as_slice()?;
    let mut tema = Tema::new(data_slice, period).unwrap();
    let mut result = tema.calculate().map_err(PyErr::new::<pyo3::exceptions::PyValueError, _>)?;

    let prepend_count = data_slice.len() - result.len();
    prepend_vec_in_place(&mut result, prepend_count, f64::NAN);

    Ok(PyArray1::from_vec(py, result))
}

#[pyfunction]
#[pyo3(signature = (data, period = 10, fast = 2, slow = 30))]
fn kama<'py>(py: Python<'py>, data: PyReadonlyArray1<f64>, period: usize, fast: usize, slow: usize) -> PyResult<Bound<'py, PyArray1<f64>>> {
    let data_slice = data.as_slice()?;
    let mut kama = Kama::new(data_slice, period, fast, slow).unwrap();
    let mut result = kama.calculate().map_err(PyErr::new::<pyo3::exceptions::PyValueError, _>)?;

    let prepend_count = data_slice.len() - result.len();
    prepend_vec_in_place(&mut result, prepend_count, f64::NAN);

    Ok(PyArray1::from_vec(py, result))
}


// Standard deviation
#[pyfunction]
#[pyo3(signature = (data, period, ddof = 0))]
fn stddev<'py>(py: Python<'py>, data: PyReadonlyArray1<f64>, period: usize, ddof: usize) -> PyResult<Bound<'py, PyArray1<f64>>> {
    let data_slice = data.as_slice()?;
    let mut stddev = StdDev::new(data_slice, period, ddof).unwrap();
    let mut result = stddev.calculate().map_err(PyErr::new::<pyo3::exceptions::PyValueError, _>)?;

    let prepend_count = data_slice.len() - result.len();
    prepend_vec_in_place(&mut result, prepend_count, f64::NAN);

    Ok(PyArray1::from_vec(py, result))
}

/// Optimized VWMA function
#[pyfunction]
fn vwma<'py>(
    py: Python<'py>,
    data: PyReadonlyArray1<f64>,
    volume: PyReadonlyArray1<f64>,
    period: usize
) -> PyResult<Bound<'py, PyArray1<f64>>> {
    let data_slice = data.as_slice()?;
    let volume_slice = volume.as_slice()?;
    let mut vwma = Vwma::new(data_slice, volume_slice, period).unwrap();
    let mut result = vwma.calculate().map_err(PyErr::new::<pyo3::exceptions::PyValueError, _>)?;

    let prepend_count = data_slice.len() - result.len();
    prepend_vec_in_place(&mut result, prepend_count, f64::NAN);

    Ok(PyArray1::from_vec(py, result))
}

#[pyfunction]
fn atr<'py>(
    py: Python<'py>,
    high: PyReadonlyArray1<f64>,
    low: PyReadonlyArray1<f64>,
    close: PyReadonlyArray1<f64>,
    period: Option<usize>
) -> PyResult<Bound<'py, PyArray1<f64>>> {
    let high_slice = high.as_slice()?;
    let low_slice = low.as_slice()?;
    let close_slice = close.as_slice()?;
    let mut atr = Atr::new(high_slice, low_slice, close_slice, period.unwrap_or(14)).map_err(PyErr::new::<pyo3::exceptions::PyValueError, _>)?;
    let mut result = atr.calculate().map_err(PyErr::new::<pyo3::exceptions::PyValueError, _>)?;

    let prepend_count = high_slice.len() - result.len();
    prepend_vec_in_place(&mut result, prepend_count, f64::NAN);

    Ok(PyArray1::from_vec(py, result))
}

/// Optimized Bollinger Bands function
#[pyfunction]
#[pyo3(signature = (data, period, std_dev, ma_type, volume = None))]
fn bbands<'py>(
    py: Python<'py>,
    data: PyReadonlyArray1<f64>,
    period: usize,
    std_dev: f64,
    ma_type: Bound<'py, PyString>,
    volume: Option<PyReadonlyArray1<f64>>,
) -> PyResult<Bound<'py, PyAny>> {
    let data_slice = data.as_slice()?;

    let volume_data = match volume.as_ref() {
        Some(v) => Some(
            v.as_slice()?
        ),
        None => None,
    };

    let ma_type_str = ma_type.to_str()?;



    let ma_type_enum = match ma_type_str {
        "sma" => crate::enums::MovingAverage::SMA(Sma::new(data_slice, period).unwrap()),
        "ema" => crate::enums::MovingAverage::EMA(Ema::new(data_slice, period).unwrap()),
        "vwma" => crate::enums::MovingAverage::VWMA(Vwma::new(data_slice, volume_data.unwrap(), period).unwrap()),
        _ => return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid moving average type")),
    };

    let mut bb = BBands::new(data_slice, period, std_dev, ma_type_enum).unwrap();
    let (mut upper, mut middle, mut lower) = bb.calculate().map_err(PyErr::new::<pyo3::exceptions::PyValueError, _>)?;

    let prepend_count = data_slice.len() - upper.len();
    prepend_vec_in_place(&mut upper, prepend_count, f64::NAN);
    prepend_vec_in_place(&mut middle, prepend_count, f64::NAN);
    prepend_vec_in_place(&mut lower, prepend_count, f64::NAN);

    let upper_py = PyArray1::from_vec(py, upper);
    let middle_py = PyArray1::from_vec(py, middle);
    let lower_py = PyArray1::from_vec(py, lower);

    // Use Python's collections.namedtuple to define the return type
    let collections = PyModule::import(py, "collections")?;
    let namedtuple = collections.getattr("namedtuple")?;
    let namedtuple_type = namedtuple.call1(("BollingerBands", "upper middle lower"))?;
    
    // Return the properly constructed named tuple
    let namedtuple_instance = namedtuple_type.call1((upper_py, middle_py, lower_py))?;
    Ok(namedtuple_instance)
    


}

/// Optimized Bollinger Bands %b function
#[pyfunction]
#[pyo3(signature = (data, period, std_dev, ma_type, volume = None))]
fn bbpb<'py>(
    py: Python<'py>,
    data: PyReadonlyArray1<f64>,
    period: usize,
    std_dev: f64,
    ma_type: &Bound<'py, PyString>,
    volume: Option<PyReadonlyArray1<f64>>,
) -> PyResult<Bound<'py, PyArray1<f64>>> {
    let data_slice = data.as_slice()?;
    let ma_type_str = ma_type.to_str()?;
    let volume_data = match volume.as_ref() {
        Some(v) => Some(
            v.as_slice()?
        ),
        None => None,
    };

    let ma_type_enum = match ma_type_str {
        "sma" => crate::enums::MovingAverage::SMA(Sma::new(data_slice, period).unwrap()),
        "ema" => crate::enums::MovingAverage::EMA(Ema::new(data_slice, period).unwrap()),
        "vwma" => crate::enums::MovingAverage::VWMA(Vwma::new(data_slice, volume_data.unwrap(), period).unwrap()),
        _ => return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid moving average type")),
    };

    let mut bbpb = Bbpb::new(data_slice, period, std_dev, ma_type_enum).unwrap();
    let mut result = bbpb.calculate().map_err(PyErr::new::<pyo3::exceptions::PyValueError, _>)?;
    
    let prepend_count = data_slice.len() - result.len();
    prepend_vec_in_place(&mut result, prepend_count, f64::NAN);
    
    Ok(PyArray1::from_vec(py, result))
}

