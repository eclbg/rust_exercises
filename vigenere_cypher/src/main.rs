// Vigenere cyper: https://en.wikipedia.org/wiki/Vigen%C3%A8re_cipher
//
// Only supports ascii characters and spaces as inputs. Spaces are ignored.
// Converts input to lowercase and output is always lowercase

use std::iter;

fn main() {
}

fn encrypt(text: &str, key: &str) -> String {
    let mut result = String::new();
    let lowercase_text = text.to_lowercase();
    let zipped = iter::zip(
        lowercase_text.as_bytes().iter(),
        iter::repeat(key.as_bytes().iter()).flatten()
    );
    for (&input_byte, &key_byte) in zipped {
        if input_byte == b' ' {
            continue;
        }
        let encrypted_byte = encrypt_byte(input_byte, key_byte);
        result.push(encrypted_byte as char);
    }
    println!("{result}");
    result
}

fn decrypt(text: &str, key: &str) -> String {
    let mut result = String::new();
    let lowercase_text = text.to_lowercase();
    let zipped = iter::zip(
        lowercase_text.as_bytes().iter(),
        iter::repeat(key.as_bytes().iter()).flatten()
    );
    for (&input_byte, &key_byte) in zipped {
        if input_byte == b' ' {
            continue;
        }
        let decrypted_byte = decrypt_byte(input_byte, key_byte);
        result.push(decrypted_byte as char);
    }
    println!("{result}");
    result
}

fn encrypt_byte(input_byte: u8, key_byte: u8) -> u8 {
    ((input_byte - b'a') + (key_byte - b'a')).rem_euclid(26) + b'a'
}

fn decrypt_byte(encrypted_byte: u8, key_byte: u8) -> u8 {
    let diff = encrypted_byte as i32 - key_byte as i32;
    ((diff.rem_euclid(26)) + 'a' as i32) as u8
}

mod tests {

    #[test]
    fn test_encrypt_byte() {
        let input_bytes = [
            b'a',
            b'o',
            b't'
        ];
        let key_bytes = [
            b'a',
            b'm',
            b'e'
        ];
        let encrypted_bytes = [
            b'a',
            b'a',
            b'x'
        ];
        for (i, &input_byte) in input_bytes.iter().enumerate() {
            assert_eq!(super::encrypt_byte(input_byte, key_bytes[i]), encrypted_bytes[i]);
        }
    }

    #[test]
    fn test_decrypt_byte() {
        let encrypted_bytes = [
            b'a',
            b'a',
            b'x'
        ];
        let key_bytes = [
            b'a',
            b'm',
            b'e'
        ];
        let decrypted_bytes = [
            b'a',
            b'o',
            b't'
        ];
        for (i, &encrypted_byte) in encrypted_bytes.iter().enumerate() {
            assert_eq!(super::decrypt_byte(encrypted_byte, key_bytes[i]), decrypted_bytes[i]);
        }
    }

    #[test]
    fn test_encrypt() {
        let text = String::from("attackatdawn");
        let key = String::from("lemon");
        assert_eq!(super::encrypt(&text, &key), "lxfopvefrnhr");
        // encrypt should remove spaces from the input
        let text = String::from("hello world");
        let key = String::from("a");
        assert_eq!(super::encrypt(&text, &key), "helloworld");
    }

    #[test]
    fn test_decrypt() {
        let text = String::from("lxfopvefrnhr");
        let key = String::from("lemon");
        assert_eq!(super::decrypt(&text, &key), "attackatdawn");
        // decrypt won't add spaces back
        let text = String::from("helloworld");
        let key = String::from("a");
        assert_eq!(super::decrypt(&text, &key), "helloworld");
    }
}
