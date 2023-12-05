use std::cmp::Ordering;

fn main() {
    let num = ftkit::random_number(1..=100);

    println!("Me and my infinite wisdom have found an appropriate secret you shall yearn for.");
    loop {
        let guess = ftkit::read_number();
        match guess.cmp(&num) {
            Ordering::Less => println!("This student might not be as smart as I was told. This answer is obviously too weak."),
            Ordering::Greater => println!("Sometimes I wonder whether I should retire. I would have guessed higher."),
            Ordering::Equal => break,
        }
    }
    println!("That is right! The secret was indeed the number {}, which you have brilliantly discovered!", num);
}
