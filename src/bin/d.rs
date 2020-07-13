#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        p: [f64; n]
    }
    let mut a = vec![0.0; n];
    for i in 0..n {
        a[i] = (p[i] + 1.0) / 2.0;
    }
    let mut sum = a.iter().take(k).sum::<f64>();
    let mut ans = sum;
    for j in k..n {
        sum += a[j] - a[j - k];
        if ans < sum {
            ans = sum;
        }
    }
    println!("{}", ans)
}
