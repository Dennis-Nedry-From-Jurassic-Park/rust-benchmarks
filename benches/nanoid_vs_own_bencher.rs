#[macro_use]
extern crate bencher;

use bencher::Bencher;
use nanoid::nanoid;

fn a(bench: &mut Bencher) {
    const N: usize = 1024;
    bench.iter(|| {
        nanoid!()
    });
    bench.bytes = N as u64;
}
fn b(bench: &mut Bencher) {
    const N: usize = 1024;
    bench.iter(|| {
        nanoid!(25);
    });
    bench.bytes = N as u64;
}

benchmark_group!(benches, a, b);
benchmark_main!(benches);