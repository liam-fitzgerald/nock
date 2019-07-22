#[derive(Debug)]
enum Noun {
  Atom(u64),
  Cell(Vec<Noun>),
}

pub fn main(input: String) {
  println!("{}", input);
  let parsed = parse(input);
  println!("{:?}", parsed);
}

fn parse(input: String) -> Noun {
  let mut iter = input.chars();

  fn parse_recursive(mut iter: &mut std::str::Chars<'_>) -> Noun {
    let mut cell = Vec::new();
    let mut atom: Option<u64> = None;

    while let Some(c) = iter.next() {
      match c {
        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => match atom {
          Some(num) => atom = Some(num * 10 + c as u64),
          None => atom = Some(c as u64),
        },
        '[' => match atom {
          Some(num) => {
            cell.push(Noun::Atom(num));
            cell.push(parse_recursive(&mut iter));
            atom = None;
          }
          None => {
            cell.push(parse_recursive(&mut iter));
          }
        },
        ']' => match atom {
          Some(num) => {
            cell.push(Noun::Atom(num));
            break;
          }
          None => {
            break;
          }
        },
        ' ' => match atom {
          Some(num) => {
            cell.push(Noun::Atom(num));
            atom = None;
          }
          None => cell.push(parse_recursive(&mut iter)),
        },
        _ => panic! {"Illegal character: {}", c},
      };
    }
    Noun::Cell(cell)
  }

  let noun = parse_recursive(&mut iter);

  return noun;
}

fn nock(noun: Noun) {
  println!("{:?}", noun)
}

