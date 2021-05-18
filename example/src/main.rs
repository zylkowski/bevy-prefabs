use bevy::prelude::*;
use bevy::ecs::system::EntityCommands;
use bevy_prefabs::*;

use serde::{Serialize,Deserialize};
use rand::Rng;

#[derive(Prefab,Clone, Serialize, Deserialize, Debug)]
struct ItemInfo{
    name: String,
    description: String
}

#[derive(Debug)]
struct Weapon{
    damage: i32,
    range: usize
}

#[derive(Serialize, Deserialize)]
struct WeaponAdapter{
    min_damage: i32,
    max_damage: i32,

    range: usize
}

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

fn load_prefabs(mut prefab_bank: ResMut<PrefabBank>){
    prefab_bank
        .load_prefab("LegendaryBow","prefabs/LegendaryBow".as_ref()).expect("Can't load prefab");
    prefab_bank
        .load_prefab("LousySword","prefabs/LousySword".as_ref()).expect("Can't load prefab");
}

fn spawn_weapons(prefab_bank: Res<PrefabBank>, mut commands: Commands) {
    prefab_bank.spawn_prefab("LegendaryBow", &mut commands);
    prefab_bank.spawn_prefab("LegendaryBow", &mut commands);
    prefab_bank.spawn_prefab("LegendaryBow", &mut commands);

    prefab_bank.spawn_prefab("LousySword", &mut commands);
    prefab_bank.spawn_prefab("LousySword", &mut commands);
}

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
