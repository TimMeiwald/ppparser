# Create flamegraph 
PERF=/usr/lib/linux-tools/6.8.0-39-generic/perf cargo flamegraph --root -- -s ../json_samples/canada.json

# Use perf inbuilt report tool
/usr/lib/linux-tools/6.8.0-39-generic/perf report


# Record Perf Data 
 /usr/lib/linux-tools/6.8.0-39-generic/perf record -g -F 10000 ./target/release/parser -s ../json_samples/canada.json 100
# Convert to Firefox Profiler format https://profiler.firefox.com/
/usr/lib/linux-tools/6.8.0-39-generic/perf script -i perf.data >
ffperf.data