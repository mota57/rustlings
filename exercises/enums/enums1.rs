// enums1.rs
// No hints this time! ;)


#[derive(Debug)]
enum Message {
    Quit, 
    Echo,
    Move, 
    ChangeColor
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
    let q = Message::Quit;
    match q  {
        Message::Quit => {
            assert_eq!(1,1);
        },
        _ => {
            assert_eq!(1,0);
        }
    }
    println!("{:?}", Message::Quit);
}
