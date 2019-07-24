use std::fmt;
use Noun::*;

#[derive(Debug)]
enum Noun {
  Atom(u32),
  Cell(Vec<Noun>),
}

impl Noun {
  fn unwrap_cell(&self) -> Option<&Vec<Noun>> {
    match *self {
      Atom(_) => None,
      Cell(ref vec) => Some(vec),
    }
  }
}

impl fmt::Display for Noun {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    fn traverse(cell: &Noun) -> String {
      let string = &mut String::from("[");
      match &cell {
        Atom(_) => panic! {"Traverse works by cell"},
        Cell(v) => {
          for noun in v.iter() {
            match noun {
              Atom(n) => string.push_str(&(" ".to_string() + &n.to_string() + " ")),
              Cell(_) => {
                string.push_str(&traverse(&noun));
                string.push_str("]")
              }
            };
          }
        }
      };
      string.to_string()
    };

    let result = traverse(&self) + "]";

    write!(f, "{}", result)
  }
}

pub fn main(input: String) {
  // println!("{}", input);
  let mut parsed = parse_from_string(input);
  // println!("{:?}", parsed);
  parsed = enforce_pairs(&parsed);
  println!("{}", parsed);
}

fn parse_from_string(input: String) -> Noun {
  let mut iter = input.chars();
  if iter.next().unwrap() != '[' {
    panic! {"Nock syntax must begin with an open bracket."}
  };

  fn parse_recursive(mut iter: &mut std::str::Chars<'_>) -> Noun {
    let mut cell = Vec::new();
    let mut atom: Option<u32> = None;

    while let Some(c) = iter.next() {
      match c {
        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => match atom {
          Some(num) => atom = Some(num * 10 + c.to_digit(10).unwrap()),
          None => atom = Some(c.to_digit(10).unwrap()),
        },
        '[' => match atom {
          Some(num) => {
            cell.push(Atom(num));
            cell.push(parse_recursive(&mut iter));
            atom = None;
          }
          None => {
            cell.push(parse_recursive(&mut iter));
          }
        },
        ']' => match atom {
          Some(num) => {
            cell.push(Atom(num));
            break;
          }
          None => {
            break;
          }
        },
        ' ' => match atom {
          Some(num) => {
            cell.push(Atom(num));
            atom = None;
          }
          None => (),
        },
        _ => panic! {"Illegal character: {}", c},
      };
    }
    Cell(cell)
  }

  parse_recursive(&mut iter)
}

fn enforce_pairs(noun: &Noun) -> Noun {
  let vector = noun.unwrap_cell().unwrap();
  let len = vector.len();
  let mut new_vec: Vec<Noun> = vec![];
  match len {
    0 | 1 => panic! {"Cells must have length of exactly 2"},
    2 => {
      new_vec.push(match &vector[0] {
        Atom(n) => Atom(*n),
        Cell(_) => enforce_pairs(&vector[0]),
      });
      new_vec.push(match &vector[1] {
        Atom(n) => Atom(*n),
        Cell(_) => enforce_pairs(&vector[1]),
      });
    }
    _ => {
      new_vec.push(match &vector[0] {
        Atom(n) => Atom(*n),
        Cell(_) => enforce_pairs(&vector[0]),
      });
      let mut rest_of_vec: Vec<Noun> = vec![];
      let mut iter = vector.iter();
      iter.next();
      while let Some(elem) = iter.next() {
        rest_of_vec.push(match elem {
          Atom(n) => Atom(*n),
          Cell(_) => panic! {"Incorrectly formatted nock"},
        });
      }
      new_vec.push(enforce_pairs(&Cell(rest_of_vec)));
    }
  };
  Cell(new_vec)
}

// fn nock(noun: Noun) {
//   println!("{:?}", noun)
// }

