# bevy-prefabs
Plugin for Rust Bevy engine enabling you to store serialized prefabs for entities. Currently only `.yaml` file format is supported, hopefuly this crate will be format agnostic and will let u chose which deserializer to use.

## Usage
All prefabs are stored in `PrefabBank` resource, in order to have `PrefabBank` available you need to add `PrefabPlugin` to your app, like so:
```rust
use bevy_prefabs::*;
use bevy::prelude::*;
fn main(){
  App::build()
      .add_plugin(PrefabPlugin);
}
```
In order to make your component a prefab it needs to derive `Serializable`, `Deserializable`, `Clone` and of course `Prefab`, for example:
```rust
#[derive(Prefab,Serialize,Deserialize,Clone)]
struct Weapon {
    damage: i32,
    range: f32
}
```
Loading a prefab with a system:
```rust
fn load_prefabs(mut prefab_bank: ResMut<PrefabBank>){
     prefab_bank
         .load_prefab("SomeItem","target/debug/prefabs/SomeItem".as_ref()).expect("Can't load prefab");
}
```
`.load_prefab()` method takes your desired name for the prefab (you will later use it to create entity with this prefab) and a path to the FOLDER where all serialized components for the prefab are stored in.
Spawning a prefab now is super easy:
```rust
fn spawn_prefab(prefab_bank: Res<PrefabBank>, mut commands: Commands) {
    prefab_bank.spawn_prefab("SomeItem", &mut commands);
    prefab_bank.spawn_prefab("SomeItem", &mut commands);
    prefab_bank.spawn_prefab("SomeItem", &mut commands);
    prefab_bank.spawn_prefab("SomeItem", &mut commands);
}
```
To serialize existing component you need to serialize it `as Prefab` otherwise typetag will not be serialized and such component once deserialized will not be accepted in `PrefabBank`.