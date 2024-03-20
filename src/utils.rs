
pub fn parse_hex_u32_err_to_0(n_str: &str) -> u32 {
    match u32::from_str_radix(n_str, 16) {
        Ok(steps) => steps,
        Err(_) => 0,
    }
}

pub fn parse_i32_err_to_min(n_str: &str) -> i32 {
    match i32::from_str_radix(n_str, 10) {
        Ok(steps) => steps,
        Err(_) => i32::MIN,
    }
}

pub fn split_string(line: String) -> Vec<String> {
    let mut res = Vec::new();

    line.trim().split_ascii_whitespace().for_each(|elem| {
        res.push(elem.to_owned());
    });
    return res;
}
