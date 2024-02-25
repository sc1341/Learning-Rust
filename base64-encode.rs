fn encode_base64(input: &str) -> String{
    let base64_chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

    let bytes = input.as_bytes();
    let mut binary_string = String::new();

    for &b in bytes {
        binary_string.push_str(&format!("{:08b}", b));
    }

    let padding_bits = (6 - binary_string.len() % 6) % 6;
    binary_string.push_str(&"0".repeat(padding_bits));

    let chunks: Vec<&str> = binary_string
        .as_bytes()
        .chunks(6)
        .map(std::str::from_utf8)
        .collect::<Result<Vec<&str>, _>>().unwrap();


        let mut encoded_string = String::new();

        for chunk in chunks{
            let num = u8::from_str_radix(chunk, 2).unwrap();
            encoded_string.push(base64_chars.chars().nth(num as usize).unwrap());
        }

        let padding = match encoded_string.len() % 4 {
            0 => 0,
            n => 4 - n,

        };
        encoded_string.push_str(&"=".repeat(padding));

        encoded_string
}




fn main() {
    let test1 : &str = "Hello world";
    println!("{}", encode_base64(test1));
}
