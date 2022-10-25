// 2 types d'erreur

// Unrecoverable: crash le programme sans possibilité de gérer l'erreur.
// assert!()
// assert_eq!()
// panic!()
// todo!()
// unimplemented!()
// .unwrap()
// .expect()


// Recoverable: crash pas directement, gestion possible.
// enum Result<T, E> {
  // Ok(T),
  // Err(E),
}

// fn ouvrir(nom_fichier: &str) -> Result<String, String>
// let resultat = ouvrir("mon_fichier.txt");

// let contenu = match resultat {
//   Ok(contenu_fichier) => contenu_fichier, // retour implicite du contenu du fichier
//   Err(err) => return(Err(format!("erreur d'ouverture: {:?}", err))),
// }


// Bonne pratique: afficher des messages d'erreur clair.
// Panic => message d'erreur pas sympa.

use std::io::Read;

// Améliorer sa gestion d'erreur
#[derive(Debug)]
pub enum MonErreur {
    IoError(std::io::Error),
    Indisponible,
    Autre(String),
}

impl From<std::io::Error> for MonErreur {
    fn from(io_error: std::io::Error) -> Self {
        Self::IoError(io_error)
    }
}

fn ouvrir() -> Result<String, MonErreur> {
    let mut fichier = std::fs::File::open("test")?;
    let mut contenu = String::new();
    fichier.read_to_string(&mut contenu)?;

    Ok(contenu)
}

fn main() {
  match ouvrir() {
    Ok(_) => {}
    Err(err) => eprintln!("une erreur est survenue {err:?}"),
  }
}