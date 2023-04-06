// 1
// User inputs some text and a key
// Output is the encrypted text
//
// 2
// User inputs encrypted text and a key
// Output is the decrypted text

fn main() {
    println!("Hello, world!");
}

fn encrypt(text: &String, key: &String) -> String {
    let key_len = key.len();
    let key_bytes: Vec<&u8> = key.as_bytes().iter().collect();
    let mut result = String::new();
    for (i, input_byte) in text.as_bytes().iter().enumerate() {
        if *input_byte == ' ' as u8 { continue };
        let key_byte = key_bytes[i % key_len];
        let encrypted_byte = encrypt_byte(input_byte, key_byte);
        result.push(encrypted_byte as char);
    }
    println!("{result}");
    result.to_uppercase()
}

fn encrypt_byte(input_byte: &u8, key_byte: &u8) -> u8 {
    (((input_byte + key_byte) - 2 * ('a' as u8)) % 26) + 'a' as u8
}

fn decrypt_byte(encrypted_byte: &u8, key_byte: &u8) -> u8 {
    println!("{} {}", encrypted_byte, key_byte);
    let diff = *encrypted_byte as i32 - *key_byte as i32;
    ((diff - 2 * ('a' as u8)) % 26) + 'a' as u8
}

fn decrypt(text: &String, key: &String) -> String {
    // Todo
    String::new()
}

mod tests {

    #[test]
    fn test_encrypt_byte() {
        let input_bytes = ['a' as u8, 'o' as u8];
        let key_bytes = ['a' as u8, 'm' as u8];
        let encrypted_bytes = ['a' as u8, 'a' as u8];
        for (i, input_byte) in input_bytes.iter().enumerate() {
            assert_eq!(super::encrypt_byte(&input_byte, &key_bytes[i]), encrypted_bytes[i]);
        }
    }

    #[test]
    fn test_decrypt_byte() {
        let encrypted_bytes = ['a' as u8, 'a' as u8];
        let key_bytes = ['a' as u8, 'm' as u8];
        let decrypted_bytes = ['a' as u8, 'o' as u8];
        for (i, encrypted_byte) in encrypted_bytes.iter().enumerate() {
            assert_eq!(super::decrypt_byte(&encrypted_byte, &key_bytes[i]), decrypted_bytes[i]);
        }
    }

    #[test]
    fn test_encrypt() {
        let text = String::from("attackatdawn");
        let key = String::from("lemon");
        assert_eq!(super::encrypt(&text, &key), "LXFOPVEFRNHR");
        // encrypt should remove spaces from the input
        let text = String::from("hello world");
        let key = String::from("a");
        assert_eq!(super::encrypt(&text, &key), "HELLOWORLD");
    }

    #[test]
    fn test_decrypt() {
        let text = String::from("LXFOPVEFRNHR");
        let key = String::from("lemon");
        assert_eq!(super::decrypt(&text, &key), "attackatdawn");
        // decrypt won't add spaces back
        let text = String::from("helloworld");
        let key = String::from("a");
        assert_eq!(super::decrypt(&text, &key), "helloworld");
    }
}
