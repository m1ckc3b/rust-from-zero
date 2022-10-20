fn main() {
  // Conditions:
  let age: i32 = 17;

  if age >= 18 {
      println!("majeur !");
  } else {
      println!("mineur !");
  }
  // pas de parenthèses autour de la condition
  // mais accolades obligatoires autour du bloc de la condition.

  // Pattern matching:
  let my_string = "hello";

  match my_string {
      "bonjour" => {
          println!("français");
      }
      "ciao" => "italien";
      "hello" => {
          println!("anglais");
      }
      "hola" => {
          println!("espagnol");
      }
      _ => { // équivalent à "default"
          println!("je ne connais pas cette langue...");
      }
  }

  // "binder" (ou matcher sur un ensemble de valeurs) 
  // la variable avec le symbole "@"
  let i = 0i32;

  match i {
      x @ 10..=100 => println!("{} est entre 10 et 100 (inclus)", x),
      x => println!("{} n'est pas entre 10 et 100 (inclus)", x)
  };
}
