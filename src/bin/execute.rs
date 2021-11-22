use futures::executor::block_on;
use futures::prelude::*;

fn main() {
    let future = async move {
        2
    };
    let result = block_on(future);
    dbg!(result);
}
