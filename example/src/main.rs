use bevy::prelude::*;
use bevy::ecs::system::EntityCommands;
use bevy_prefabs::*;

use serde::{Serialize,Deserialize};
use rand::Rng;

// Here I create simple component for items and turn it into a serializable prefab
#[derive(Prefab,Clone, Serialize, Deserialize, Debug)]
struct ItemInfo{
    name: String,
    description: String
}

/*  
Here I create simple weapon component, but let's say I would like all weapons in my game to have randomly generated 
damage for each instance of the same weapon. That's why I don't derive Prefab and create Adapter down below
*/
#[derive(Debug)]
struct Weapon{
    damage: i32,
    range: usize
}

// Here is the adapter for Weapon component, it stores min and max values for damage and range which will be constant for all instances of same weapon
#[derive(Serialize, Deserialize)]
struct WeaponAdapter{
    min_damage: i32,
    max_damage: i32,

    range: usize
}

/* 
In order to have working adapter you need to implement Prefab and transform data from your adapter to target component. It's simple as that!
Also remember to add `#[typetag::serde]` it's neccessarry to have something serialized as a prefab.
*/
#[typetag::serde]
impl Prefab for WeaponAdapter{
    fn add_to_entity(&self, entity_commands: &mut EntityCommands){
        let generated_damage = rand::thread_rng().gen_range(self.min_damage..self.max_damage);

        entity_commands.insert(Weapon{
            damage: generated_damage,
            range: self.range
        });
    }
}


// Load prefabs from directories. Each directory represents one entity prefab and .yaml files inside are serialized components/adapters
fn load_prefabs(mut prefab_bank: ResMut<PrefabBank>){
    prefab_bank
        .load_prefab("LegendaryBow","prefabs/LegendaryBow".as_ref()).expect("Can't load prefab");
    prefab_bank
        .load_prefab("LousySword","prefabs/LousySword".as_ref()).expect("Can't load prefab");
}

// Spawn some weapons
fn spawn_weapons(prefab_bank: Res<PrefabBank>, mut commands: Commands) {
    prefab_bank.spawn_prefab("LegendaryBow", &mut commands);
    prefab_bank.spawn_prefab("LegendaryBow", &mut commands);
    prefab_bank.spawn_prefab("LegendaryBow", &mut commands);

    prefab_bank.spawn_prefab("LousySword", &mut commands);
    prefab_bank.spawn_prefab("LousySword", &mut commands);
}

// Print out weapon with their names and descriptions
fn print_weapons_with_names(named_weapon_query: Query<(&ItemInfo,&Weapon)>) {
    for (name,weapon) in named_weapon_query.iter(){
        println!("*********************************");
        println!("{:#?}", name);
        println!("{:#?}\n\n", weapon);
    }
}

fn main() {
    App::build()
        .add_plugin(PrefabPlugin)
        .add_startup_system(load_prefabs.system().label("load"))
        .add_startup_system(spawn_weapons.system().label("weapon_spawn").after("load"))
        .add_system(print_weapons_with_names.system().after("weapon_spawn"))
        .run();
}
