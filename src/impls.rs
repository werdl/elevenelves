use crate::defs::*;

// random number generator
use rand::{random, Rng};



pub trait NewEntity {
    fn new(age: Option<f32>, surname: Option<String>) -> Self;
}

impl NewEntity for Elf {
    fn new(age: Option<f32>, surname: Option<String>) -> Self {
        let forenames = vec![
            "vfaanraazr",
            "raazr",
            "moetraazr",
            "apeth",
        ];

        let surnames = vec![
            "zroahhaa",
            "zruamoet",
            "hhaavfoetsraazr",
            "zroahhaavfoetsraazr"
        ];

        let mut rng = rand::thread_rng();

        let name = vec![forenames[rng.gen_range(0..forenames.len())].to_string(), surname.unwrap_or_else(|| surnames[rng.gen_range(0..surnames.len())].to_string())];

        // randomly select 3 roles
        let mut roles = Vec::new();

        for _ in 0..3 {
            roles.push(RoleAbility {
                role: Role::random(),
                ability: AttributeLevel::random(),
            });
        }

        Elf {
            name: name,
            age: age.unwrap_or_else(|| rng.gen_range(18.0..100.0)),
            roles,

            happiness: HappinessLevel::Content,
            patience: AttributeLevel::random(),
            intelligence: AttributeLevel::random(),
            strength: AttributeLevel::random(),
            agility: AttributeLevel::random(),
            loyalty: AttributeLevel::random(),
            obedience: AttributeLevel::random(),
            hunger: AttributeLevel::random(),
            thirst: AttributeLevel::random(),
            sleep: AttributeLevel::random(),
            health: 100,

            building: None,
            task: None,
        }
    }

}