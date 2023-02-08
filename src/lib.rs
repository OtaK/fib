#[tailcall::tailcall]
fn fib_rec_inner(acc: usize, n: usize) -> usize {
    if n > 1 {
        fib_rec_inner(acc, n - 1)
    } else {
        acc
    }
}

pub fn fib_rec(n: usize) -> usize {
    fib_rec_inner(0, n)
    //match n {
    //    0 | 1 => n,
    //    n => fib_rec(n - 1) + fib_rec(n - 2),
    //}
}

pub fn fib_linear(n: usize) -> usize {
    let mut s = 0;
    let mut l = 0;
    let mut c = 1;
    for _ in 1..n {
        s = l + c;
        l = c;
        c = s;
    }
  
    s
}

