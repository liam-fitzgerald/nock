#[cfg(test)]
mod tests {

  use super::super::nock::*;

  #[test]
  fn it_works() {
    assert!(1 == 1)
  }


  #[test]
  fn op_0() {
    let input = "[57 [0 1]]";
    let product = parse_and_nock(input.to_string());
    assert_eq!(format!("{}", product), "57");
    let input = "[[132 19] [0 3]]";
    let product = parse_and_nock(input.to_string());
    assert_eq!(format!("{}", product), "19");
    let input = "[[[4 5] [6 14 15]] [0 7]]";
    let product = parse_and_nock(input.to_string());
    assert_eq!(format!("{}", product), "[ 14  15 ]");
    let input = "[[[4 5] [6 14 15]] [0 2]]";
    let product = parse_and_nock(input.to_string());
    assert_eq!(format!("{}", product), "[ 4  5 ]");
  }

  #[test]
  fn op_1() {
    let input = "[42 [1 153 218]]";
    let product = parse_and_nock(input.to_string());
    assert_eq!(format!("{}", product), "[ 153  218 ]");
    let input = "[42 [1 153]]";
    let product = parse_and_nock(input.to_string());
    assert_eq!(format!("{}", product), "153");
  }

  #[test]
  fn op_2() {
    let input = "[77 [2 [1 42] [1 1 153 218]]]";
    let product = parse_and_nock(input.to_string());
    assert_eq!(format!("{}", product), "[ 153  218 ]");
  }

  #[test]
  fn op_3() {
    let input = "[42 [3 0 1]]";
    let product = parse_and_nock(input.to_string());
    assert_eq!(format!("{}", product), "1");
    let input = "[42 [[4 0 1] [3 0 1]]]";
    let product = parse_and_nock(input.to_string());
    assert_eq!(format!("{}", product), "[ 43  1 ]");
  }

  #[test]
  fn op_4() {
    let input = "[42 4 0 1]";
    let product = parse_and_nock(input.to_string());
    assert_eq!(format!("{}", product), "43");
    let input = "[42 [4 0 1]]";
    let product = parse_and_nock(input.to_string());
    assert_eq!(format!("{}", product), "43");
    let input = "[57 [4 0 1]]";
    let product = parse_and_nock(input.to_string());
    assert_eq!(format!("{}", product), "58");
    let input = "[[132 19] [4 0 3]]";
    let product = parse_and_nock(input.to_string());
    assert_eq!(format!("{}", product), "20");
  }

  // TODO: 5

  #[test]
  fn op_6() {
    let input = "[42 [6 [1 0] [4 0 1] [1 233]]]";
    let product = parse_and_nock(input.to_string());
    assert_eq!(format!("{}", product), "43");
    let input = "[42 [6 [1 1] [4 0 1] [1 233]]]";
    let product = parse_and_nock(input.to_string());
    assert_eq!(format!("{}", product), "233");
  }

  #[test]
  fn op_7() {
    let input = "[42 [7 [4 0 1] [4 0 1]]]";
    let product = parse_and_nock(input.to_string());
    assert_eq!(format!("{}", product), "44");
  }

  #[test]
  fn op_8() {
    let input = "[42 [8 [4 0 1] [0 1]]]";
    let product = parse_and_nock(input.to_string());
    assert_eq!(format!("{}", product), "[ 43  42 ]");
    let input = "[42 [8 [4 0 1] [4 0 3]]]";
    let product = parse_and_nock(input.to_string());
    assert_eq!(format!("{}", product), "43");
  }

  // TODO: 9
  // TODO: 10

  #[test]
  fn op_11() {
    let input = "[[132 19] [11 37 [4 0 3]]]";
    let product = parse_and_nock(input.to_string());
    assert_eq!(format!("{}", product), "20");
    let input = "[[132 19] [11 [1 1 1] [4 0 3]]]";
    let product = parse_and_nock(input.to_string());
    assert_eq!(format!("{}", product), "20");
  }

}
