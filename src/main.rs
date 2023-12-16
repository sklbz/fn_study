mod generate_fn;
use generate_fn::generate_fn;
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


fn main() {
    let mut rng = rand::thread_rng();
    let dist = Filter {
        dist: Uniform::new_inclusive(-10, 10),
        test: |x: &_| (x != &0) & (x != &2),
    };


    generate_fn(rng.sample(&dist),rng.sample(&dist),rng.sample(&dist),rng.sample(&dist),rng.sample(&dist));
}