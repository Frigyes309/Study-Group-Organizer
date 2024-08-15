use colored::Colorize;

// Debug should print out the secret and detailed errors
pub enum Error {
    InfoError,
    NoSheetsFound,
    CannotOpenExcelFile,
    DataConversionError,
    MissingData,
    MissingFilename,
    ForeignError,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::InfoError => write!(
                f,
                "{} Not enough parameters for information",
                "[Error]".red()
            ),
            Error::NoSheetsFound => {
                write!(f, "{} No sheets found in the workbook", "[Error]".red())
            }
            Error::CannotOpenExcelFile => {
                write!(f, "{} Cannot open the Excel file", "[Error]".red())
            }
            Error::DataConversionError => {
                write!(f, "{} Cannot convert data from excel", "[Error]".red())
            }
            Error::MissingData => write!(f, "{} Missing data in the Excel file", "[Error]".red()),
            Error::MissingFilename => {
                write!(f, "{} Missing filename in the path", "[Error]".red())
            }
            Error::ForeignError => write!(f, "{} Foreign error", "[Error]".red()),
        }
    }
}

pub enum Info {
    MultipleSheetsFound,
    FoundEmptyCell,
}

impl Info {
    pub fn print(self, params: Vec<String>) -> Result<(), Error> {
        match self {
            Info::MultipleSheetsFound => {
                let filename = match params.get(0) {
                    Some(p) => p.clone(),
                    None => return Err(Error::InfoError),
                };
                let sheetname = match params.get(1) {
                    Some(p) => p.clone(),
                    None => return Err(Error::InfoError),
                };
                println!(
                    "{} Multiple sheets found in {}, using sheet {}",
                    "[Info]".yellow(),
                    filename,
                    sheetname
                );
                Ok(())
            }
            Info::FoundEmptyCell => {
                let filename = match params.get(0) {
                    Some(p) => p.clone(),
                    None => return Err(Error::InfoError),
                };
                println!(
                    "{} Found empty cell in the Excel file {}",
                    "[Info]".yellow(),
                    filename
                );
                Ok(())
            }
        }
    }
}

impl From<anyhow::Error> for Error {
    fn from(error: anyhow::Error) -> Self {
        Error::ForeignError
    }
}

impl From<office::Error> for Error {
    fn from(error: office::Error) -> Self {
        Error::ForeignError
    }
}