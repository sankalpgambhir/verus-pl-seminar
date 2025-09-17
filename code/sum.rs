
use vstd::prelude::*;

verus! {

spec fn sum_direct(n: int) -> int {
    (n * (n + 1)) / 2
}

spec fn sum(n: int) -> (res: int) 
    decreases n
{
    if n <= 0 {
        0
    } else {
        n + sum((n - 1) as int)
    }
}

proof fn sum_direct_rec1(n: int)
    ensures
        (n * (n + 1)) == (((n * (n - 1))) + (2*n)),
{
    assert((n * (n + 1)) == ((n * n) + n)) by(nonlinear_arith);
    assert((n * (n - 1)) + (2*n) == ((n * n) - n + (2*n))) by(nonlinear_arith);
    assert(((n * n) - n + (2*n)) == ((n * n) + n)) by(nonlinear_arith);
}

proof fn sum_direct_rec(n: int)
    requires 
        n > 0
    ensures
        sum_direct(n) == (sum_direct((n - 1) as int) + n),
{
    assert((sum_direct(n) == (sum_direct((n - 1) as int) + n))) by(nonlinear_arith);
}

proof fn sum_equiv(n: int) 
    requires n >= 0
    ensures sum(n) == sum_direct(n)
    decreases n
{
    if n == 0 {
        // base case
    } else {
        sum_direct_rec(n);
        sum_equiv((n - 1) as int);
    }
}

proof fn sum_monotonic(n1: int, n2: int) 
    requires n1 <= n2 && n1 >= 0
    ensures sum(n1) <= sum(n2)
    decreases(n2 - n1)
{
    if n1 == n2 {
        // base case
    } else {
        sum_monotonic(n1, n2 - 1);
    }
}

exec fn while_sum(n: u32) -> (res: u32) 
    requires
        sum(n as int) <= u32::MAX,
    ensures
        res == sum(n as int - 1),
{
    let mut curr = 0;
    let mut i = 0;
    while i < n
        invariant
            0 <= i <= n,
            sum(n as int) <= u32::MAX,
            sum(i as int - 1) <= u32::MAX,
            curr == sum(i as int - 1),
        decreases n - i
    {
        proof {
            sum_monotonic(i as int, n as int);
            sum_direct_rec(i as int + 1);
        }
        curr += i;
        i = i + 1;
    }
    
    curr
}

fn main() {
}

}
