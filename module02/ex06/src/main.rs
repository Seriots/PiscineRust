#![allow(dead_code)]

#[derive(Debug)]
enum Token<'a> {
    Word(&'a str),
    RedirectStdout,
    RedirectStdin,
    Pipe,
}

fn next_token<'b>(s: &'b mut &str) -> Option<Token<'b> > {
    *s = s.trim_start();
    if s.is_empty() {
        return None;
    }
    let mut index = s.char_indices();

    while let Some((i, c)) = index.next() {
        if i == 0 {
            if c == '>' {
                *s = s.split_at(1).1;
                return Some(Token::RedirectStdout);
            }
            else if c == '<' {
                *s = s.split_at(1).1;
                return Some(Token::RedirectStdin);
            }
            else if c == '|' {
                *s = s.split_at(1).1;
                return Some(Token::Pipe);
            }
        }
        else
        {
            if c.is_whitespace() || c == '>' || c == '<' || c == '|' || i >= s.len() - 1 {
                let word;
                (word, *s) = s.split_at(i);
                return Some(Token::Word(word));

            }
        }
    }
    return None;
}

fn print_all_tokens(mut s: &str) {
    while let Some(token) = next_token(&mut s) {
        println!("{token:?}");

    }
}


fn main() {
    assert_eq!(ftkit::ARGS.len(), 2);
    
    print_all_tokens(&ftkit::ARGS[1]);
}
