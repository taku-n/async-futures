use futures::channel::oneshot;
use futures::prelude::*;

fn main() {
    let (tx, rx) = oneshot::channel<dyn Future<Output = _>>();
}
