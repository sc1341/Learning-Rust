fn main() {
    // Hexadecimal to decimal
    let hex_value = 0x1A; // 26 in decimal
    println!("Hexadecimal 0x1A to decimal: {}", hex_value);

    // Binary to decimal
    let binary_value = 0b11010; // 26 in decimal
    println!("Binary 0b11010 to decimal: {}", binary_value);

    // Decimal to hexadecimal string
    let decimal_value = 26;
    let hex_string = format!("{:X}", decimal_value); // "1A"
    println!("Decimal 26 to hexadecimal string: {}", hex_string);

    // Decimal to binary string
    let binary_string = format!("{:b}", decimal_value); // "11010"
    println!("Decimal 26 to binary string: {}", binary_string);

    // String to decimal (parsing)
    let hex_str = "1A";
    let hex_to_dec = i32::from_str_radix(hex_str, 16).unwrap(); // 26
    println!("Hex string '1A' to decimal: {}", hex_to_dec);

    let binary_str = "11010";
    let binary_to_dec = i32::from_str_radix(binary_str, 2).unwrap(); // 26
    println!("Binary string '11010' to decimal: {}", binary_to_dec);
}
