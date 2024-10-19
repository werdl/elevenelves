use crate::defs::*;
use crate::game::*;

// random number generator
use rand::{random, Rng};



pub trait NewEntity {
    fn new(age: Option<f32>, surname: Option<String>, roles: Option<Vec<Role>>) -> Self;
}

impl NewEntity for Elf {
    fn new(age: Option<f32>, surname: Option<String>, roles: Option<Vec<Role>>) -> Self {
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
        let mut roles = roles.unwrap_or_else(|| {
            let mut roles = Vec::new();

            while roles.len() < 3 {
                roles.push(Role::random());

                roles.dedup();
            }

            roles
        });

        // pad out roles if less than 3
        while roles.len() < 3 {
            roles.push(Role::random());

            // ensure last role pushed is unique and is not nitwit
            if roles.len() == 3 {
                roles.dedup();
                roles.retain(|role| *role != Role::Nitwit);
            }
        }

        // if nitwit is present, remove all other roles
        if roles.contains(&Role::Nitwit) {
            roles = vec![Role::Nitwit];
        }

        let finished_roles = roles.iter().map(|role| RoleAbility {
            role: role.clone(),
            ability: AttributeLevel::random(),
        }).collect::<Vec<RoleAbility>>();

        Elf {
            name: name,
            age: age.unwrap_or_else(|| rng.gen_range(18.0..100.0) as i32 as f32),
            roles: finished_roles,

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
            task_start: None,
        }
    }
}

impl NewEntity for Goblin {
    fn new(age: Option<f32>, surname: Option<String>, roles: Option<Vec<Role>>) -> Self {
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

        Goblin {
            name,
            age: age.unwrap_or_else(|| rng.gen_range(18.0..100.0)),
            strength: AttributeLevel::random(),
            agility: AttributeLevel::random(),
            loyalty: AttributeLevel::random(),
            charisma: AttributeLevel::random(),
            health: 100,
        }
    }
}

pub trait TaskOperations {
    fn new_task(&mut self, task: Task) -> Result<(), GameError>;
    fn list_tasks(&self, elf: Option<&Elf>) -> Result<Vec<Task>, GameError>;
    fn check_tasks_complete(&mut self, tick: u64) -> Result<Vec<Object>, GameError>;
}

impl TaskOperations for Stronghold {
    fn new_task(&mut self, task: Task) -> Result<(), GameError> {
        // first, check we have the required building(s)
        if !self.buildings.iter().any(|building| building.building_type == task.required_building) {
            return Err(GameError::NoSuitableBuildingError("Missing required building".to_string()));
        }
        

        // first, check if we have a free elf
        let mut free_elves = self.elves.iter_mut().filter(|elf| elf.task.is_none()).collect::<Vec<&mut Elf>>();
        // remove elves that don't have the required roles (all roles must be present)
        for role in task.required_roles.iter() {
            // ensure all elves have this role
            free_elves.retain(|elf| elf.roles.iter().any(|r| r.role == *role));
        }


        if free_elves.is_empty() {
            // if an elf exists but isn't available, we push to the task queue
            let mut possible_elves = self.elves.clone();

            for role in task.required_roles.iter() {
                // ensure all elves have this role
                possible_elves.retain(|elf| elf.roles.iter().any(|r| r.role == *role));
            }

            if !possible_elves.is_empty() {
                self.task_queue.push(task);
                return Ok(());
            }

            // othwise, unlikely to fixed quickly, so we error out rather than push to the task queue
            return Err(GameError::NoSuitableElfError("No suitable elf available".to_string()));
        }


        // factor in elf skill level to task duration
        let mut task = task.clone();
        
        // find the elf with the best combined skill level of the required roles
        let mut best_elf_position = 0;

        let mut current_best_skill = 0;

        for elf in &free_elves {
            let mut combined_skill = 0;

            for role in task.required_roles.iter() {
                if let Some(role_ability) = elf.roles.iter().find(|r| r.role == *role) {
                    combined_skill += role_ability.ability.clone() as i32;
                }
            }

            if combined_skill > current_best_skill {
                best_elf_position = free_elves.iter().position(|e| e == elf).unwrap();
                current_best_skill = combined_skill;
            }
        }

        println!("Best elf stats: {:#?}", free_elves[best_elf_position]);

        // assign the task to the first free elf
        free_elves[best_elf_position].task = Some(task);
        Ok(())
    }
    fn list_tasks(&self, elf: Option<&Elf>) -> Result<Vec<Task>, GameError> {
        if let Some(elf) = elf {
            if let Some(task) = &elf.task {
                return Ok(vec![task.clone()]);
            }
        }

        Ok(self.task_queue.clone())
    }
    fn check_tasks_complete(&mut self, tick: u64) -> Result<Vec<Object>, GameError> {
        let mut completed_tasks = Vec::new();

        for elf in &mut self.elves {
            if let Some(task) = &elf.task {
                if elf.task_start.unwrap_or(0) + task.duration as u64 >= tick {
                    // task is complete
                    for object in &task.produced_objects {
                        completed_tasks.push(object.clone());
                    }
                    elf.task = None;
                    elf.task_start = None;
                }
            }
        }

        Ok(completed_tasks)
    }
}