

// Enums
enum Direction {
  Nord,
  Sud, 
  Est,
  Ouest,
}

impl Direction {
  fn info(&self) {
    // pattern matching => équivalent au destructuring
    match self {
      Direction::Nord => println!("Je suis au nord");
      Direction::Sud => println!("Je suis au sud");
      Direction::Est => println!("Je suis à l'est");
      Direction::Ouest => println!("Je suis à l'ouest");
    }
  }
}

enum ipAddr {
  V4(String),
  V6(String),
}

impl ipAddr {
  fn addr(&self) -> String {
    match self {
      ipAddr::V4(ip_addr_str) => ip_addr_str.clone(),
      ipAddr::V6(ip_addr_str) => ip_addr_str.clone(),
    }
  }
}

fn main() {
  let direction = Direction::Nord;
  direction.info();


  let ip_addr = ipAddr::V4(String::from("127.0.0.1")).addr();
}

// Les enums de la librairie standard
// Option -> remplace null
// Result -> gestion des erreurs

// A implémenter
// Either -> lorsqu'une donnée peut être sous 2 formes différentes
pub enum Either<L, R> {
  Left(L),
  Right(R),
}