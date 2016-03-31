fn get(b:u64, i:usize) -> usize {
  if (b >> i) & 1u64 == 0 {
    return 0;
  } else {
    return 1
  }
}

fn set(b:&mut u64, i:usize, val:usize) {
  if val == 0 {
    *b &= !(1u64 << i);
  } else {
    *b |= 1u64 << i;
  }
}

fn print(x:u64, n:usize) {
  println!("{x:0>0$b}", n as usize, x = x >> 1);
}

fn genneck(b:&mut u64, n:usize, k:usize, t:usize, p:usize) {
  if t > n {
    if n % p == 0 {
      print(*b, n);
    }
  } else {
    let x = get(*b, t - p);
    set(b, t, x);
    genneck(b, n, k, t + 1, p);
    if x == 0 {
      set(b, t, 1);
      genneck(b, n, k, t + 1, t);
    }
  }
}

fn main() {
  let mut b:u64 = 0;
  let n:usize = 16;
  let k:usize = 2;

  genneck(&mut b, n, k, 1, 1);
}
