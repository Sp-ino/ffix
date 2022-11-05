use crate::types::Roundings;


pub fn find_rounding(label: &str) -> Roundings {
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