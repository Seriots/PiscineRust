use unicode_width::UnicodeWidthStr;

fn handle_text(text: &[String], columns: usize) {
    let mut width = 0;
    let mut i = 0;

    for elem in 0..text.len() {
        if width + UnicodeWidthStr::width(text[elem].as_str()) > columns {
            let mut size = elem - i - 1;
            width -= size;
            for j in i..elem {
                print!("{}", text[j]);
                let spaces;
                if size != 0 {
                    if j == elem - 2 {
                        spaces = columns - width;
                    }
                    else {
                        spaces = (columns - width) / size;
                    }
                    size -= 1;
                    width += spaces;
                    for _ in 0..spaces {
                        print!(" ");
                    }
                }
            }
            i = elem;
            width = UnicodeWidthStr::width(text[elem].as_str());
            println!();
        }
        else {
            if width == 0 {
                width += UnicodeWidthStr::width(text[elem].as_str());
            }
            else {
                width += UnicodeWidthStr::width(text[elem].as_str()) + 1;
            }
        }
    }
    for j in i..text.len() {
        print!("{}", text[j]);
        if j != text.len() - 1 {
            print!(" ");
        }
    }
    println!();
}

fn main() {
    assert_eq!(ftkit::ARGS.len(), 2);
    let columns: usize = ftkit::ARGS[1].parse().unwrap();

    let mut text = Vec::new();
    let mut newlines_count = 0;

    loop {
        let line = ftkit::read_line();

        if line.is_empty() {
            handle_text(&text, columns);
            break;
        }
        if line.trim().is_empty() {
            newlines_count += 1;
        } 
        else {
            if newlines_count >= 2 {
                handle_text(&text, columns);
                text.clear();
                println!();
            }
            newlines_count = 0;
        }

        for word in line.split_whitespace() {
            text.push(word.to_string());
        }
        
    }
}
