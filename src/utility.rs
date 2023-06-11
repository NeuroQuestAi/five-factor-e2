
fn big5_ocean_is_valid(label: &str) -> Result<bool, &'static str> {
    match label {
        "O" | "C" | "E" | "A" | "N" => Ok(true),
        _ => Err("The Big-Five label is invalid!"),
    }
}


