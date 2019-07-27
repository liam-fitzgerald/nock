#[cfg(test)]
mod tests {

  use super::super::nock::*;

  #[test]
  fn it_works() {
    assert!(1 == 1)
  }

  #[test]
  fn op_0_1() {
    let input1 = "[[[4 5] [6 14 15]] [0 7]]";
    let product = parse_and_nock(input1.to_string());
    assert_eq!(format!("{}", product), "[ 14  15 ]");
    let input1 = "[[[4 5] [6 14 15]] [0 2]]";
    let product = parse_and_nock(input1.to_string());
    assert_eq!(format!("{}", product), "[ 4  5 ]");
  }
}
