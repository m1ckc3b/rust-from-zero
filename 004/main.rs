
fn main() {
  // if let -> simplifier le traitement de pattern matching
  if let Some(s) = fais_quelque_chose(100) {
    println!("{}", &s)
  } else {
    println!("il ne s'est rien passé")
  }

  // while let
  let mut v = vec!(1, 2, 3);

  while let Some(x) = v.pop() {
      println!("{}", x);
  }
}

fn fais_quelque_chose(i: i32) -> Option<String> {
  if i < 10 {
      Some("variable inférieure à 10".to_owned())
  } else {
      None
  }
}