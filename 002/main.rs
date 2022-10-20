
fn main() {
  // variable immutable
  let i = 0;
  // variable mutable
  let mut j = 1;

  // Les types
  let a: i32 = 0; // integer 32 bits
  let b = 0i32;   // integer 32 bits
  // Inférence de type: let i = 0; Rust comprend qu'il s'agit d'un integer
  // i8, i16, i32, i64, i128 -> signé
  // u8, u16, u32, u64, u128 -> non-signé
  // isize, usize -> 32 ou 64 bits
  // f32, f64 -> flottant
  // String
  // slice

  a += 1; // incrémentation

  // Les slices
  // morceau de tableau (un pointeur + une taille)
  let tab = &[0, 1, 2]; 
}