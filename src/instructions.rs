use rand::seq::SliceRandom;

static SIDES: [&str; 2] = ["gauche", "droite"];
static BODY_PARTS: [&str; 4] = ["ta tete", "ton nez", "ton ventre", "ton cou"];

fn random_hand_instructions() -> String {
    let mut rng = rand::thread_rng();
    format!(
        "Mets ta main {} sur {}",
        SIDES.choose(&mut rng).unwrap(),
        BODY_PARTS.choose(&mut rng).unwrap()
    )
    .to_string()
}

fn random_foot_instructions() -> String {
    let mut rng = rand::thread_rng();
    format!("Lève ton pied {}", SIDES.choose(&mut rng).unwrap(),).to_string()
}

pub fn random_instructions() -> String {
    let mut rng = rand::thread_rng();
    let funcs = [random_hand_instructions, random_foot_instructions];
    funcs.choose(&mut rng).unwrap()()
}
