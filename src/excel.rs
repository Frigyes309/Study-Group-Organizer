#![allow(unused_imports)]
use crate::config::{do_config, dormitory_config, gtb_config, senior_config};
use crate::log::{Error, Info};
use crate::types::GTBStudent;
//use calamine::{open_workbook_auto, DataType};
use office::{Excel, Error as OfficeError};

pub fn gtb_import(path: &str) -> Result<Vec<GTBStudent>, Error> {
    let mut workbook: Excel = match Excel::open(path) {
        Ok(wb) => wb,
        Err(_) => return Err(Error::CannotOpenExcelFile),
    };
    let worksheet: String = match workbook.sheet_names() {
        Ok(sheets) => sheets.get(0).unwrap().to_string(),
        Err(_) => return Err(Error::NoSheetsFound),
    };
    match workbook.worksheet_range(&worksheet) {
        Ok(0) => return Err(Error::NoSheetsFound),
        Ok(x) => {
            if x > 1 {
                Info::MultipleSheetsFound.print(vec![path.to_string(), workbook.sheet_names()?.get(0).to_string()])?;
            }
        },
        Err(_) => return Err(Error::DataConversionError),
    }
    let filename:String = match path.split("/").last()? {
        Ok(f) => f.to_string(),
        Err(_) => return Err(Error::MissingFilename),
    };
    let config = gtb_config();
    let mut students = Vec::new();
    if let Some(Ok(range)) = workbook.worksheet_range(workbook.sheet_names()?.get(0)) {
        for row in range.rows().skip(1) {
            let mut missing_data = false;
            let neptun = row[config.get(0)]  {
                //value.is_string() => value,
                DataType::is_empty(_) => {
                    missing_data = true;
                    "".to_string()
                }
                _ => return Err(Error::DataConversionError),
            };
            let room_senior = match row[config.get(1)] {
                DataType::is_string(value) => value,
                DataType::is_empty(_) => {
                    if missing_data {
                        return Err(Error::DataConversionError);
                    }
                    missing_data = true;
                    "".to_string()
                }
                _ => return Err(Error::DataConversionError),
            };
            let card_senior = match row[config.get(2)] {
                DataType::is_string(value) => value,
                DataType::is_empty(_) => {
                    if missing_data {
                        return Err(Error::DataConversionError);
                    }
                    missing_data = true;
                    "".to_string()
                }
                _ => return Err(Error::DataConversionError),
            };
            let color = match row[config.get(3)] {
                DataType::is_string(value) => value,
                DataType::is_empty(_) => {
                    if missing_data {
                        return Err(Error::DataConversionError);
                    }
                    missing_data = true;
                    "".to_string()
                }
                _ => return Err(Error::DataConversionError),
            };
            let student = GTBStudent::new(neptun, room_senior, card_senior, color);
            if missing_data {
                Info::FoundEmptyCell.print(vec![filename])?;
            } else {
                students.push(student);
            }
        }
    }
    Ok(students)
}