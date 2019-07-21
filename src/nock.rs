// type Atom = u32;
// type Cell = Vec<Noun>;

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
  let noun = Noun::new();

  let mut n = 0;
  let mut atom: u32 = 0;
  let mut current_cell = &noun;

  for c in input.chars() {
    &noun.push(Noun::Atom(32));
  }

  return noun;
}

fn nock(noun: Noun) {
  println!("{:?}", noun)
}

