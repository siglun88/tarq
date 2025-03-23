import timeit
import numpy as np
import talib
import tarq as pytarq
from memory_profiler import memory_usage

# Generate large test dataset (5,000,000 data points)
N = 5_000_000
price_data = np.random.uniform(50, 200, N)  # Random prices between $50-$200
high_data = price_data + np.random.uniform(2, 15, N)
low_data = price_data - np.random.uniform(2, 15, N)
volume_data = np.random.uniform(1000, 5000, N) # Random volume between 1000-5000
period = 50
std_dev = 2.0
num_runs = 3  # Number of repetitions for averaging


# Measure memory usage
def measure_memory(func, *args):
    mem_usage = memory_usage((func, args), interval=0.001)
    return max(mem_usage) - min(mem_usage)  # Peak memory usage


### PYTARQ BENCHMARK FUNCTIONS ###
def pytarq_sma():
    return pytarq.sma(price_data, period)


def pytarq_ema():
    return pytarq.ema(price_data, period)

def pytarq_wma():
    return pytarq.wma(price_data, period)

def pytarq_atr():
    return pytarq.atr(high_data, low_data, price_data, period)

def pytarq_vwma():
    return pytarq.vwma(price_data, volume_data, period)

def pytarq_dema():
    return pytarq.dema(price_data, period)

def pytarq_tema():
    return pytarq.tema(price_data, period)

def pytarq_kama():
    return pytarq.kama(price_data, period)

def pytarq_bbands_sma():
    return pytarq.bbands(price_data, period, std_dev, "sma", None)

def pytarq_bbands_ema():
    return pytarq.bbands(price_data, period, std_dev, "ema", volume_data)

def pytarq_bbands_vwma():
    return pytarq.bbands(price_data, period, std_dev, "vwma", volume_data)

def pytarq_bbpb():
    return pytarq.bbpb(price_data, period, std_dev, "sma")

def pytarq_stddev():
    return pytarq.stddev(price_data, period)

### TA-LIB BENCHMARK FUNCTIONS ###
def talib_sma():
    return talib.SMA(price_data, timeperiod=period)

def talib_ema():
    return talib.EMA(price_data, timeperiod=period)

def talib_wma():
    return talib.WMA(price_data, timeperiod=period)

def talib_atr():
    return talib.ATR(high_data, low_data, price_data, timeperiod=period)

def talib_dema():
    return talib.DEMA(price_data, timeperiod=period)

def talib_tema():
    return talib.TEMA(price_data, timeperiod=period)

def talib_kama():
    return talib.KAMA(price_data, timeperiod=period)


def talib_bbands_sma():
    return talib.BBANDS(price_data, timeperiod=period, nbdevup=std_dev, nbdevdn=std_dev, matype=0)  # matype=0 → SMA

def talib_bbands_ema():
    return talib.BBANDS(price_data, timeperiod=period, nbdevup=std_dev, nbdevdn=std_dev, matype=talib.MA_Type.EMA)

def talib_steddev():
    return talib.STDDEV(price_data, timeperiod=period)

def talib_dummy():
    pass


### RUN BENCHMARKS ###
results = {}

for name, func_pytarq, func_talib in [
    ("SMA", pytarq_sma, talib_sma),
    ("EMA", pytarq_ema, talib_ema),
    ("VWMA", pytarq_vwma, talib_dummy),
    ("WMA", pytarq_wma, talib_wma),
    ("ATR", pytarq_atr, talib_atr),
    ("DEMA", pytarq_dema, talib_dema),
    ("TEMA", pytarq_tema, talib_tema),
    ("KAMA", pytarq_kama, talib_kama),
    ("BBands (SMA)", pytarq_bbands_sma, talib_bbands_sma),
    ("BBands (EMA)", pytarq_bbands_ema, talib_bbands_ema),
    ("BBands (VWMA)", pytarq_bbands_vwma, talib_dummy),
    ("BBands percent bandwidth", pytarq_bbpb, talib_dummy),
    ("Std Deviation", pytarq_stddev, talib_steddev),
]:
    # Execution Time
    pytarq_time = timeit.timeit(func_pytarq, number=num_runs) / num_runs
    talib_time = timeit.timeit(func_talib, number=num_runs) / num_runs

    # Memory Usage
    pytarq_mem = measure_memory(func_pytarq)
    talib_mem = measure_memory(func_talib)

    results[name] = {
        "Pytarq Time (s)": pytarq_time,
        "TA-Lib Time (s)": talib_time,
        "Pytarq Mem (MiB)": pytarq_mem,
        "TA-Lib Mem (MiB)": talib_mem,
    }

    print(f"{name}:")
    print(f"  Pytarq   → Time: {pytarq_time:.6f}s, Memory: {pytarq_mem:.3f} MiB")
    print(f"  TA-Lib   → Time: {talib_time:.6f}s, Memory: {talib_mem:.3f} MiB\n")



