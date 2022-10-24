fn main() {
  let mon_tuple = (1, 3);
  let mon_autre_tuple: (usize, String, &str) = (42, "chats".to_string(), "Paris");
  let tuple_un_element: (i32,) = (8, ); // il faut ajouter une virgule !
  let tuple_vide: () = ();

  // pour accèder aux éléments: tuple.index
  println!(
    "nombre de {} qui vivent à {} = {}",
    mon_autre_tuple.1, mon_autre_tuple.2, mon_autre_tuple.0
);

// la méthode enumerate renvoie un tuple(index, element)
for (index, element) in [1, 2, 3].iter().enumerate() {
    println!("element index {index} = {element}");
}
}