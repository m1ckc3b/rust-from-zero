fn main() {
  // Vecteur = tableau à taille dynamique
  let mon_vecteur = vec![0usize; 5];            // tableau de taille 5, remplit de 0.
  let mon_autre_vecteur = vec![1 , 2, 3, 4, 5]; // tableau de taille 5.
  println!("egalite ? {}", mon_vecteur == mon_autre_vecteur);


  println!("mon_vecteur avant dedup {:?}", mon_vecteur);
    mon_vecteur.dedup();  // helper: déduplique
    println!("mon_vecteur après dedup {:?}", mon_vecteur);
    mon_vecteur.push(5);
    mon_vecteur.push(5);
    mon_vecteur.push(5);
    mon_vecteur.push(5);
    mon_vecteur.push(6);

    // itération sur un vecteur
    println!("mon_autre_vec {:#?}", mon_autre_vec);
    for mon_element in mon_autre_vec.iter().skip(2).rev() {
        println!("mon element {mon_element}");
    }


    // Collection -> dictionnaire
    let mut animaux: HashMap<String, i32> = HashMap::new();
    animaux.insert("chien".to_string(), 42);
    animaux.insert("chat".to_string(), 3);

    println!(
        "nombre de chats = {}",
        animaux.get(&"chat".to_string()).unwrap()
    );

    // itération sur une collection
    for (animal, nombre) in animaux {
        println!("animal '{animal}' = {nombre}");
    }

}