
fn main() {
  // while
  let mut i = 0i32;

  while i < 10 {
    println!("Bonjour {i} !");
    i += 1;
  }

  // loop { ... } -> boucle infinie
  loop {
    println!("bonjour !");
    i += 1;
    if i > 10 {
        break; // -> quitte la boucle
        // return -> quitte la fonction
    }
  }

  // for
  for i in 0..10 {
    println!("i vaut : {}", i);
  }

  // Enumération
  for (i, j) in (5..10).enumerate() {
    println!("i = {} et j = {}", i, j);
  }

  // Boucles nommées
  'global: for _ in 0..10 {
    'outer: for x in 0..10 {
        'inner: for y in 0..10 {
            if x > 3 { break 'global; } // on arrête la boucle qui s'appelle global
            if x % 2 == 0 { continue 'outer; } // on continue la boucle sur x
            if y % 2 == 0 { continue 'inner; } // on continue la boucle sur y
            println!("x: {}, y: {}", x, y);
        }
    }
  }
}