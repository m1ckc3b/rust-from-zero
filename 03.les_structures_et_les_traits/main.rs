
// Déclarer et implémenter une structure (équivalent à une classe)
// Struct Data :
// struct Rectangle {
//   hauteur: u64,
//   longueur: u64,
// }

// impl Rectangle {
//   fn new(h: u64, l: u64) -> Self {
//     Self { // Self = Rectangle
//       hauteur: h,
//       largeur: l,
//     }
//   }
//   pub fn aire(&self) -> u64 { // pub = publique
//     self hauteur * self largeur // self = this => l'instance
//   }
// }


// Les traits = interfaces
// partage un comportement (méthodes) entre diff structs.
// possible d'implémenter un trait sur un type qui ne nous appartient pas.
// trait Aire {
//   fn aire(&self) -> u64;
// }


// Implémentation:
struct Rectangle {
    largeur: f64,
    longueur: f64,
}

trait Aire {
  fn aire(&self) -> f64;
}

impl Aire for Rectangle {
  fn aire(&self) -> f64 {
    self.largeur * self.longueur
  }
}

fn main() {
  let rectangle = Rectangle {
    largeur: 2.0,
    longueur: 3.0,
  };
  println!("Aire du rectangle = {}", rectangle.aire());
}

// Les traits de la librairie standard
// Clone -> permet de rendre un type clonable -> clone()
// Debug -> println!("{:?}")
// Display -> println!("{}")
// PartialEq -> permet de comparer des types (mon_type == mon_autre_type)
// Default -> initialiser un type avec ses valeurs par défaut -> default()
// Drop -> méthode drop() appelée (implicite) juste avant que la var de ce type soit jetée de la mémoire
// Iterator -> itérer sur un élément (ex boucle for..in)