fn main() {
  // Utilité : éviter les data races et libérer la data au bon moment.
  
  // les 3 régles (T = un type de donnée)
  // T -> un seul propriétaire.
  // &T -> on détient une/plusieurs ref immutables de T.
  // &mut T -> on détient une ref mutable et exclusive de T.


  // la syntaxe
  // T -> un seul proprio sur la donnée au même moment
  // fn take_ownership(my_string: String)

  // &T -> on détient une/plusieurs ref immutables
  // fn immutable_reference(my_string: &String)

  // &mut T 
  // fn mutable_reference(my_string: &mut String)


  // utiliser le borrow checker
  let ma_string = String::from("Ceci est ma string");
  prend_possession(ma_string);                    // la fonction possède ma_string (régle 1).
  println!("J'affiche ma string: {}", ma_string); // Error: impossible de prendre possession de ma_string!

  prend_une_reference(&ma_string);                // la fonction ne possède pas ma_string (régle 1).
  println!("J'affiche ma string: {}", ma_string); // Ici pas d'erreur !
}

fn prend_possession(my_string_param: String) {
  println!("my_string {}", my_string_param);
}

fn prend_une_reference(my_string_param: &String) {
  println!("my_string {}", my_string_param);
}