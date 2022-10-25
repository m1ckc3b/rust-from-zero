// Les types génériques => factoriser le code
// une sorte de placeholder remplaçable par n'importe quel type.
// utilisable dans : struct, enum, param de fonctions, retour de fonction.
// pas de coût supplémentaire au runtime


// Syntaxe:
// enum Result<T, E> {
//   Ok(T),
//   Err(E),
// }

// fn ouvrir(filename: &str) -> Result<String, String>
// fn display<T: Display>(contenu: T) // where T: Display


// Monomorphisation: 
// n'inclut pas de coûts de performance lors de l'utilisation de génériques.
// A la compilation, Rust adapte le code selon le type passé.


// Implémentation
use std::fmt::Display;

fn main() {
  let max_usize: usize = max(gauche: 5usize, droite: 19usize);
  dbg!(max_usize);

  let max_f64: usize = max(gauche: 5f64, droite: 19f64);
  dbg!(max_usize);
}

// générique T avec contraintes de comparaison et d'affichage
fn max<T: PartialOrd + Display>(gauche: T, droite: T) -> T {
  // fn max<T>(gauche: T, droite: T) -> T 
  // where 
  //  T: PartialOrd + Display, {
  if gauche >= droite {
    gauche
  } else {
    droite
  }
}