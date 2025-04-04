# Date Type Detector

A Rust-powered Python package that analyzes pandas DataFrames to detect columns containing date or timestamp strings.

## What it does

Tired of constantly fixing data types in your DataFrames? Or perhaps you'd like to automatically generate table definitions right after parsing CSV files you've never seen before? You've just found the tool that will make your life easier.

`date_type_detector` rapidly scans object-type columns in pandas DataFrames to determine if they contain date or timestamp data stored as strings. It considers a wide range of common date formats and can help automate the process of converting string date columns to proper datetime types.

Key features:
- Fast performance with Rust implementation
- Handles null values gracefully
- Recognizes multiple common date/datetime formats
- Returns a simple dictionary showing which columns likely contain date strings

## Installation

### Prerequisites
- Python 3.6+
- Rust compiler (latest stable version)
- [Maturin](https://github.com/PyO3/maturin) (`pip install maturin`)

### Building from source

1. Clone this repository
```bash
git clone https://github.com/msmch/date_type_detector.git
cd date_type_detector
```

2. Setup virtual python environment
```bash
sh setup_venv.sh
```

3. Build the package
```bash
maturin build --release
```

Or check build_package.sh file where you can specify command for your OS and run
```bash
sh build_package.sh
``` 

4. Install the built wheel
```bash
pip install target/wheels/date_type_detector-*.whl
```

## Usage

Basic usage example:

```python
import pandas as pd
from date_type_detector import analyze_dataframe

# Create a sample DataFrame
df = pd.DataFrame({
    'order_date': ['2023-01-01', '2023-02-15', '2023-03-30'],
    'customer_id': ['C001', 'C002', 'C003'],
    'timestamp': ['2023-01-01 08:15:30', '2023-02-15 14:22:45', '2023-03-30 19:05:12'],
    'mixed_data': ['2023-01-01', 'not a date', '2023-03-30']
})

# Analyze the DataFrame
date_columns = analyze_dataframe(df)
print(date_columns)
# Output: {'order_date': True, 'customer_id': False, 'timestamp': True, 'mixed_data': False}

# Use the results to automatically convert date columns
for col, is_date in date_columns.items():
    if is_date:
        df[col] = pd.to_datetime(df[col])

print(df.dtypes)
# Output:
# order_date     datetime64[ns]
# customer_id           object
# timestamp      datetime64[ns]
# mixed_data            object
# dtype: object
```

## Customizing Date Formats

The package includes common date and datetime formats, but you can modify them before building:

1. Edit `date_formats.txt` to add or remove date formats
2. Edit `datetime_formats.txt` to add or remove datetime formats

Each format should be on a separate line using [strftime format codes](https://docs.python.org/3/library/datetime.html#strftime-and-strptime-format-codes).

## Performance

This package is implemented in Rust for high performance, making it suitable for large DataFrames where Python-based solutions might be slower.

## License

[MIT License](LICENSE)