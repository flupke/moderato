use rand::seq::{IteratorRandom, SliceRandom};

fn random_hand_instructions() -> String {
    let mut rng = rand::thread_rng();
    format!(
        "Mets ta main {} sur {}",
        ["gauche", "droite"].choose(&mut rng).unwrap(),
        ["ta tête", "ton nez", "ton ventre", "ton cou"]
            .choose(&mut rng)
            .unwrap()
    )
}

fn random_foot_instructions() -> String {
    let mut rng = rand::thread_rng();
    format!(
        "Lève ton pied {}",
        ["gauche", "droit"].choose(&mut rng).unwrap(),
    )
}

fn random_addition() -> String {
    let mut rng = rand::thread_rng();
    format!(
        "{} + {} = ?",
        (1..10).choose(&mut rng).unwrap(),
        (1..10).choose(&mut rng).unwrap()
    )
}

fn random_clothes_color_question() -> String {
    let mut rng = rand::thread_rng();
    format!(
        "De quelle couleur {} ?",
        [
            "est ton tee-shirt",
            "est ton pantalon",
            "sont tes chaussettes",
            "est ton slip",
        ]
        .choose(&mut rng)
        .unwrap(),
    )
}

pub fn random_instructions() -> String {
    let mut rng = rand::thread_rng();
    let funcs = [
        random_hand_instructions,
        random_foot_instructions,
        random_addition,
        random_clothes_color_question,
    ];
    funcs.choose(&mut rng).unwrap()()
}
