
pub fn anything() -> impl Fn(&str) -> bool {
        |_: &str| true
}

pub fn exactly<'a>(to_match: &'a str) -> impl 'a + Fn(&str) -> bool {
    move |s: &str| s == to_match
}
pub fn alphabetic() -> impl Fn(&str) -> bool {
    |s: &str| s.chars().all(|c| c.is_alphabetic())
}
pub fn ascii_digits() -> impl Fn(&str) -> bool {
    |s: &str| s.chars().all(|c| c.is_ascii_digit())
}

pub fn min_size(min: usize) -> impl Fn(&str) -> bool {
    move |s: &str| s.chars().count() >= min
}

pub fn maybe(pattern: impl Fn(&str) -> bool) -> impl Fn(&str) -> bool {
    move |s: &str| s.is_empty() || pattern(s)
}

pub fn not(pattern: impl Fn(&str) -> bool) -> impl Fn(&str) -> bool {
    move |s: &str| !pattern(s)
}

pub fn or(first: impl Fn(&str) -> bool, second: impl Fn(&str) -> bool) -> impl Fn(&str) -> bool {
    move |s: &str| first(s) || second(s) 
}

pub fn and(first: impl Fn(&str) -> bool, second: impl Fn(&str) -> bool) -> impl Fn(&str) -> bool {
    move |s: &str| first(s) && second(s)
}

pub fn chain(first: impl Fn(&str) -> bool, second: impl Fn(&str) -> bool) -> impl Fn(&str) -> bool {
    move |s| (0..=s.len()).any(|i| first(&s[..i]) && second(&s[i..]))
}

fn main() {
    println!("{}", exactly("hell")("hello"));
    println!("{}", anything()("hello"));
    println!("{}", alphabetic()("hello"));
    println!("{}", alphabetic()("hell4o"));
    println!("{}", ascii_digits()("7774o"));
    println!("{}", ascii_digits()("87544"));
    println!("{}", min_size(7)("87544"));
    println!("{}", min_size(7)("8888887544"));
    println!("{}", maybe(exactly("hello"))("hello"));
    println!("{}", not(maybe(exactly("hello")))("hllo"));
    println!("{}", and(min_size(7), alphabetic())("hllfczerfo"));
    println!("{}", or(ascii_digits(), alphabetic())("hllo"));
    println!("{}", chain(ascii_digits(), alphabetic())("745hllo"));
    println!("\n");

    let pattern = chain(and(alphabetic(), min_size(1)),
                    chain(exactly("@"),
                    chain(and(alphabetic(), min_size(1)),
                        or(exactly(".fr"), exactly(".com")))));

    println!("{}",pattern("my@address.com"));
    println!("{}",pattern("a@b.fr"));
    println!("{}",!pattern("@example.com"));
    println!("{}",!pattern("abc@.com"));
    println!("{}",!pattern("my@address.net"));
    println!("{}",!pattern("myaddress.fr"));
    println!("{}",!pattern("my-address@domain.fr"));
    println!("{}",!pattern("address@my-domain.fr"));

    println!("\n");

    let pattern2 = chain(exactly("("),
                    chain(
                        maybe(
                            or(
                                not(chain(chain(anything(), exactly("(")), anything())),
                                not(chain(chain(anything(), exactly(")")), anything())),
                            )), 
                                    exactly(")")));

    println!("{}",pattern2("()"));
    println!("{}",pattern2("(hello)"));
    println!("{}",pattern2("(hello 123)"));
    println!("{}",!pattern2("(hello () 123)"));
    println!("{}",!pattern2("(hello "));
    println!("{}",!pattern2(")"));
    println!("{}",!pattern2(" (test) "));
    println!("\n");

    let pattern3 = chain(
        maybe(or(exactly("+"), exactly("-"))),
        chain(
            and(ascii_digits(), min_size(1)),
            chain(
                maybe(chain(exactly("."), and(ascii_digits(), min_size(1)))),
                maybe(chain(
                    or(exactly("e"), exactly("E")),
                    chain(
                        maybe(or(exactly("+"), exactly("-"))),
                        and(ascii_digits(), min_size(1)),
                    ),
                )),
            ),
        ),
    );

    println!("{}",pattern3("12"));
    println!("{}",pattern3("+12"));
    println!("{}",pattern3("-12"));
    println!("{}",pattern3("12.5"));
    println!("{}",pattern3("12.5e20"));
    println!("{}",pattern3("+12E10"));
    println!("{}",pattern3("12E+9"));
    println!("{}",pattern3("12E-9"));
    println!("{}",!pattern3(""));
    println!("{}",!pattern3("+"));
    println!("{}",!pattern3("+12."));
    println!("{}",!pattern3("+12e"));
    println!("{}",!pattern3("+12.e10"));
    println!("{}",!pattern3("12e10.10"));
}
