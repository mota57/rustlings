// strings3.rs
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a hint.
use std::convert::TryInto;


fn trim_me(input: &str) -> String {
    //method trim()
    let total = input.len();
    let mut index_start = 0;
    for i in 0..total {
        if &input[i] != "\\s" {
            index_start = i;
            break;
        }
    }

    let mut index_end = 0;
    for i in (total-1)..=0 {
        if &input[i] != "\\s" {
            index_end = i;
            break;
        }
    }

    let mut content = String::new();
    content.push_str(&input[index_start..=index_end]);

    content
}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There's multiple ways to do this!
    let mut content = String::new();
    content.push_str(&input);
    content.push_str(" world!");

    content
}

fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons"!

    let mut content = String::new();

    let word = "cards";
    let replace = "ballons";
    let offset:i32 = word.len().try_into().unwrap();
    let input_len:i32 = input.len().try_into().unwrap();
    let iterations:i32 = input_len / offset;

    let mut ix_start = 0;
    let mut ix_end = offset;

    for _ in 0..iterations {
        if ix_start >= input_len {
            break;
        } else if ix_end >= input_len {
            content.push_str(&input[ix_start..input_len]);
            break;
        }

        let part = &input[ix_start..ix_end];
        if part == word  {
            content.push_str(replace);
        } else {
            content.push_str(part);
        }
        ix_start = ix_end + 1;
        ix_end = ix_start + offset
    }

    content
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
        assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
    }
}
