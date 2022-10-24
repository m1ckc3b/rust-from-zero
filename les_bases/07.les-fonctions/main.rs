fn main() {
  let somme = additionne(5, 20);
  let division = divise(5, 0);

  println!("double tuple -- {:?}", double_tuple((5, 225)));
  // {:?} = affichage en mode debug, pour les structures de données plus compliquées.

  let reponse = execute(fonction: additionne); // passe la fonction additionne en paramètre.
  dbg!(reponse); // équivalent aux println pour débuger.
}

fn additionne(left: i32, right: i32) -> i32 {
  // Tail expression (renvoie implicite)
  left + right // sans point-virgule à la fin!
}

fn divise(left: usize, right: usize) -> f64 {
  if right == 0 {
    return 0.0;
  }

  // Tail expression (renvoie implicite)
  left as f64 / right as f64
}

fn double_tuple(val: (usize, i32)) -> (usize, i32) {
  // Tail expression (renvoie implicite)
  (val.0 *2, val.1 * 2)
}

fn execute(fonction: fn(i32, i32) -> i32) -> i32 {
  fonction(1, 3)
}