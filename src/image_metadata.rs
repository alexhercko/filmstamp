use anyhow::{Context, Result, anyhow};

/// Extracts timestamp from raw EXIF data bytes extracted from an image.
/// Currently, it only supports the DateTimeOriginal field.
///
/// If extracting the timestamp or parsing it fails, an error is returned.
pub fn extract_timestamp_from_exif(raw_exif: Vec<u8>) -> Result<chrono::NaiveDateTime> {
    let exifreader = exif::Reader::new();
    let exif = exifreader
        .read_raw(raw_exif)
        .map_err(|e| anyhow!("Failed to read EXIF data from raw bytes: {}", e))?;

    let datetime = match exif.get_field(exif::Tag::DateTimeOriginal, exif::In::PRIMARY) {
        Some(timestamp) => extract_timestamp_from_exif_field(timestamp)?,
        None => {
            return Err(anyhow!(
                "No EXIF timestamp (DateTimeOriginal field) found in the image."
            ));
        }
    };

    let naive_datetime = exif_datetime_to_naive_datetime(&datetime)?;
    Ok(naive_datetime)
}

fn extract_timestamp_from_exif_field(field: &exif::Field) -> Result<exif::DateTime> {
    match &field.value {
        exif::Value::Ascii(values) if !values.is_empty() => exif::DateTime::from_ascii(&values[0])
            .with_context(|| "Failed to parse EXIF timestamp (DateTimeOriginal field)."),
        _ => Err(anyhow!(
            "Invalid EXIF format for timestamp (DateTimeOriginal field)."
        )),
    }
}

fn exif_datetime_to_naive_datetime(
    exif_datetime: &exif::DateTime,
) -> Result<chrono::NaiveDateTime> {
    chrono::NaiveDate::from_ymd_opt(
        exif_datetime.year.into(),
        exif_datetime.month.into(),
        exif_datetime.day.into(),
    )
    .and_then(|date| {
        date.and_hms_opt(
            exif_datetime.hour.into(),
            exif_datetime.minute.into(),
            exif_datetime.second.into(),
        )
    })
    .ok_or_else(|| anyhow!("Failed to convert EXIF DateTime to NaiveDateTime."))
}

#[cfg(test)]
mod tests {
    use chrono::{Datelike, Timelike};

    use super::*;

    mod extract_timestamp_from_exif_tests {
        use super::*;

        #[test]
        fn test_extract_timestamp_from_exif_empty_data() {
            let empty_data: Vec<u8> = vec![];

            let result = extract_timestamp_from_exif(empty_data);
            assert!(result.is_err());
        }

        #[test]
        fn test_extract_timestamp_from_exif_invalid_data() {
            let invalid_data: Vec<u8> = b"not a valid image".to_vec();

            let result = extract_timestamp_from_exif(invalid_data);
            assert!(result.is_err());
        }
    }

    mod extract_timestamp_from_exif_field_tests {
        use super::*;

        #[test]
        fn test_extract_timestamp_from_exif_field_valid() {
            let exif_field = exif::Field {
                tag: exif::Tag::DateTimeOriginal,
                ifd_num: exif::In::PRIMARY,
                value: exif::Value::Ascii(vec![b"2025:01:02 03:04:05".to_vec()]),
            };

            let result = extract_timestamp_from_exif_field(&exif_field);

            assert!(result.is_ok());

            let datetime = result.unwrap();

            assert_eq!(datetime.year, 2025);
            assert_eq!(datetime.month, 1);
            assert_eq!(datetime.day, 2);
            assert_eq!(datetime.hour, 3);
            assert_eq!(datetime.minute, 4);
            assert_eq!(datetime.second, 5);
        }

        #[test]
        fn test_extract_timestamp_from_exif_field_invalid_value() {
            let exif_field = exif::Field {
                tag: exif::Tag::DateTimeOriginal,
                ifd_num: exif::In::PRIMARY,
                value: exif::Value::Short(vec![1]),
            };

            let result = extract_timestamp_from_exif_field(&exif_field);

            assert!(result.is_err());
            assert!(
                result
                    .unwrap_err()
                    .to_string()
                    .contains("Invalid EXIF format for timestamp (DateTimeOriginal field).")
            )
        }

        #[test]
        fn test_extract_timestamp_from_exif_field_empty_date() {
            let exif_field = exif::Field {
                tag: exif::Tag::DateTimeOriginal,
                ifd_num: exif::In::PRIMARY,
                value: exif::Value::Ascii(vec![]),
            };

            let result = extract_timestamp_from_exif_field(&exif_field);

            assert!(result.is_err());
            assert!(
                result
                    .unwrap_err()
                    .to_string()
                    .contains("Invalid EXIF format for timestamp (DateTimeOriginal field).")
            )
        }

        #[test]
        fn test_extract_timestamp_from_exif_field_invalid_date() {
            let exif_field = exif::Field {
                tag: exif::Tag::DateTimeOriginal,
                ifd_num: exif::In::PRIMARY,
                value: exif::Value::Ascii(vec![b"this is not a date".to_vec()]),
            };

            let result = extract_timestamp_from_exif_field(&exif_field);

            assert!(result.is_err());
            assert!(
                result
                    .unwrap_err()
                    .to_string()
                    .contains("Failed to parse EXIF timestamp (DateTimeOriginal field).")
            )
        }
    }

    mod exif_datetime_to_naive_datetime_tests {
        use super::*;

        #[test]
        fn test_exif_datetime_to_naive_datetime_valid() {
            let exif_dt = exif::DateTime {
                year: 2025,
                month: 1,
                day: 2,
                hour: 3,
                minute: 4,
                second: 5,
                nanosecond: None,
                offset: None,
            };

            let result = exif_datetime_to_naive_datetime(&exif_dt);

            assert!(result.is_ok());

            let result = result.unwrap();

            assert_eq!(result.year(), 2025);
            assert_eq!(result.month(), 1);
            assert_eq!(result.day(), 2);
            assert_eq!(result.hour(), 3);
            assert_eq!(result.minute(), 4);
            assert_eq!(result.second(), 5);
        }

        #[test]
        fn test_exif_datetime_to_naive_datetime_invalid_date() {
            let exif_dt = exif::DateTime {
                year: 2025,
                month: 13,
                day: 42,
                hour: 1,
                minute: 1,
                second: 1,
                nanosecond: None,
                offset: None,
            };

            let result = exif_datetime_to_naive_datetime(&exif_dt);

            assert!(result.is_err());
            assert!(
                result
                    .unwrap_err()
                    .to_string()
                    .contains("Failed to convert EXIF DateTime to NaiveDateTime.")
            )
        }

        #[test]
        fn test_exif_datetime_to_naive_datetime_invalid_time() {
            let exif_dt = exif::DateTime {
                year: 2025,
                month: 1,
                day: 1,
                hour: 25,
                minute: 61,
                second: 61,
                nanosecond: None,
                offset: None,
            };

            let result = exif_datetime_to_naive_datetime(&exif_dt);

            assert!(result.is_err());
            assert!(
                result
                    .unwrap_err()
                    .to_string()
                    .contains("Failed to convert EXIF DateTime to NaiveDateTime.")
            )
        }
    }
}
