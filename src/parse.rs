use crate::token;

type Iter<'s> = std::iter::Peekable<std::iter::Enumerate<std::str::Chars<'s>>>;

pub fn parse<Read, Write>(source: &mut Read, prompt: &mut Option<Write>)
where
    Read: std::io::BufRead,
    Write: std::io::Write,
{
    let mut s = String::new();
    if let Some(prompt) = prompt {
        write!(prompt, "> ").unwrap();
        prompt.flush().unwrap();
    }
    let mut chars: Iter = read_line(source, &mut s).enumerate().peekable();
    loop {
        while let Some(token) = token::next(&mut chars) {
            println!("{:?}", token);
        }
        if false {
            if let Some(prompt) = prompt {
                write!(prompt, "+ ").unwrap();
                prompt.flush().unwrap();
            }
            chars = read_line(source, &mut s).enumerate().peekable();
        } else {
            break;
        }
    }
}

fn read_line<'s, Read: std::io::BufRead>(
    source: &mut Read,
    s: &'s mut String,
) -> std::str::Chars<'s> {
    *s = String::new();
    source.read_line(s).unwrap();
    s.chars()
}