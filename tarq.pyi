from typing import Optional, NamedTuple
import numpy as np

def sma(data: np.ndarray, period: int) -> np.ndarray:
    """
    Calculate the Simple Moving Average (SMA).
    
    :param data: Input price data as a NumPy array.
    :param period: Period for SMA calculation.
    :return: NumPy array containing SMA values.
    """
    pass

def ema(data: np.ndarray, period: int) -> np.ndarray:
    """
    Calculate the Exponential Moving Average (EMA).
    
    :param data: Input price data as a NumPy array.
    :param period: Period for EMA calculation.
    :return: NumPy array containing EMA values.
    """
    pass

def wma(data: np.ndarray, period: int) -> np.ndarray:
    """
    Calculate the Weighted Moving Average (WMA).
    
    :param data: Input price data as a NumPy array.
    :param period: Period for WMA calculation.
    :return: NumPy array containing WMA values.
    """
    pass

def dema(data: np.ndarray, period: int) -> np.ndarray:
    """
    Calculate the Double Exponential Moving Average (DEMA).
    
    :param data: Input price data as a NumPy array.
    :param period: Period for DEMA calculation.
    :return: NumPy array containing DEMA values.
    """
    pass

def tema(data: np.ndarray, period: int) -> np.ndarray:
    """
    Calculate the Triple Exponential Moving Average (TEMA).
    
    :param data: Input price data as a NumPy array.
    :param period: Period for TEMA calculation.
    :return: NumPy array containing TEMA values.
    """
    pass


def kama(data: np.ndarray, period: Optional[int] = 10, fast: Optional[int] = 2, slow: Optional[int] = 30) -> np.ndarray:
    """
    Calculate the Kaufman Adaptive Moving Average (KAMA).
    
    :param data: Input price data as a NumPy array.
    :param period: Period for KAMA calculation (optional).
    :param fast: The fast smoothing factor (optional).
    :param slow: The slow smoothing factor (optional).
    :return: NumPy array containing KAMA values.
    """
    pass


def vwma(data: np.ndarray, volume: np.ndarray, period: int) -> np.ndarray:
    """
    Calculate the Volume Weighted Moving Average (VWMA).
    
    :param data: Input price data as a NumPy array.
    :param volume: Volume data as a NumPy array.
    :param period: Period for VWMA calculation.
    :return: NumPy array containing VWMA values.
    """
    pass

def atr(high: np.ndarray, low: np.ndarray, close: np.ndarray, period: Optional[int] = 14) -> np.ndarray:
    """
    Calculate the ATR using Wilders smoothing (RMA).
    
    :param high: Input high price data as a NumPy array.
    :param low: Input low price data as a NumPy array.
    :param close: Input close price data as a NumPy array.
    :param period: Period for ATR calculation.
    :return: NumPy array containing ATR values.
    """
    pass

def stddev(data: np.ndarray, period: int, ddof: Optional[int] = None) -> np.ndarray:
    """
    Calculate the Standard Deviation.
    
    :param data: Input price data as a NumPy array.
    :param period: Period for standard deviation calculation.
    :param ddof: Degrees of freedom (optional).
    :return: NumPy array containing standard deviation values.
    """
    pass

class BollingerBands(NamedTuple):
    upper: np.ndarray
    middle: np.ndarray
    lower: np.ndarray

def bbands(data: np.ndarray, period: int, std_dev: float, ma_type: str, volume: Optional[np.ndarray] = None) -> BollingerBands:
    """
    Calculate Bollinger Bands.
    
    :param data: Input price data as a NumPy array.
    :param period: Period for Bollinger Bands calculation.
    :param std_dev: Standard deviation multiplier.
    :param ma_type: Moving average type ("sma", "ema", "vwma").
    :param volume: Volume data (required for VWMA, optional otherwise).
    :return: BollingerBands named tuple containing (upper, middle, lower) as NumPy arrays.
    """
    pass

def bbpb(data: np.ndarray, period: int, std_dev: float, ma_type: str, volume: Optional[np.ndarray] = None) -> np.ndarray:
    """
    Calculate Bollinger Bands %b (Percent Bandwidth).
    
    :param data: Input price data as a NumPy array.
    :param period: Period for Bollinger Bands %b calculation.
    :param std_dev: Standard deviation multiplier.
    :param ma_type: Moving average type ("sma", "ema").
    :return: NumPy array containing Bollinger Bands %b values.
    """
    pass
