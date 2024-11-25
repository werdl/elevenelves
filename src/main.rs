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

    println!("{:#?}", world.colonies[0].strongholds[0].elves[0]);

    // assign test task and see if it gets assigned to the best elf
    let task = Task {
        description: "Test Task".to_string(),
        required_roles: vec![Role::Trader, Role::Warrior],
        duration: 10,
        required_building: BuildingType::MeetingHall,
        required_objects: vec![],
        produced_objects: vec![],
    };

    world.colonies[0].strongholds[0].task_queue.push(task.clone());

    world.tick();

    println!("{:#?}", world.colonies[0].strongholds[0].task_queue);

    // now upgrade an elf
    // add 5 wood to the stronghold
    for _ in 0..15 {
        world.colonies[0].strongholds[0].stockpile.push(Object {
            resource_type: ResourceType::Plant,
            object_type: ObjectType::RawMaterial {
                name: "Plant".to_string(),
                description: "It's green and (possibly) edible".to_string(),
            },
        });
    }

    let elf = &mut world.colonies[0].strongholds[0].elves[0];
    println!("{:#?}", world.colonies[0].strongholds[0].buildings);
    println!("{:#?}", world.colonies[0].strongholds[0].upgrade_building(BuildingType::Farm));

    println!("{:#?}", world.colonies[0].strongholds[0].buildings);

    


}
