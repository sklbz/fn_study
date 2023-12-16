pub fn generate_fn(first_coefficient: i32, first_derivative_root: i32, second_derivative_root:i32, /* first_value: i32, last_value: i32, */ first_extremum: i32, second_extremum: i32, first_derivative_coefficient: i32) {
  let sum_derivative_roots: i32 = first_derivative_root + second_derivative_root;
  let product_derivative_roots: i32 = first_derivative_root * second_derivative_root;

  let second_derivative_coefficient: i32 = -first_derivative_coefficient * product_derivative_roots; // calculate b'
  let third_derivative_coefficient: i32 = first_derivative_coefficient;
  let mut _b: i32;
  let mut _c: i32;
  let mut _d: i32;
  let mut _e: i32;
  //print!("{first_coefficient}\n{first_derivative_root}\n{second_derivative_root}\n{first_value}\n{last_value}\n{first_extremum}\n{second_extremum}\n");
}