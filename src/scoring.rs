

pub fn move_score(size : i32, ls : i32, ls_old : i32) -> i32 {
    let points = size + 100 * (1 + ls) * ls / 2;
    let line_bonus = if ls_old > 1 { ((ls_old - 1) * points) / 10 }
                     else          { 0 };
    points + line_bonus
}


#[test]
fn move_score_test() {
   assert!(move_score(0, 0, 0) == 0);
   assert!(move_score(5, 2, 0) == 305);
   assert!(move_score(5, 2, 2) == 335);
}
