use crate::{defs::*, impls::*, };
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::File;
use std::io::{Read, Write};

/// error types
#[derive(Debug)]
pub enum GameError {
    SaveError(String),
    LoadError(String),
    NoSuitableElfError(String),
    NoSuitableBuildingError(String),
    IoError(std::io::Error),
    ParseError(std::num::ParseIntError),
    Utf8Error(std::str::Utf8Error),
}

impl From<std::io::Error> for GameError {
    fn from(error: std::io::Error) -> Self {
        GameError::IoError(error)
    }
}

impl From<std::num::ParseIntError> for GameError {
    fn from(error: std::num::ParseIntError) -> Self {
        GameError::ParseError(error)
    }
}

impl From<std::str::Utf8Error> for GameError {
    fn from(error: std::str::Utf8Error) -> Self {
        GameError::Utf8Error(error)
    }
}

impl From<serde_json::Error> for GameError {
    fn from(error: serde_json::Error) -> Self {
        GameError::SaveError(error.to_string())
    }
}


pub trait GameOptions {
    fn new(name: Option<String>, username: Option<String>) -> Self;
    fn save(&self) -> Result<(), GameError>;
    fn load(name: String) -> Result<Self, GameError> where Self: Sized;
    fn tick(&mut self) -> Result<(), GameError> ;
}

impl GameOptions for World {
    fn new(name: Option<String>, username: Option<String>) -> Self {
        let mut elves = Vec::new();
        
        // generate initial 11 elves
        
        elves.push(Elf::new(None, None, Some(vec![Role::Leader, Role::Elder])));
        elves.push(Elf::new(None, None, Some(vec![Role::Trader, Role::Elder, Role::Miner])));
        elves.push(Elf::new(None, None, Some(vec![Role::Trader, Role::Warrior])));
        elves.push(Elf::new(None, None, Some(vec![Role::Warrior, Role::Elder])));
        elves.push(Elf::new(None, None, Some(vec![Role::Warrior, Role::Miner])));
        elves.push(Elf::new(None, None, Some(vec![Role::Warrior])));
        elves.push(Elf::new(None, None, Some(vec![Role::Scientist, Role::Elder])));
        elves.push(Elf::new(None, None, Some(vec![Role::Farmer, Role::Elder])));
        elves.push(Elf::new(None, None, Some(vec![Role::Cook])));
        elves.push(Elf::new(None, None, Some(vec![Role::Farmer])));
        elves.push(Elf::new(None, None, Some(vec![Role::Miner])));




        // generate initial 3 buildings
        let buildings = vec![
            Building {
                id: 0,
                building_type: BuildingType::MeetingHall,
                level: 1
            },
            Building {
                id: 1,
                building_type: BuildingType::Barracks,
                level: 1
            },
            Building {
                id: 2,
                building_type: BuildingType::Farm,
                level: 1
            }
        ];

        // generate a stockpile: 110 food, 550 water
        let mut stockpile = vec![];

        for _ in 0..110 {
            stockpile.push(Object {
                object_type: ObjectType::Food {
                    name: "Unidentified Meat".to_string(),
                    description: "A piece of meat from an unknown animal".to_string(),
                    hunger_power: 3.0,
                },
                resource_type: ResourceType::Animal,
            });
        }

        for _ in 0..550 {
            stockpile.push(Object {
                object_type: ObjectType::Water,
                resource_type: ResourceType::Glass, // glass bottle
            });
        }

        let stronghold = Stronghold {
            name: format!("First Stronghold of {} Colony", name.clone().unwrap_or_else(|| "Earth".to_string())),
            elves,
            buildings,
            task_queue: Vec::new(),
            stockpile,
        };

        let colony = Colony {
            name: name.clone().unwrap_or_else(|| "Earth".to_string()),
            strongholds: vec![stronghold],
            leader: username.unwrap_or_else(|| "Player".to_string()),
        };

        World {
            name: name.unwrap_or_else(|| "World".to_string()),
            colonies: vec![colony],
            tick: 0,
        } 
    }

    fn save(&self) -> Result<(), GameError> {
        let json = serde_json::to_string(&self)?;
        let mut file = File::create("world.json")?;
        file.write_all(json.as_bytes())?;

        Ok(())
    }

    fn load(name: String) -> Result<Self, GameError> {
        let mut file = File::open("world.json")?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;

        let world: World = serde_json::from_str(&buffer)?;

        Ok(world)
    }

    fn tick(&mut self) -> Result<(), GameError> {
        // todo

        // check all elves for task completion
        for colony in &mut self.colonies {
            for stronghold in &mut colony.strongholds {
                stronghold.check_tasks_complete(self.tick)?;

                // now check if there are any tasks we can now do in the task queue
                // for every task in the queue, try to call assign_task, and if returned Ok(true), remove the task from the queue
                let mut i = 0;
                while i < stronghold.task_queue.len() {
                    let task = stronghold.task_queue[i].clone();
                    if stronghold.new_task(task)? {
                        stronghold.task_queue.remove(i);
                    } else {
                        i += 1;
                    }
                }

            }
        }

        self.tick += 1;

        Ok(())
    }
}

pub trait Train {
    fn train(&mut self, elf: i32, role: Role) -> Result<(), GameError>;
}

impl Train for Stronghold {
    fn train(&mut self, elf_index: i32, role: Role) -> Result<(), GameError> {
        let elf = &mut self.elves[elf_index as usize];

        // if we are already at excellent level, return
        if elf.roles.iter().any(|r| r.role == role && r.ability == AttributeLevel::Excellent) {
            return Ok(());
        }


        // check if we have the required building
        if !self.buildings.iter().any(|building| building.building_type == profession_to_building(role.clone())) {
            return Err(GameError::NoSuitableBuildingError("Missing required building".to_string()));
        }

        // ensure that the given elf is unoccupied
        if elf.task.is_some() {
            return Err(GameError::NoSuitableElfError("Elf is already occupied".to_string()));
        }

        // determine how many resources we need to train the elf (specified by profession_to_resource)
        let required_resources = profession_to_resource(role.clone());

        // check if we have enough resources
        let num_required = match elf.roles.iter().find(|r| r.role == role) {
            Some(role) => role.ability as i32 + 1,
            None => 1,
        };

        // now check if we have enough resources
        let mut num_resources = 0;

        for object in &self.stockpile {
            if object.resource_type == required_resources {
                num_resources += 1;
            }
        }

        if num_resources < num_required {
            return Err(GameError::NoSuitableBuildingError("Not enough resources".to_string()));
        }

        // remove the resources
        self.stockpile.retain(|object| object.resource_type != required_resources);

        // now train the elf
        let role_ability = elf.roles.iter_mut().find(|r| r.role == role);

        if let Some(role_ability) = role_ability {
            role_ability.ability = match role_ability.ability {
                AttributeLevel::Terrible => AttributeLevel::Poor,
                AttributeLevel::Poor => AttributeLevel::Average,
                AttributeLevel::Average => AttributeLevel::Good,
                AttributeLevel::Good => AttributeLevel::Excellent,
                AttributeLevel::Excellent => AttributeLevel::Excellent, // should never happen
            };
        } else {
            elf.roles.push(RoleAbility {
                role,
                ability: AttributeLevel::Terrible,
            });
        }

        Ok(())
    }
}

pub trait UpgradeBuilding {
    fn upgrade_building(&mut self, building: BuildingType) -> Result<(), GameError>;
}

impl UpgradeBuilding for Stronghold {
    fn upgrade_building(&mut self, building: BuildingType) -> Result<(), GameError> {
        // upgrade all buildings of the given type - if there are none, create one at level 1

        // for each building, consume resources of the level of the building + 1
        // if we don't have enough resources, return an error

        let mut found = false;

        // first, upgrade any and all existing buildings
        for checked_building in &mut self.buildings {
            if checked_building.building_type == building {
                found = true;


                // first, check if we have enough resources
                let num_required = checked_building.level + 1;

                let mut num_resources = 0;

                for object in &self.stockpile {
                    if object.resource_type == building_to_resource(building.clone()) {
                        num_resources += 1;
                    }
                }

                if num_resources < num_required {
                    return Err(GameError::NoSuitableBuildingError("Not enough resources".to_string()));
                }

                // remove num_required resources
                for _ in 0..num_required {
                    self.stockpile.remove(
                        self.stockpile.iter().position(|object| object.resource_type == building_to_resource(building.clone())).unwrap());
                }

                checked_building.level += 1;
            }
        }

        if !found {
            // create a new building
            let num_required = 1;

            let mut num_resources = 0;


            for object in &self.stockpile {
                if object.resource_type == building_to_resource(building.clone()) {
                    num_resources += 1;
                }
            }

            if num_resources < num_required {
                return Err(GameError::NoSuitableBuildingError("Not enough resources".to_string()));
            }

            // remove num_required resources
            for _ in 0..num_required {
                self.stockpile.remove(
                    self.stockpile.iter().position(|object| object.resource_type == building_to_resource(building.clone())).unwrap());
            }

            self.buildings.push(Building {
                id: self.buildings.len() as u32,
                building_type: building,
                level: 1,
            });
        }

        Ok(())
    }
}