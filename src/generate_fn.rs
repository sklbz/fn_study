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


pub fn generate_fn() {

  let mut rng = rand::thread_rng();
    let square_filter = Filter {
        dist: Uniform::new_inclusive(-10, 10),
        test: |x: &_| (x != &0),
    };

    let first_coefficient: i32 = rng::sample(&square_filter); 
    let second_coefficient: i32;
    let first_derivative_root: i32;
    let second_derivative_root:i32;
    let first_derivative_coefficient: i32;

  let sum_derivative_roots: i32 = first_derivative_root + second_derivative_root;
  let product_derivative_roots: i32 = first_derivative_root * second_derivative_root;

  let second_derivative_coefficient: i32 = -first_derivative_coefficient * product_derivative_roots; // calculate b'
  let third_derivative_coefficient: i32 = first_derivative_coefficient * sum_derivative_roots; // calculate c'

  let b_minus_d: i32 = -first_derivative_coefficient / first_coefficient;
  let e_minus_c: i32 = second_derivative_coefficient / ( 2 * first_coefficient );

  let third_coefficient: i32 = third_derivative_coefficient / b_minus_d; // calculate c
  let fifth_coefficient: i32 = e_minus_c + third_coefficient;

  let fourth_coefficient: i32 = second_coefficient - b_minus_d;



  println!("a:{first_coefficient}");
  println!("b:{second_coefficient}");
  println!("c:{third_coefficient}");
  println!("d:{fourth_coefficient}"); 
  println!("e:{fifth_coefficient}");
}