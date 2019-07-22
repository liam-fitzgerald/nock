#[derive(Debug)]
enum Noun {
  Atom(u32),
  Cell(Vec<Noun>),
}

impl Noun {
  fn new() -> Noun {
    Noun::Cell(vec![])
  }

  fn push(&self, noun: Noun) {
    match noun {
      Noun::Atom(int) => println!("This is an atom"),
      Noun::Cell(vec) => println!("This is a cell. {:?}", self),
    }
  }
}

pub fn main(input: String) {
  println!("{}", input);
  println!("{:?}", vec![1, 2, 3]);
  parse(input);
}

fn parse(input: String) -> Noun {
  // let noun = Noun::new();

  // let len: usize = input.len();
  // let mut i: usize = 0;
  let mut atom: u32 = 0;
  // let mut current_cell = &noun;

  let mut iter = input.chars();

  fn parse_recursive(mut iter: &mut std::str::Chars<'_>) -> Noun {
    let cell = Noun::new();
    let char = iter.next();
    match char {
      None => return cell,
      Some(c) => println!("{}", c),
    };
    cell
  }

  // if iter.next() == Some('[') {
  //   let noun = parse_recursive(iter);
  // };
  let noun = parse_recursive(&mut iter);

  let c = iter.next();
  println!("{:?}", c);

  return noun;
}

fn nock(noun: Noun) {
  println!("{:?}", noun)
}

