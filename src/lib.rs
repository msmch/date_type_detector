use pyo3::prelude::*;
use pyo3::types::PyDict;
use chrono::{NaiveDate, NaiveDateTime};
use std::collections::HashMap;
use once_cell::sync::Lazy;

// --- Load formats from files at compile time ---
const DATE_FORMATS_STR: &str = include_str!("patterns/date_formats.txt");
const DATETIME_FORMATS_STR: &str = include_str!("patterns/datetime_formats.txt");

// Lazily parse the strings into vectors of string slices (&str)
// This parsing happens only once when the vectors are first accessed.
static DATE_FORMATS: Lazy<Vec<&'static str>> = Lazy::new(|| {
    DATE_FORMATS_STR
        .lines()
        .map(|line| line.trim()) 
        .filter(|line| !line.is_empty())
        .collect()
});

static DATETIME_FORMATS: Lazy<Vec<&'static str>> = Lazy::new(|| {
    DATETIME_FORMATS_STR
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect()
});

fn is_potential_datetime_str(s: &str) -> bool {
    if s.len() < 6 {
        return false;
    }

    for fmt in DATETIME_FORMATS.iter() {
        if NaiveDateTime::parse_from_str(s, fmt).is_ok() {
            return true;
        }
    }

    for fmt in DATE_FORMATS.iter() {
        if NaiveDate::parse_from_str(s, fmt).is_ok() {
            return true;
        }
    }

    // Fallback: Try chrono's default parser
    if s.parse::<NaiveDateTime>().is_ok() || s.parse::<NaiveDate>().is_ok() {
        return true;
    }

    false
}

/// Analyzes object columns in a Pandas DataFrame for date/timestamp strings.
#[pyfunction]
fn analyze_dataframe(py: Python<'_>, df: &PyAny) -> PyResult<PyObject> {
    let mut results: HashMap<String, bool> = HashMap::new();

    let columns = df.getattr("columns")?;
    let columns_list = columns.getattr("tolist")?.call0()?;
    let columns: Vec<String> = columns_list.extract()?;
    
    // Get dtypes series
    let dtypes = df.getattr("dtypes")?;
    for col_name in columns {
        let dtype_obj = dtypes.get_item(&col_name)?;
        let dtype_str = dtype_obj.str()?.to_str()?;

        // Only check object columns and focus on non-NA values
        if dtype_str == "object" {
            let series = df.get_item(&col_name)?;
            let dropna = series.getattr("dropna")?.call0()?;
            let values = dropna.getattr("values")?;
            let iter = values.iter()?;
            let mut non_null_count = 0;
            let mut parsable_count = 0;
            let mut contains_non_date_string = false;

            // Iterate and check if item is a string that can be parsed as date/datetime
            for item_result in iter {
                let item = item_result?;
                non_null_count += 1;
                if let Ok(item_str) = item.extract::<&str>() {
                    let trimmed_string = item_str.trim();         
                    if !trimmed_string.is_empty() {
                        if is_potential_datetime_str(trimmed_string) {
                            parsable_count += 1;
                        } else {
                            contains_non_date_string = true;
                            break; // Found a non-date string, stop checking this column
                        }
                    }
                } else {
                    // Not a string
                    contains_non_date_string = true;
                    break;
                }
            }

            // Column is likely datetime if no bad strings found and all values were parsable
            let is_likely_datetime_col = !contains_non_date_string && non_null_count > 0 && parsable_count == non_null_count;
            results.insert(col_name, is_likely_datetime_col);
        }
    }
    let py_dict = PyDict::new(py);
    for (key, value) in results {
        py_dict.set_item(key, value)?;
    }

    Ok(py_dict.into())
}

#[pymodule]
fn date_type_detector(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(analyze_dataframe, m)?)?;
    Ok(())
}