// strings3.rs
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a hint.
use std::convert::TryInto;


fn trim_me(input: &str) -> String {
    let mut chrs = input.chars();
    let total = input.len();
    let mut index_start = 0;
    for (i,c) in chrs.enumerate() {
        if c != ' ' {
            index_start = i;
            break;
        }
    }

    let mut chrs = input.chars().rev();

    let mut index_end = total;
    for (i,c) in chrs.enumerate() {
        index_end -= 1;
        if c != ' ' {
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

    let mut content = String::new();

    let word = "cars";
    let replace_content = "balloons";

    let offset = word.len();
    let input_len = input.len();
    let letter0 = &word[0..1];
    

    log_main_var(word, replace_content, offset, input_len, letter0 );

    let mut i = 0;

    if input == word {
        return replace_content.to_string();
    }


    loop {

        log(i, &content);

        if i >= input_len {
            break;
        }

        let c = &input[i..i+1];
        if c == letter0 {
            let ix_stop = i + offset; //not inclusive
            
            if ix_stop > input_len {
                break;
            }
            let part = &input[i..ix_stop];

            if word == part  {
                println!("found word {} at start = {}  end= {}, part = {}", word, i, ix_stop, part);
                println!("skip to stop+1 = {}", ix_stop);
                content.push_str(replace_content);
                i = ix_stop;
                continue;
            } 
        } 
        content.push_str(c);
        i+=1;
    }
    println!("=======================================");

    content
}
fn log(i:usize, content:&String) {
    println!("DEBUG:: i={}\ncontent={}",i, content);


}

fn log_main_var(word:&str, replace_content:&str, offset:usize, input_len:usize, letter0:&str ) {

    println!("DEBUG:: word={}\n replace_content={}\n offset={}\n input_len={}\n letter0={}\n"
     , word, replace_content, offset, input_len, letter0);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
        assert_eq!(trim_me("                Hola!                                   "), "Hola!");
        assert_eq!(trim_me("                Hola! \n                                   "), "Hola! \n");

    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(replace_me("Icars think cars are cool"), "Iballoons think balloons are cool");
        // assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
    }
}
