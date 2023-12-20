use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
//use tailcall::tailcall;
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

fn compare_fract(value: &f32) -> bool {
  let decimal: f32 = value.fract();
  return decimal != 0.0 && decimal != 0.1 && decimal != 0.2 && decimal != 0.25 && decimal != 0.3 && decimal != 0.4 && decimal != 0.5 && decimal != 0.6 && decimal != 0.7 && decimal != 0.75 && decimal != 0.8 && decimal != 0.9;
}


pub fn generate_fn() {
  // ?#[tailcall]
  fn generate_fn_inner(iteration_count: u32) -> u64 {
    let max_iterations: u32 = 5000;

    let current_iteration: u32 = iteration_count + 1;
    if current_iteration > max_iterations {
      write_stack_overflow();
      panic!("too many iterations");
    }

    let mut rng = rand::thread_rng();
    let square_filter = Filter {
      dist: Uniform::new_inclusive(-10, 10),
      test: |x: &_| (x != &0),
    };
    

    let first_coefficient: i32 = rng.sample(&square_filter); 
    let second_numerator_coefficient: i32 = rng.sample(&square_filter);
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
      generate_fn_inner(current_iteration);
      return 1;
    }

    let third_coefficient: i32 = third_derivative_coefficient / b_minus_d; // calculate c
    let fourth_coefficient: i32 = second_numerator_coefficient - b_minus_d; // calculate d
    let fifth_coefficient: i32 = e_minus_c + third_coefficient; // calculate e

    // * creating a function
    let function = Function {
      a: first_coefficient,
      b: second_numerator_coefficient,
      c: third_coefficient,
      d: fourth_coefficient,
      e: fifth_coefficient,
    };

    // * Testing with a few conditions

    // * High value testing
    if third_coefficient.abs() > 15 || fourth_coefficient.abs() > 15  || fifth_coefficient.abs() > 15 {
      generate_fn_inner(current_iteration);
      return 1;
    }
    // * Derivative high value testing
    let second_derivative_coefficient: i32 = 2 * first_coefficient * e_minus_c;
    let third_derivative_coefficient: i32 = second_numerator_coefficient * fifth_coefficient - fourth_coefficient * third_coefficient;
    if second_derivative_coefficient.abs() > 20 || third_derivative_coefficient.abs() > 20 {
      generate_fn_inner(current_iteration);
      return 1;
    }
    /* --- */

    // * Asymptote testing
    let denominator_discriminant: i32 = fourth_coefficient * fourth_coefficient - 4 * first_coefficient * fifth_coefficient;
    if denominator_discriminant >= 0 {
      generate_fn_inner(current_iteration);
      return 1;
    }
    /* --- */

    // * Extremum testing
    let first_extremum: f32 = calculate_function(&function, first_derivative_root);
    let second_extremum: f32 = calculate_function(&function, second_derivative_root);
    let difference: f32 = first_extremum.abs() - second_extremum.abs();

    if compare_fract(&first_extremum) || first_extremum.abs() > 10.0
    || compare_fract(&second_extremum) || second_extremum.abs() > 10.0
    || difference.abs() < 3.0 {
      generate_fn_inner(current_iteration);
      return 1;
    };
    /* --- */
    
    println!("{current_iteration}");
    println!("a:{first_coefficient}");
    println!("b:{second_numerator_coefficient}");
    println!("c:{third_coefficient}");
    println!("d:{fourth_coefficient}"); 
    println!("e:{fifth_coefficient}");
    println!("x1:{first_derivative_root}");
    println!("f(x1):{first_extremum}");
    println!("x2:{second_derivative_root}");
    println!("f(x2):{second_extremum}");
    println!("Δ:{denominator_discriminant}");

    write_data_to_file(current_iteration);

    return 0;
  }

  generate_fn_inner(0);
}

fn write_data_to_file(data: u32) {
  let data: String = format!("{data}\n");

  let mut f: File = OpenOptions::new()
        .write(true)
        .append(true)
        .open("/home/sklbz/maths/fn_study/statistics/iterations.txt")
        .unwrap();


  f.write(data.as_bytes()).expect("Unable to write data");
}

fn write_stack_overflow() {
  let data: String = format!("STACK OVERFLOW\n");

  let mut f: File = OpenOptions::new()
        .write(true)
        .append(true)
        .open("/home/sklbz/maths/fn_study/statistics/iterations.txt")
        .unwrap();


  f.write(data.as_bytes()).expect("Unable to write data"); 
}
//TODO: Add tailcall optimisation