mod nock;

fn main() {
  // let noun = nock::Noun::Atom(93);
  // nock::nock(noun);

  let input= "some string!";

  nock::main(input.to_string());
}
