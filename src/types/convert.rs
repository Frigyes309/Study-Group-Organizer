use office::DataType;

trait FromDataType {
    fn from_data_type(data: &DataConverter) -> Self;
}

// Example DataType
pub enum DataConverter {
    UInt(u32),
    String(String),
    Bool(bool),
    Class(super::Class),
    Gender(super::Gender)
}

impl DataConverter {
    pub fn to_string(self) -> Option<String> {
        match self {
            DataConverter::String(value) => Some(value),
            _ => None
        }
    }

    pub fn to_uint(self) -> Option<u32> {
        match self {
            DataConverter::UInt(value) => Some(value),
            _ => None
        }
    }

    pub fn to_bool(self) -> Option<bool> {
        match self {
            DataConverter::Bool(value) => Some(value),
            _ => None
        }
    }

    pub fn to_class(self) -> Option<super::Class> {
        match self {
            DataConverter::Class(value) => Some(value),
            _ => None
        }
    }

    pub fn to_gender(self) -> Option<super::Gender> {
        match self {
            DataConverter::Gender(value) => Some(value),
            _ => None
        }
    }
}

// pub fn convert<T: FromDataType>(data: &DataConverter) -> T {
//     T::from_data_type(data)
// }

// // Implementing the trait for various types
// impl FromDataType for u32 {
//     fn from_data_type(data: &DataConverter) -> Self {
//         if let DataConverter::UInt(value) = data {
//             *value
//         } else {
//             panic!("Expected an unsigned integer");
//         }
//     }
// }

// impl FromDataType for String {
//     fn from_data_type(data: &DataConverter) -> Self {
//         if let DataConverter::String(value) = data {
//             value.clone()
//         } else {
//             panic!("Expected a string");
//         }
//     }
// }

// impl FromDataType for bool {
//     fn from_data_type(data: &DataConverter) -> Self {
//         if let DataConverter::Bool(value) = data {
//             value.clone()
//         } else {
//             panic!("Expected a string");
//         }
//     }
// }

// impl FromDataType for super::Class {
//     fn from_data_type(data: &DataConverter) -> Self {
//         if let DataConverter::Class(value) = data {
//             value.clone()
//         } else {
//             panic!("Expected a string");
//         }
//     }
// }

// impl FromDataType for super::Gender {
//     fn from_data_type(data: &DataConverter) -> Self {
//         if let DataConverter::Gender(value) = data {
//             value.clone()
//         } else {
//             panic!("Expected a string");
//         }
//     }
// }