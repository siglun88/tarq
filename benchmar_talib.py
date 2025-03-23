import time
import talib
import numpy as np
import tracemalloc

# Sample data
price_data = np.array([5.2, 12.6, 9.8, 8.1, 2.4, 2.5, 1.2, 11.5, 8.1, 9.5, 0.7, 12.9], dtype=np.float64)

def benchmark_function(func, *args):
    tracemalloc.start()  # Start memory tracking
    start_time = time.perf_counter()
    result = func(*args)
    end_time = time.perf_counter()
    current, peak = tracemalloc.get_traced_memory()
    tracemalloc.stop()
    return (end_time - start_time) * 1e6, peak  # Execution time in µs, peak memory usage

# Benchmark TA-Lib SMA
sma_time, sma_mem = benchmark_function(talib.SMA, price_data, 5)

# Benchmark TA-Lib EMA
ema_time, ema_mem = benchmark_function(talib.EMA, price_data, 5)

# Benchmark TA-Lib Bollinger Bands
def bbands():
    return talib.BBANDS(price_data, timeperiod=5, nbdevup=2, nbdevdn=2, matype=talib.MA_Type.SMA)
bbands_time, bbands_mem = benchmark_function(bbands)

# Print results
print(f"SMA TA-Lib: {sma_time:.2f} µs, Memory: {sma_mem / 1024:.2f} KB")
print(f"EMA TA-Lib: {ema_time:.2f} µs, Memory: {ema_mem / 1024:.2f} KB")
print(f"BBands TA-Lib: {bbands_time:.2f} µs, Memory: {bbands_mem / 1024:.2f} KB")
