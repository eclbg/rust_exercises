// This program converts temperatures between Fahrenheit and Celsius

fn main() {
    println!("Welcome to the temperature conversion tool");
    loop {
        let conv_func = get_conv_type_from_user();
        let input_temp = get_input_temp_from_user();
        let result = conv_func(input_temp);
        println!("The converted temperature is {result}")
    }
}

fn get_input_temp_from_user() -> f32 {
    let input_temp = loop {
        let mut input_temp = String::new();
        println!("Type in the temperature you'd like to convert:");
        std::io::stdin()
            .read_line(&mut input_temp)
            .expect("Input error");

        let input_temp: f32 = match input_temp.trim().parse() {
            Err(_) => {
                println!("Your input is not a valid number");
                continue;
            }
            Ok(num) => num,
        };
        break input_temp;
    };
    input_temp
}

fn get_conv_type_from_user() -> fn(f32) -> f32 {
    let conv_func = loop {
        let conv_func: fn(f32) -> f32;
        let mut conv_type = String::new();
        println!("Choose the conversion you want to perform:");
        println!("1: Fahrenheit to Celsius");
        println!("2: Celsius to Fahrenheit");
        std::io::stdin()
            .read_line(&mut conv_type)
            .expect("Input error");

        let conv_type: usize = match conv_type.trim().parse() {
            Err(_) => {
                println!("Your input is not a valid number");
                continue;
            }
            Ok(num) => num,
        };

        if conv_type == 1 {
            conv_func = fahrenheit_to_celsius;
            break conv_func;
        } else if conv_type == 2 {
            conv_func = celsius_to_fahrenheit;
            break conv_func;
        } else {
            println!("Please input 1 or 2");
        };
    };
    conv_func
}

fn fahrenheit_to_celsius(temp: f32) -> f32 {
    (temp - 32.) * 0.5556
}

fn celsius_to_fahrenheit(temp: f32) -> f32 {
    (temp * 1.8) + 32.
}

mod tests {

    #[test]
    fn test_fahrenheit_to_celsius() {
        let inputs = [32.];
        let outputs = [0.];
        let mut idx = 0;
        while idx < inputs.len() {
            assert!(super::fahrenheit_to_celsius(inputs[idx]) == outputs[idx]);
            idx += 1;
        }
    }

    #[test]
    fn test_celsius_to_fahrenheit() {
        let inputs = [0.];
        let outputs = [32.];
        let mut idx = 0;
        while idx < inputs.len() {
            let result = super::celsius_to_fahrenheit(inputs[idx]);
            println!("{result}");
            assert!(result == outputs[idx]);
            idx += 1;
        }
    }
}
