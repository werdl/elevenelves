use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum AttributeLevel {
    Excellent = 5,
    Good = 4,
    Average = 3,
    Poor = 2,
    Terrible = 1,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum HappinessLevel {
    Ecstatic = 5,
    Happy = 4,
    Content = 3,
    Unhappy = 2,
    Miserable = 1,
    Depressed = 0,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Role {
    /// members of a council that makes decisions for the colony. being an elder boosts happiness, obedience, and loyalty of the elf. elders are the only elves that can become leaders. elders can also perform any task, but they are not as efficient as other roles. not being an elder after being one for a long time will cause a happiness drop, as will not being one after a certain age.
    Elder,

    /// leader of the colony. all leaders must have the elder role. having a leader with positive traits boosts happiness, obedience, and loyalty of all elves. conversely, having a leader with negative traits lowers happiness, obedience, and loyalty of all elves. leaders can also perform any task, but they are not as efficient as other roles. not being a leader after being one for a long time will cause a happiness drop. each colony can only have one leader at a time.
    Leader,

    /// Stronghold masters control each stronghold in the colony. they report to the leader. they can also perform any task, but they are not as efficient as other roles. each stronghold can only have one stronghold master at a time.
    StrongholdMaster,

    /// warriors protect the colony from enemies
    Warrior,

    /// farmers grow food for the colony
    Farmer,

    /// hunters gather meat for the colony
    Hunter,

    /// gatherers gather resources (seeds, logs etc.) for the colony
    Gatherer,

    /// carpenters build and repair wooden structures
    Carpenter,

    /// stonemasons build and repair stone structures
    Stonemason,

    /// blacksmiths build and repair metal structures
    Blacksmith,

    /// tailors make and repair clothing and leather or cloth armor
    Tailor,

    /// cooks prepare food for the colony
    Cook,

    /// healers heal injured or sick elves
    Healer,

    /// herbalists make medicine and potions
    Herbalist,

    /// alchemists make potions and other magical items. scientists look down upon alchemists, thus lowering the alchemist's happiness level when they are in the same building as a scientist.
    Alchemist,

    /// miners mine for resources such as stone, iron, and gold
    Miner,

    /// builders repair and upgrade buildings
    Builder,

    /// scientists research new technologies and make new items such as novel potions, albeit extremely slowly.
    Scientist,

    /// traders trade with other colonies and goblins (if possible)
    Trader,

    /// nitwits are useless and cannot perform any tasks. they are a burden, but can be trained to perform tasks. they are also more likely to rebel or leave the colony.
    Nitwit,
}

/// Resource types describe the materials that objects are made from
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ResourceType {
    Wood,
    Stone,
    Iron,
    Diamond,
    Gold,
    Cloth,
    Glass,
    Animal,
}

/// Object types describe the function of objects - ex. food, water, medicine, potions, etc. It also details their metadata
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ObjectType {
    Food {
        name: String,
        description: String,

        /// how many hunger points are restored - if percentage, will restore floor(hunger) and the decimal chance of restoring 1 more
        hunger_power: f32,
    
    },
    Water,
    Medicine {
        name: String,
        description: String,

        /// how much health is restored
        healing_power: u32,
    },
    Potion {
        name: String,
        description: String,

        /// how much health is restored
        healing_power: u32,

        /// how much strength is granted
        strength_power: u32,

        /// how much agility is granted
        agility_power: u32,

        /// how much intelligence is granted
        intelligence_power: u32,

        /// how much obedience is granted
        obedience_power: u32,

        /// how much loyalty is granted
        loyalty_power: u32,

        /// effect length in game ticks
        effect_length: u32,
    },
    Weapon {
        name: String,
        description: String,

        /// base damage of weapon (further affected by strength)
        damage_power: u32,

        /// how much agility is required to wield
        agility_requirement: AttributeLevel,

        /// how much agility is removed from wearer
        agility_penalty: AttributeLevel,
    },
    Armor {
        name: String,
        description: String,

        /// base defense of armor (further affected by strength)
        defense_power: u32,

        /// how much agility is required to wear
        agility_requirement: AttributeLevel,

        /// how much agility is removed from wearer
        agility_penalty: AttributeLevel,
    },
    PickAxe {
        name: String,
        description: String,

        /// base breaking power
        breaking_power: u32,

        /// how much agility is required to wear 
        agility_requirement: AttributeLevel,

        /// how much agility is removed from wearer
        agility_penalty: AttributeLevel
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Object {
    pub object_type: ObjectType,
    pub resource_type: ResourceType,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Task {
    /// name and synopsis of task
    pub description: String,

    /// duration of task in seconds
    pub duration: u32,

    /// required building for task
    pub required_building: BuildingType,

    /// required roles workers must have to perform task
    pub required_roles: Vec<Role>,

    /// required objects that must be present in the stockpile to perform task
    pub required_objects: Vec<Object>,

    /// objects that will be produced by task
    pub produced_objects: Vec<Object>,
}

/// describes how well an elf can perform a task
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct RoleAbility {
    pub role: Role,
    pub ability: AttributeLevel,
}

/// An elf in the colony
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Elf {
    /// name of elves (can be multiple names, ex: first, middle, last)
    pub name: Vec<String>,

    /// age of elf in years
    pub age: f32,

    /// affects elf's ability to withstand displeasure
    pub patience: AttributeLevel,

    /// affects elf's ability to think and reason (aka how fast they pick up on things)
    pub intelligence: AttributeLevel,

    /// affects elf's ability to withstand physical pain and deal damage
    pub strength: AttributeLevel,

    /// affects elf's ability to move quickly and gracefully (attack speed, dodge chance)
    pub agility: AttributeLevel,

    /// affects required happiness level to keep elf from leaving or rebelling
    pub obedience: AttributeLevel,

    /// affects elf's loyalty to the colony (aka how likely they are to leave or rebel)
    pub loyalty: AttributeLevel,

    /// current level of happiness (affects other stats and behavior)
    pub happiness: HappinessLevel,

    /// hunger level (affects happiness) (excellent = full, terrible = starving)
    pub hunger: AttributeLevel,

    /// thirst level (affects happiness) (excellent = full, terrible = dehydrated)
    pub thirst: AttributeLevel,

    /// sleep level (affects happiness) (excellent = well rested, terrible = exhausted). all elves need to sleep for 8 hours a day, but when this happens is irrelevant (usually when there are no tasks and their sleep level is Poor or Terrible, or when their sleep level is Terrible regardless of tasks)
    pub sleep: AttributeLevel,

    /// roles in the colony (affects behavior and stats)
    pub roles: Vec<RoleAbility>,

    /// current task (affects behavior and stats)
    pub task: Option<Task>,

    /// tick when current task was started
    pub task_start: Option<u64>,

    /// current building ID (affects behavior and stats)
    pub building: Option<u32>,

    /// current health level (affects behavior and stats), 0 = dead, 100 = full health
    pub health: u32, 
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum BuildingType {
    /// elders & leaders
    MeetingHall,

    /// warriors
    Barracks,

    /// farmers
    Farm,

    /// hunters
    HuntingLodge,

    /// gatherers
    GatheringHut,

    /// carpenters
    CarpenterWorkshop,

    /// stonemasons
    StonemasonWorkshop,

    /// blacksmiths
    Forge,

    /// tailors
    TailorShop,

    /// cooks
    Kitchen,

    /// healers
    Hospital,

    /// herbalists
    HerbalistHut,

    /// alchemists
    AlchemistLab,

    /// miners
    Mine,

    /// builders
    BuilderHut,

    /// scientists
    Laboratory,

    /// turrets
    Tower,

    /// keep out enemies
    Wall,

    /// traders
    TradingPost,
}

/// Buildings are required for tasks, ex. a blacksmith requires a forge, a cook requires a kitchen, etc. Buildings can be upgraded to improve efficiency, capacity, etc. Buildings can be destroyed by enemies, natural disasters, or elves rebelling. They are also needed for defense, ex. walls, towers, etc.
#[derive(Serialize, Deserialize, Debug)]
pub struct Building {
    /// building ID (unique identifier)
    pub id: u32,

    /// level of building (affects efficiency, capacity, etc.)
    pub level: u32,

    /// type of building (affects tasks, defense, etc.)
    pub building_type: BuildingType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Stronghold {
    /// name of stronghold
    pub name: String,

    /// elves in colony
    pub elves: Vec<Elf>,

    /// buildings in stronghold. if all buildings are destroyed, the stronghold is disbanded
    pub buildings: Vec<Building>,

    /// list of all tasks currently awaiting a worker
    pub task_queue: Vec<Task>,

    /// list of all items in stockpile
    pub stockpile: Vec<Object>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Colony {
    /// name of colony
    pub name: String,

    /// strongholds in colony
    pub strongholds: Vec<Stronghold>,

    /// owner of colony's username (currently unimplemented, but left in for future multiplayer functionality)
    pub leader: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct World {
    /// name of world
    pub name: String,

    /// colonies in game
    pub colonies: Vec<Colony>,

    /// current in game tick (twenty ticks per second)
    pub tick: u64,
}

pub struct Goblin {
    /// name of goblin
    pub name: Vec<String>,

    /// age of goblin in years
    pub age: f32,

    /// affects goblin's ability to withstand attacks and how much damage they can deal
    pub strength: AttributeLevel,

    /// affects goblin's ability to move quickly and dodge attacks. also affects attack speed
    pub agility: AttributeLevel,

    /// affects goblin's likelihood to flee or surrender
    pub loyalty: AttributeLevel,

    /// affects trading likelihood, prices, and behavior
    pub charisma: AttributeLevel,

    /// current level of health (0 = dead, 100 = full health)
    pub health: u32,
}

pub trait Random {
    fn random() -> Self;
}

impl Random for Role {
    fn random() -> Role {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..19) {
            0 => Role::Elder,
            1 => Role::Leader,
            2 => Role::StrongholdMaster,
            3 => Role::Warrior,
            4 => Role::Farmer,
            5 => Role::Hunter,
            6 => Role::Gatherer,
            7 => Role::Carpenter,
            8 => Role::Stonemason,
            9 => Role::Blacksmith,
            10 => Role::Tailor,
            11 => Role::Cook,
            12 => Role::Healer,
            13 => Role::Herbalist,
            14 => Role::Alchemist,
            15 => Role::Miner,
            16 => Role::Builder,
            17 => Role::Scientist,
            18 => Role::Trader,
            _ => Role::Nitwit,
        }
    }
}

impl Random for AttributeLevel {
    fn random() -> AttributeLevel {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..5) {
            0 => AttributeLevel::Excellent,
            1 => AttributeLevel::Good,
            2 => AttributeLevel::Average,
            3 => AttributeLevel::Poor,
            4 => AttributeLevel::Terrible,
            _ => AttributeLevel::Average,
        }
    }
}

pub fn profession_to_building(profession: Role) -> BuildingType {
    match profession {
        Role::Elder => BuildingType::MeetingHall,
        Role::Leader => BuildingType::MeetingHall,
        Role::StrongholdMaster => BuildingType::MeetingHall,
        Role::Warrior => BuildingType::Barracks,
        Role::Farmer => BuildingType::Farm,
        Role::Hunter => BuildingType::HuntingLodge,
        Role::Gatherer => BuildingType::GatheringHut,
        Role::Carpenter => BuildingType::CarpenterWorkshop,
        Role::Stonemason => BuildingType::StonemasonWorkshop,
        Role::Blacksmith => BuildingType::Forge,
        Role::Tailor => BuildingType::TailorShop,
        Role::Cook => BuildingType::Kitchen,
        Role::Healer => BuildingType::Hospital,
        Role::Herbalist => BuildingType::HerbalistHut,
        Role::Alchemist => BuildingType::AlchemistLab,
        Role::Miner => BuildingType::Mine,
        Role::Builder => BuildingType::BuilderHut,
        Role::Scientist => BuildingType::Laboratory,
        Role::Trader => BuildingType::TradingPost,
        _ => BuildingType::MeetingHall,
    }
}

pub fn building_to_profession(building: BuildingType) -> Role {
    match building {
        BuildingType::MeetingHall => Role::Elder,
        BuildingType::Barracks => Role::Warrior,
        BuildingType::Farm => Role::Farmer,
        BuildingType::HuntingLodge => Role::Hunter,
        BuildingType::GatheringHut => Role::Gatherer,
        BuildingType::CarpenterWorkshop => Role::Carpenter,
        BuildingType::StonemasonWorkshop => Role::Stonemason,
        BuildingType::Forge => Role::Blacksmith,
        BuildingType::TailorShop => Role::Tailor,
        BuildingType::Kitchen => Role::Cook,
        BuildingType::Hospital => Role::Healer,
        BuildingType::HerbalistHut => Role::Herbalist,
        BuildingType::AlchemistLab => Role::Alchemist,
        BuildingType::Mine => Role::Miner,
        BuildingType::BuilderHut => Role::Builder,
        BuildingType::Laboratory => Role::Scientist,
        BuildingType::TradingPost => Role::Trader,
        BuildingType::Tower => Role::Warrior,
        BuildingType::Wall => Role::Warrior,
        _ => Role::Nitwit,
    }
}
