pub fn convert(s: String, num_rows: i32) -> String {
    if num_rows == 1 {
        return s;
    }

    let rows = num_rows as usize;
    let cycle_length = 2 * rows - 2;
    let s = s.into_bytes();

    let mut result: Vec<u8> = Vec::new();

    for i in 0..rows {  
        let idx = i;
        let mut offset = 0usize;
        while idx + offset < s.len() {
            result.push(s[offset + idx]);
            if idx != 0 && idx != rows - 1 && offset + cycle_length - idx < s.len() {
                result.push(s[offset + cycle_length - idx]);
            }
            offset += cycle_length;
        }
    }

    String::from_utf8(result).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_example_1() {
        let s = "PAYPALISHIRING".to_string();
        let rows = 3;
        let result = convert(s, rows);
        let expected = "PAHNAPLSIIGYIR".to_string();
        
        assert_eq!(expected, result);
    }

    #[test]
    fn test_example_2() {
        let s = "PAYPALISHIRING".to_string();
        let rows = 4;
        let result = convert(s, rows);
        let expected = "PINALSIGYAHRPI".to_string();
        
        assert_eq!(expected, result);
    }
}
