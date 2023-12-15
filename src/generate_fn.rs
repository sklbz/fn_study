mod rand;
use rand::Rng;

pub fn generate_fn() {
  let mut rng = rand::thread_rng();
  let mut a: i8 = rng.gen_range(-5..5);
  print!("hello");
}

fn initialize_value() {

}