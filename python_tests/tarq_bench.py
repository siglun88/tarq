import timeit
import numpy as np
import talib
import tarq
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


### TARQ BENCHMARK FUNCTIONS ###
def tarq_sma():
    return tarq.sma(price_data, period)


def tarq_ema():
    return tarq.ema(price_data, period)

def tarq_wma():
    return tarq.wma(price_data, period)

def tarq_atr():
    return tarq.atr(high_data, low_data, price_data, period)

def tarq_vwma():
    return tarq.vwma(price_data, volume_data, period)

def tarq_dema():
    return tarq.dema(price_data, period)

def tarq_tema():
    return tarq.tema(price_data, period)

def tarq_kama():
    return tarq.kama(price_data, period)

def tarq_bbands_sma():
    return tarq.bbands(price_data, period, std_dev, "sma", None)

def tarq_bbands_ema():
    return tarq.bbands(price_data, period, std_dev, "ema", volume_data)

def tarq_bbands_vwma():
    return tarq.bbands(price_data, period, std_dev, "vwma", volume_data)

def tarq_bbpb():
    return tarq.bbpb(price_data, period, std_dev, "sma")

def tarq_stddev():
    return tarq.stddev(price_data, period)

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

for name, func_tarq, func_talib in [
    ("SMA", tarq_sma, talib_sma),
    ("EMA", tarq_ema, talib_ema),
    ("VWMA", tarq_vwma, talib_dummy),
    ("WMA", tarq_wma, talib_wma),
    ("ATR", tarq_atr, talib_atr),
    ("DEMA", tarq_dema, talib_dema),
    ("TEMA", tarq_tema, talib_tema),
    ("KAMA", tarq_kama, talib_kama),
    ("BBands (SMA)", tarq_bbands_sma, talib_bbands_sma),
    ("BBands (EMA)", tarq_bbands_ema, talib_bbands_ema),
    ("BBands (VWMA)", tarq_bbands_vwma, talib_dummy),
    ("BBands percent bandwidth", tarq_bbpb, talib_dummy),
    ("Std Deviation", tarq_stddev, talib_steddev),
]:
    # Execution Time
    tarq_time = timeit.timeit(func_tarq, number=num_runs) / num_runs
    talib_time = timeit.timeit(func_talib, number=num_runs) / num_runs

    # Memory Usage
    tarq_mem = measure_memory(func_tarq)
    talib_mem = measure_memory(func_talib)

    results[name] = {
        "Pytarq Time (s)": tarq_time,
        "TA-Lib Time (s)": talib_time,
        "Pytarq Mem (MiB)": tarq_mem,
        "TA-Lib Mem (MiB)": talib_mem,
    }

    print(f"{name}:")
    print(f"  Pytarq   → Time: {tarq_time:.6f}s, Memory: {tarq_mem:.3f} MiB")
    print(f"  TA-Lib   → Time: {talib_time:.6f}s, Memory: {talib_mem:.3f} MiB\n")



