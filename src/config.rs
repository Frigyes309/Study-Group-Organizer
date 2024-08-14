/// ## This is the configuration file, where the user have to give the data about the Excel files.
/// ## All the data provided in the Excel files must be in the same language
///
/// There is no default configuration, the user must provide the configuration here.
/// At every commented line, you will have to give the letter of the column,
/// that contains the required data commented above the line
///
/// If the column's name is bigger than one letter, for example "AA", you will receive an error.
/// Therefore, the reduction of the Excel table is recommended to one letter named columns.

/// ## GTBConfig() returns the configuration of the gtb.xlsx file
/// ### Required columns:
/// - Neptun code
/// - Room senior
/// - Card senior
/// - Color
pub fn gtb_config() -> Vec<u8> {
    let mut config: Vec<char> = Vec::new();

    // Neptun code
    config.push('A');

    // Room senior
    config.push('C');

    // Card senior
    config.push('B');

    // Color
    config.push('D');

    // Return the configuration in the required formatting
    chars_to_u8(config)
}

/// ## DormitoryConfig() returns the configuration of the koli.xlsx file
/// ### Required columns:
/// - Neptun code
/// - Room
/// - Color
pub fn dormitory_config() -> Vec<u8> {
    let mut config: Vec<char> = Vec::new();

    // Neptun code
    config.push('B');

    // Room
    config.push('A');

    // Color
    config.push('C');

    // Return the configuration in the required formatting
    chars_to_u8(config)
}

/// ## DOConfig() returns the configuration of the dh.xlsx file
/// ### Required columns:
/// - Neptun code
/// - Name
/// - Zip
/// - Gender
/// - IMSc class
/// - Double passive
/// - German class
pub fn do_config() -> Vec<u8> {
    let mut config: Vec<char> = Vec::new();

    // Neptun code
    config.push('B');

    // Name
    config.push('C');

    // Zip
    config.push('D');

    // Gender
    config.push('E');

    // IMSc class
    config.push('F');

    // Double passive
    config.push('G');

    // German class
    config.push('H');

    // Return the configuration in the required formatting
    chars_to_u8(config)
}

/// ## SeniorConfig() returns the configuration of the tsz.xlsx file
/// ### Required columns:
/// - Course code
/// - Preferred color
pub fn senior_config() -> Vec<u8> {
    let mut config: Vec<char> = Vec::new();

    // Course code
    config.push('A');

    // Preferred color
    config.push('O');

    // Return the configuration in the required formatting
    chars_to_u8(config)
}

fn chars_to_u8(chars: Vec<char>) -> Vec<u8> {
    let mut u8s: Vec<u8> = Vec::new();
    let vocabulary = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    for c in chars {
        let mut i = 0;
        for v in vocabulary.chars() {
            if c == v {
                u8s.push(i);
                break;
            }
            i += 1;
        }
    }
    u8s
}
