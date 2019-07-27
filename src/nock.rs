use std::fmt;
use Noun::*;

#[derive(Debug, Clone)]
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

  fn unwrap_atom(&self) -> Option<&u32> {
    match *self {
      Atom(ref n) => Some(n),
      Cell(_) => None,
    }
  }
}

impl fmt::Display for Noun {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    fn traverse(cell: &Noun) -> String {
      let mut string = String::new();
      match &cell {
        Atom(n) => string = n.to_string(),
        Cell(v) => {
          string.push_str("[");
          for noun in v.iter() {
            match noun {
              Atom(n) => string.push_str(&(" ".to_string() + &n.to_string() + " ")),
              Cell(_) => {
                string.push_str(&traverse(&noun));
              }
            };
          }
          string.push_str("]");
        }
      };
      string.to_string()
    };

    let result = traverse(&self);

    write!(f, "{}", result)
  }
}

pub fn main(input: String) {
  // println!("{}", input);
  let mut parsed = parse_from_string(input);
  // println!("{:?}", parsed);
  parsed = enforce_pairs(&parsed);
  println!("{}", parsed);
  // println!("{:?}", parsed);
  let product = nock(&parsed);
  println!("{}", product);
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
          Cell(_) => enforce_pairs(elem),
        });
      }
      new_vec.push(enforce_pairs(&Cell(rest_of_vec)));
    }
  };
  Cell(new_vec)
}

fn nock(noun: &Noun) -> Noun {
  tar(noun)
}

// ?
fn wut(noun: Noun) -> Noun {
  match noun {
    Atom(_) => Atom(1),
    Cell(_) => Atom(0),
  }
}

// +
fn lus(noun: Noun) -> Noun {
  match noun {
    Atom(n) => Atom(n + 1),
    Cell(v) => Cell(v),
  }
}

// =
fn tis(noun: Noun) -> Noun {
  match noun {
    Atom(_) => panic! {"tis atom"},
    Cell(v) => {
      // Check for deep equality of v[0] and v[1]
      if deeply_equal(&v[0], &v[1]) {
        return Atom(0);
      } else {
        return Atom(1);
      }
    }
  }
}

// /
fn fas(addr: Noun, noun: &Noun) -> Noun {
  let mut address = *addr.unwrap_atom().unwrap();
  let mut path: Vec<u8> = vec![];
  while address > 1 {
    if address % 2 == 0 {
      address = address / 2;
      path.push(0);
    } else {
      address = (address - 1) / 2;
      path.push(1);
    }
  }
  let mut current_cell = noun;
  for i in path.iter().rev() {
    if *i == 0 {
      current_cell = &current_cell.unwrap_cell().unwrap()[0];
    } else {
      current_cell = &current_cell.unwrap_cell().unwrap()[1];
    };
  }
  return current_cell.clone();
}

// #
// fn hax()

// *
fn tar(noun: &Noun) -> Noun {
  let unwrapped_noun = &noun.unwrap_cell().unwrap();
  let subject = &unwrapped_noun[0];
  let formula = &unwrapped_noun[1];
  let formula_v = formula.unwrap_cell().unwrap();
  let operation = &formula_v[0];
  match operation {
    Atom(a) => match a {
      0 => fas(formula_v[1].clone(), &subject),
      1 => formula_v[1].clone(),
      2 => tar(&Cell(vec![
        tar(&Cell(vec![
          subject.clone(),
          formula_v[1].unwrap_cell().unwrap()[0].clone(),
        ])),
        tar(&Cell(vec![
          subject.clone(),
          formula_v[1].unwrap_cell().unwrap()[1].clone(),
        ])),
      ])),
      3 => wut(tar(&Cell(vec![subject.clone(), formula_v[1].clone()]))),
      4 => lus(tar(&Cell(vec![subject.clone(), formula_v[1].clone()]))),
      5 => tis(Cell(vec![
        tar(&Cell(vec![
          subject.clone(),
          formula_v[1].unwrap_cell().unwrap()[0].clone(),
        ])),
        tar(&Cell(vec![
          subject.clone(),
          formula_v[1].unwrap_cell().unwrap()[1].clone(),
        ])),
      ])),
      6 => tar(&Cell(vec![
        subject.clone(),
        Cell(vec![
          Atom(2),
          Cell(vec![Atom(0), Atom(1)]),
          Cell(vec![
            Atom(2),
            Cell(vec![Cell(vec![
              Atom(1),
              formula_v[1].unwrap_cell().unwrap()[1]
                .unwrap_cell()
                .unwrap()[0]
                .clone(),
              formula_v[1].unwrap_cell().unwrap()[1]
                .unwrap_cell()
                .unwrap()[1]
                .clone(),
            ])]),
            Cell(vec![
              Cell(vec![Atom(0), Atom(1)]),
              Cell(vec![
                Cell(vec![Atom(1), Cell(vec![Atom(2), Atom(3)])]),
                Cell(vec![
                  Cell(vec![Atom(1), Atom(0)]),
                  Atom(4),
                  Cell(vec![
                    Atom(4),
                    formula_v[1].unwrap_cell().unwrap()[0].clone(),
                  ]),
                ]),
              ]),
            ]),
          ]),
        ]),
      ]))
      .clone(),
      7 => tar(&Cell(vec![
        subject.clone(),
        Cell(vec![
          Atom(2),
          Cell(vec![
            formula_v[1].unwrap_cell().unwrap()[0].clone(),
            Cell(vec![
              Atom(1),
              subject.clone(),
              formula_v[1].unwrap_cell().unwrap()[1].clone(),
            ]),
          ]),
        ]),
      ]))
      .clone(),
      8 => tar(&Cell(vec![
        Cell(vec![
          tar(&Cell(vec![
            subject.clone(),
            formula_v[1].unwrap_cell().unwrap()[0].clone(),
          ])),
          subject.clone(),
        ]),
        formula_v[1].unwrap_cell().unwrap()[1].clone(),
      ]))
      .clone(),
      9 => tar(&Cell(vec![
        tar(&Cell(vec![
          subject.clone(),
          formula_v[1].unwrap_cell().unwrap()[1].clone(),
        ])),
        Cell(vec![
          Atom(2),
          Cell(vec![
            Cell(vec![Atom(0), Atom(1)]),
            Atom(0),
            formula_v[1].unwrap_cell().unwrap()[0].clone(),
          ]),
        ]),
      ]))
      .clone(),
      // 11 => {

      // }
      _ => panic! {"invalid operation"},
    },
    Cell(v) => Cell(vec![
      tar(&Cell(vec![subject.clone(), formula_v[0].clone()])),
      tar(&Cell(vec![subject.clone(), formula_v[1].clone()])),
    ]),
  }
}

fn deeply_equal(a: &Noun, b: &Noun) -> bool {
  match a {
    Atom(num_a) => match b {
      Atom(num_b) => return num_a == num_b,
      Cell(_) => return false,
    },
    Cell(vec_a) => match b {
      Atom(_) => return false,
      Cell(vec_b) => {
        if !deeply_equal(&vec_a[0], &vec_b[0]) {
          return false;
        };
        if !deeply_equal(&vec_a[1], &vec_b[1]) {
          return false;
        };
        return true;
      }
    },
  }
}