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

struct Bracelets {
  b:u64
  , n:usize
  , k:usize
  , t:usize
  , p:usize
}

impl Iterator for Bracelets {
  type Item = u64;

  fn next(&mut self) -> Option<u64> {

    if self.t > self.n {
      if self.n % self.p == 0 {
        Some(self.b);
      }
    } else {
      let x = get(self.b, self.t - self.p);
      set(self.b, self.t, self.x);
      //???genneck(b, n, k, t + 1, p);
      if x == 0 {
        set(self.b, self.t, 1);
        //??genneck(self.b, self.n, self.k, self.t + 1, self.t);
      }
    }
  }
}

fn bracelets(n:usize, k:usize) -> Bracelets {
  Bracelets {b:0, n:n, k:k, t:1, p:1}
}


fn main() {
  for i in bracelets() {
    print(i, n);
  }
}





