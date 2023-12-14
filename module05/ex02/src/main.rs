use std::fs::metadata;

fn get_directory_size(path: &str) -> u64 {
    let mut size = 0;
    for each in std::fs::read_dir(path).unwrap() {
        let meta = each.as_ref().unwrap().metadata().unwrap();
        size += meta.len();
        if meta.is_dir() {
            size += get_directory_size(&each.unwrap().path().to_str().unwrap());
        }

    }
    size
}

fn main() {
    if std::env::args().len() != 2 {
        eprintln!("Usage: ex02 <input>");
        return ;
    }

    let file = std::env::args().nth(1).unwrap();

    let meta = match metadata(&file) {
        Ok(meta) => meta,
        Err(e) => {
            eprintln!("error: {:?}", e);
            return ;
        }
    };

    let mut size = 0;
    if meta.is_file() {
        size = meta.len();
    } else if meta.is_dir() {
        size = get_directory_size(&file);
    } else {
        println!("{} is something else", file);
    }

    match size {
        0..=999 => println!("{size} bytes"),
        1000..=999999 => println!("{:.2} kilobytes", (size as f64)/1000.0),
        1000000..=999999999 => println!("{:.2} megabytes", (size as f64)/1000000.0),
        _ => println!("{:.2} gigabytes", (size as f64)/1000000000.0),
    }
}
