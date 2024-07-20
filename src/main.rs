// TODO: implement dynamic day and part call using custom macro
fn main() {
    let mut day: Option<u8> = None;
    let mut part: Option<u8> = None;

    // we want to run a command like cargo run --day=1 --part=1
    for (i, argument) in std::env::args().enumerate() {
        match argument.as_str() {
            "-d" => {
                if (i + 1) < std::env::args().len() {
                    day = Some(std::env::args().nth(i + 1).unwrap().parse().unwrap());
                }
            }
            "-p" => {
                if (i + 1) < std::env::args().len() {
                    part = Some(std::env::args().nth(i + 1).unwrap().parse().unwrap());
                }
            }
            _ => {}
        };
    }

    if !day.is_some() || !part.is_some() {
        panic!("You need to provide day and part arguments.");
    }

    println!("Day: {}; Part: {}", day.unwrap(), part.unwrap());
}
