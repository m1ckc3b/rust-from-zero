fn main() {
  // les 3 types de boucles
  let tableau: [i32; 4] = [0, 1, 2, 3];

  // boucle for in avec range
  for index in 0..=3 { // range incluant le 3 (0..3)
    println!(
      "boucle avec range, element index {index} = {}",
      tableau[index]
    );
  }


  // boucle loop
  let mut index = 0;
  loop {
    if index >= tableau.len() {
      break;
    }
    println!("element avec boucle loop, {index} = {}", tableau[index]);

    index +=1; // index++ n'existe pas !
  }

  // boucle while
  let mut index = 0;
  while index < tableau.len() {
      println!("element avec boucle while, {index} = {}", tableau[index]);
      index += 1;
  }
}