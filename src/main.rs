pub mod defs;
pub mod impls;
pub mod game;

use {
    defs::*,
    impls::*,
    game::*,
};

fn main() {
    let mut world = World::new(Some("World".to_string()), Some("Player".to_string()));

    println!("{:#?}", world.colonies[0].strongholds[0].elves);

    // assign test task and see if it gets assigned to the best elf
    let task = Task {
        description: "Test Task".to_string(),
        required_roles: vec![Role::Trader, Role::Warrior],
        duration: 10,
        required_building: BuildingType::MeetingHall,
        required_objects: vec![],
        produced_objects: vec![],
    };

    let res = world.colonies[0].strongholds[0].new_task(task.clone());

    println!("{:#?}", res);
}
