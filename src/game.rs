use crate::{defs::*, impls::*};
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
        elves.push(Elf::new(None, None, Some(vec![Role::Trader])));
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
                
            }
        }

        self.tick += 1;

        Ok(())
    }
}

