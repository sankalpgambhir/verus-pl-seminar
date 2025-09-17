
use vstd::prelude::*;

verus! {

spec fn fib(n: nat) -> nat
    decreases n
{
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fib((n - 2) as nat) + fib((n - 1) as nat)
    }
}

proof fn lemma_fib_is_monotonic(i: nat, j: nat)
    requires
        i <= j,
    ensures
        fib(i) <= fib(j),
    decreases j - i,
{
    if j < 2 {
    } else if i == j {
    } else if i == j - 1 {
    } else {
        lemma_fib_is_monotonic(i, (j - 1) as nat);
        lemma_fib_is_monotonic(i, (j - 2) as nat);
    }
}

spec fn fib_bounded(n: nat) -> bool {
    fib(n) <= u64::MAX
}

exec fn fib_impl(n: u64) -> (result: u64)
    requires
        fib_bounded(n as nat),
    ensures
        result == fib(n as nat),
{
    if n == 0 {
        return 0;
    }
    let mut prev: u64 = 0;
    let mut cur: u64 = 1;
    let mut i: u64 = 1;
    while i < n
        invariant
            0 < i <= n,
            cur == fib(i as nat),
            prev == fib((i - 1) as nat),
            fib_bounded(n as nat),
            fib_bounded(i as nat),
        decreases n - i
    {
        i = i + 1;
        proof {
            lemma_fib_is_monotonic(i as nat, n as nat);
        }
        let new_cur = cur + prev;
        prev = cur;
        cur = new_cur;
    }
    cur
}

fn main() {

}

} // verus!