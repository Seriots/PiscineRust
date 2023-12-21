use rayon::prelude::*;

enum ArgsError {
    InvalidUsage,
    InvalidNumber,
}

fn get_input() -> Result<i32, ArgsError> {
    let args = match std::env::args().len() {
        2 => std::env::args().nth(1).unwrap(),
        _ => {
            return Err(ArgsError::InvalidUsage);
        }
    };

    match args.parse::<i32>() {
        Ok(n) => return Ok(n),
        Err(_) => {
            return Err(ArgsError::InvalidNumber);
        }
    };
}

fn without_rayon(n: i32) {
    println!("---Without rayon--");
    let time = std::time::Instant::now();
    let pi = (0..n).into_iter().map(|_| {
        let x = ftkit::random_number(0..10000000) as f64 / 10000000.0;
        let y = ftkit::random_number(0..10000000) as f64 / 10000000.0;
        if x * x + y * y <= 1.0 {
            1
        } else {
            0
        }
    }).sum::<i32>() as f64 / n as f64 * 4.0;
    println!("n = {}", pi);
    println!("time = {}ms", time.elapsed().as_millis());
}

fn with_rayon(n: i32) {
    println!("\n----With rayon----");
    let time = std::time::Instant::now();
    let pi = (0..n).into_par_iter().map(|_| {
        let x = ftkit::random_number(0..10000000) as f64 / 10000000.0;
        let y = ftkit::random_number(0..10000000) as f64 / 10000000.0;
        if x * x + y * y <= 1.0 {
            1
        } else {
            0
        }
    }).sum::<i32>() as f64 / n as f64 * 4.0;
    println!("n = {}", pi);
    println!("time = {}ms", time.elapsed().as_millis());
}

fn main() {
    let n = match get_input() {
        Ok(n) => n,
        Err(err) => match err {
            ArgsError::InvalidUsage => return eprintln!("Error: Invalid usage"),
            ArgsError::InvalidNumber => return eprintln!("Error: Invalid number"),
        }
    };
    without_rayon(n);
    with_rayon(n);

}
