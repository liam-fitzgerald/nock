#[cfg(test)]
mod tests {

  use super::super::nock::*;

  #[test]
  fn it_works() {
    assert!(1 == 1)
  }

  #[test]
  fn op_0() {
    let input1 = "[[[4 5] [6 14 15]] [0 7]]";
    let product = parse_and_nock(input1.to_string());
    assert_eq!(format!("{}", product), "[ 14  15 ]");
    let input1 = "[[[4 5] [6 14 15]] [0 2]]";
    let product = parse_and_nock(input1.to_string());
    assert_eq!(format!("{}", product), "[ 4  5 ]");
  }

  #[test]
  fn op_1() {
    let input1 = "[42 [1 153 218]]";
    let product = parse_and_nock(input1.to_string());
    assert_eq!(format!("{}", product), "[ 153  218 ]");
    let input1 = "[42 [1 153]]";
    let product = parse_and_nock(input1.to_string());
    assert_eq!(format!("{}", product), "153");
  }


  // let input1 = "[42 [1 153 218]]"; // 1   [153 218]
  // let input1 = "[77 [2 [1 42] [1 1 153 218]]]"; // 2   [153 218]

  // let input1 = "[57 [0 1]]"; // 57
  // let input1 = "[[132 19] [0 3]]"; // 19
  // let input1 = "[57 [4 0 1]]"; // 58
  // let input1 = "[[132 19] [4 0 3]]"; // 20

  // let input1 = "[42 [3 0 1]]"; // 1
  // let input1 = "[42 [4 0 1]]"; // 43
  // let input1 = "[42 [[4 0 1] [3 0 1]]]"; // [43, 1]

  // let input2 = "[[[12 13] 14] 15]";
  // let input3 = "[[12 13] [62 38 97]]";
}
