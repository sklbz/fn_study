use rand::{
  self,
  distributions::{Distribution, Uniform},
  Rng,
};

struct Filter<Dist, Test> {
  dist: Dist,
  test: Test,
}

impl<T, Dist, Test> Distribution<T> for Filter<Dist, Test>
where
  Dist: Distribution<T>,
  Test: Fn(&T) -> bool,
{
  fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> T {
      loop {
          let x = self.dist.sample(rng);
          if (self.test)(&x) {
              return x;
          }
      }
  }
}

struct Function<I32> {
  a: I32,
  b: I32,
  c: I32,
  d: I32,
  e: I32,
}

fn calculate_function(parameters: &Function<i32>, value: i32) -> f32 {
  let numerator: i32 = value * value * parameters.a + value * parameters.b + parameters.c;
  let denominator: i32 = value * value * parameters.a + value * parameters.d + parameters.e;
  return numerator as f32 / denominator as f32;
}


pub fn generate_fn(iteration_count: u32) {

  let current_iteration: u32 = iteration_count + 1;
  if current_iteration > 10000 {
    println!("too many iterations");
    return;
  }

  let mut rng = rand::thread_rng();
  let square_filter = Filter {
    dist: Uniform::new_inclusive(-10, 10),
    test: |x: &_| (x != &0),
  };
  

  let first_coefficient: i32 = rng.sample(&square_filter); 
  let second_coefficient: i32 = rng.sample(&square_filter);
  let first_derivative_coefficient: i32 = rng.sample(&square_filter);
  let first_derivative_root: i32 = rng.sample(&square_filter);

  let root_filter = Filter {
    dist: Uniform::new_inclusive(-10, 10),
    test: |x: &_| (x != &first_derivative_root),
  };

  let second_derivative_root:i32 = rng.sample(&root_filter);
  

  let sum_derivative_roots: i32 = first_derivative_root + second_derivative_root;
  let product_derivative_roots: i32 = first_derivative_root * second_derivative_root;

  let second_derivative_coefficient: i32 = -first_derivative_coefficient * product_derivative_roots; // calculate b'
  let third_derivative_coefficient: i32 = first_derivative_coefficient * sum_derivative_roots; // calculate c'

  let b_minus_d: i32 = -first_derivative_coefficient / first_coefficient;
  let e_minus_c: i32 = second_derivative_coefficient / ( 2 * first_coefficient );

  if b_minus_d == 0 {
    generate_fn(current_iteration);
    return;
  }

  let third_coefficient: i32 = third_derivative_coefficient / b_minus_d; // calculate c
  let fourth_coefficient: i32 = second_coefficient - b_minus_d; // calculate d
  let fifth_coefficient: i32 = e_minus_c + third_coefficient; // calculate e

  // * creating a function
  let function = Function {
    a: first_coefficient,
    b: second_coefficient,
    c: third_coefficient,
    d: fourth_coefficient,
    e: fifth_coefficient,
  };

  // * Testing with a few conditions

  // * Huge value testing
  if third_coefficient.abs() > 15 || fourth_coefficient.abs() > 15  || fifth_coefficient.abs() > 15 {
    generate_fn(iteration_count);
    return;
  }
  /* --- */

  // * Asymptote testing
  let denominator_discriminant: i32 = fourth_coefficient * fourth_coefficient - 4 * first_coefficient * fifth_coefficient;
  if denominator_discriminant >= 0 {
    generate_fn(iteration_count);
    return;
  }

  // * Extremum testing
  let first_extremum: f32 = calculate_function(&function, first_derivative_root);
  let second_extremum: f32 = calculate_function(&function, second_derivative_root);
  let difference: f32 = first_extremum.abs() - second_extremum.abs();

  if first_extremum.fract() == 0.0 || first_extremum.abs() > 10.0
  || second_extremum.fract() == 0.0 || second_extremum.abs() > 10.0
  || difference.abs() < 3.0 {
    generate_fn(current_iteration);
    return;
  };
  /* --- */
  
  println!("a:{first_coefficient}");
  println!("b:{second_coefficient}");
  println!("c:{third_coefficient}");
  println!("d:{fourth_coefficient}"); 
  println!("e:{fifth_coefficient}");
}

