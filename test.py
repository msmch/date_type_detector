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

# Use the results to automatically convert date columns
for col, is_date in date_columns.items():
    if is_date:
        df[col] = pd.to_datetime(df[col])

print(df.dtypes)