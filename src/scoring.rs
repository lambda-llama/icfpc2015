

pub fn move_score(size : i32, ls : i32, ls_old : i32) -> i32 {
    let points = size + 100 * (1 + ls) * ls / 2;
    let line_bonus = if ls_old > 1 { ((ls_old - 1) * points) / 10 }
                     else          { 0 };
    points + line_bonus
}

pub fn power_score(len : i32, reps : i32) -> i32 {
    let power_bonus = if reps > 0 { 300 }
                      else        { 0 };
    2 * len + reps + power_bonus
}

#[test]
fn move_score_test() {
   assert!(move_score(0, 0, 0) == 0);
   assert!(move_score(5, 2, 0) == 305);
   assert!(move_score(5, 2, 2) == 335);
}

#[test]
fn power_score_test() {
    assert!(power_score(10, 0) == 20);
    assert!(power_score(15, 2) == 332);
}
