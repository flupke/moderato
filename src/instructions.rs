use rand::seq::SliceRandom;

fn random_hand_instructions() -> String {
    let mut rng = rand::thread_rng();
    format!(
        "Mets ta main {} sur {}",
        ["gauche", "droite"].choose(&mut rng).unwrap(),
        ["ta tête", "ton nez", "ton ventre", "ton cou"]
            .choose(&mut rng)
            .unwrap()
    )
    .to_string()
}

fn random_foot_instructions() -> String {
    let mut rng = rand::thread_rng();
    format!(
        "Lève ton pied {}",
        ["gauche", "droit"].choose(&mut rng).unwrap(),
    )
    .to_string()
}

pub fn random_instructions() -> String {
    let mut rng = rand::thread_rng();
    let funcs = [random_hand_instructions, random_foot_instructions];
    funcs.choose(&mut rng).unwrap()()
}
