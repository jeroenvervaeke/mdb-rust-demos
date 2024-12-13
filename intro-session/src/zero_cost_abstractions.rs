// Average 2.56 ns/op +/- 0.14
pub fn factorial_loop(num: usize) -> usize {
    let mut acc = 1;
    for i in 1..num {
        acc = acc * i
    }
    return acc;
}

// Average 2.56 ns/op +/- 0.19
pub fn factorial_iterator(num: usize) -> usize {
    (1..num).fold(1, |acc, i| acc * i)
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use test::{black_box, Bencher};

    const NUM: usize = 10;

    #[bench]
    fn bench_loop(b: &mut Bencher) {
        b.iter(|| factorial_loop(black_box(NUM)));
    }

    #[bench]
    fn bench_iterator(b: &mut Bencher) {
        b.iter(|| factorial_iterator(black_box(NUM)));
    }
}
