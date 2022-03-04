use std::env;

fn main() {
    let scene = env::args().nth(1);

    beverage::run(Default::default());
}
