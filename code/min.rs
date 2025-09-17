
// struct Boxed(i32);

// impl Boxed {
//   fn inc(&mut self) -> () {
//     self.0 += 1;
//   }
//   fn consume(self) -> () {
//     ()
//   }
//   fn inc_immutable(self) -> Self {
//     Boxed(self.0 + 1)
//   }
// }

// fn main() {
//   let x = Boxed(5);
//   let mut x = x.inc_immutable();
//   x.inc();
//   x.consume();
// }

use vstd::prelude::*;

verus! {

spec fn min(x: int, y: int) -> int {
    if x <= y {
        x
    } else {
        y
    }
}

proof fn min_chain(x: int, y: int, z: int)
    requires 
        min(x, y) == x,
        min(y, z) == y,
    ensures
        min(x, z) == x
{
}

exec fn main() {
    assert(min(10, 20) == 10);
    assert(min(-10, -20) == -20);
    assert(forall|a: int, b: int| min(a, b) <= a && min(a, b) <= b);
}

} // verus!
