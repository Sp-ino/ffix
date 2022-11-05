use crate::types::Roundings;


pub fn find_rounding(label: &str) -> Roundings {
    // This function maps a string literal, which
    // should be one of "Floor", "Ceil", "Zero",
    // "Infinity", to a variant of the Rounding
    // enum
    match label {
        "Floor" => Roundings::Floor,
        "Ceil" => Roundings::Ceil,
        "Zero" => Roundings::Zero,
        "Infinity" => Roundings::Infinity,
        _ => panic!("Invalid specifier for rounding method. The correct specifiers are:
    - Floor
    - Ceil
    - Zero
    - Infinity"),
    }
}