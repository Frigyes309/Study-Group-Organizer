#![allow(unused_imports)]
use crate::config::{do_config, dormitory_config, gtb_config, senior_config};
use crate::log::{Error, Info};
use crate::types::{GTBStudent, Class, Gender, convert::DataConverter};
//use calamine::{open_workbook_auto, DataType};
use office::{DataType, Error as OfficeError, Excel};
use std::array;
use std::ops::Deref;
use std::path::Path;
use std::sync::Arc;

pub fn gtb_import(path: &str) -> Result<Arc<Vec<GTBStudent>>, Error> {
    let mut workbook: Excel = match Excel::open(path) {
        Ok(wb) => wb,
        Err(_) => return Err(Error::CannotOpenExcelFile),
    };
    let worksheet: String = match workbook.sheet_names() {
        Ok(sheets) => {
            let sheet = sheets.get(0).expect("No problem, because at least 1 sheet is present here");
            if sheets.len() > 1 {
                Info::MultipleSheetsFound.print(vec![path.to_string(), sheet.clone()])?;
            }
            sheet.clone()
        }
        Err(_) => return Err(Error::NoSheetsFound),
    };
    let filename: String;
    if let Some(f) = Path::new(path).file_name() {
        filename = String::from(f.to_str().expect("Filename contains invalid UTF-8"));
    } else {
        return Err(Error::MissingFilename)
    }
    let config = gtb_config();
    let mut students: Vec<GTBStudent> = Vec::new();
    let range = workbook.worksheet_range(&worksheet)?;
    for row in range.rows().skip(1) {
        let neptun = match get_verified_cell(row, config[0], 2)?.to_string() {
                Some(x) => x,
                None => return Err(Error::DataConversionError),
        };
        let room_senior = match get_verified_cell(row, config[1], 2)?.to_string() {
            Some(x) => x,
            None => return Err(Error::DataConversionError),
        };
        let card_senior = match get_verified_cell(row, config[2], 2)?.to_string() {
            Some(x) => x,
            None => return Err(Error::DataConversionError),
        };
        let color = match get_verified_cell(row, config[3], 2)?.to_string() {
            Some(x) => x,
            None => return Err(Error::DataConversionError),
        };
        if neptun.is_empty() || room_senior.is_empty() || card_senior.is_empty() || color.is_empty() {
            if !neptun.is_empty() || !room_senior.is_empty() || !card_senior.is_empty() || !color.is_empty() {
                return Err(Error::MissingData)
            }
            Info::FoundEmptyCell.print(vec![filename.clone()])?;
        }
        let student = GTBStudent::new(neptun, room_senior, card_senior, color);
        students.push(student);
    }
    let arc = Arc::new(students);
    Ok(arc)
}

fn get_verified_cell(row: &[DataType], element: usize, cast_option: u8) -> Result<DataConverter, Error> {
    let value = match row.get(element) {
        Some(x) => x.clone(),
        None => return Err(Error::DataConversionError)
    };
    let data = match cast_option {
        1 => {
            match value {
                DataType::Int(value) => {
                    if value < 0 {
                        return Err(Error::DataConversionError);
                    }
                    DataConverter::UInt(value as u32)
                },
                _ => return Err(Error::DataConversionError)
            }
        }
        2 => {
            match value {
                DataType::String(value) => DataConverter::String(value),
                DataType::Empty => DataConverter::String("".to_string()),
                _ => return Err(Error::DataConversionError)
            }
        }
        3 => {
            match value {
                DataType::Bool(value) => DataConverter::Bool(value),
                _ => return Err(Error::DataConversionError)
            }
        }
        4 => {
            match value {
                DataType::String(value) => DataConverter::Class(Class::Imsc),
                DataType::Empty => DataConverter::Class(Class::Standard),
                _ => return Err(Error::DataConversionError)
            }
        }
        5 => {
            match value {
                DataType::String(value) => DataConverter::Class(Class::German),
                DataType::Empty => DataConverter::Class(Class::Standard),
                _ => return Err(Error::DataConversionError)
            }
        }
        6 => {
            match value {
                DataType::String(value) => DataConverter::Class(Class::English),
                DataType::Empty => DataConverter::Class(Class::Standard),
                _ => return Err(Error::DataConversionError)
            }
        }
        7 => {
            match value {
                DataType::String(value) => {
                    match value.as_str() {
                        "Férfi" => DataConverter::Gender(Gender::Male),
                        "Nő" => DataConverter::Gender(Gender::Female),
                        _ => return Err(Error::DataConversionError)
                    }
                },
                _ => return Err(Error::DataConversionError)
            }
        }
        _ => return Err(Error::DataConversionError)
    };
    Ok(data)
}