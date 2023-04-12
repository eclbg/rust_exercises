// options2.rs
// Execute `rustlings hint options2` or use the `hint` watch subcommand for a hint.

fn main() {}

#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // TODO: Make this an if let statement whose value is "Some" type
        if let Some(word) = optional_target {
            assert_eq!(word, target);
        }
    }

    #[test]
    fn layered_option() {
        let mut range = 10;
        let mut optional_integers: Vec<Option<i8>> = Vec::new();
        for i in 0..(range + 1) {
            optional_integers.push(Some(i));
        }

        println!("while let");
        while let Some(Some(integer)) = optional_integers.pop() {
            println!("{integer}");
            assert_eq!(integer, range);
            range -= 1;
        }

        let mut range = 10;
        let mut optional_integers2: Vec<Option<i8>> = Vec::new();
        for i in 0..(range + 1) {
            optional_integers2.push(Some(i));
        }

        println!("ugly thing");
        // The below is equivalent to the while let above
        loop {
            match optional_integers2.pop() {
                Some(optional_integer) => match optional_integer {
                    Some(integer) => {
                        println!("{integer}");
                        assert_eq!(integer, range);
                        range -= 1
                    }
                    _ => break,
                },
                _ => break,
            }
        }
    }
}
