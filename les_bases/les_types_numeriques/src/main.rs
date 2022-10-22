fn main() {
    let ma_valeur_virgule = 0.3; // inférence de type.
    let valeur_entiere = 42;
    let valeur_entiere_type: i32 = 42;
    let mon_usize = 5usize;
    let petite_donnee: i8 = 55;

    let somme = valeur_entiere + mon_usize; // inférence de type: valeur_entiere devient un usize.
    let mut somme = valeur_entiere_type as usize + mon_usize; // change temporairement le type avec "as".

    println!("voici mon résultat: {}", somme);

    somme += 42;

    println!("voici mon résultat: {somme}");
}

// Taille	    Signé	Non signé
// -------------------------------
// 8 bits	    i8	    u8
// 16 bits	    i16	    u16
// 32 bits	    i32	    u32
// 64 bits	    i64	    u64
// 128 bits	    i128	u128
// archi	    isize	usize