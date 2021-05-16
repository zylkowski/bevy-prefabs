pub use bevy_prefabs_derive::Prefab;
use bevy::{ecs::system::EntityCommands, prelude::*};
use std::{collections::HashMap, fs, path::Path};
use std::result;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    IOError(std::io::Error),
    DeserializeError(serde_yaml::Error),
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::IOError(err)
    }
}

impl From<serde_yaml::Error> for Error {
    fn from(err: serde_yaml::Error) -> Error {
        Error::DeserializeError(err)
    }
}


#[typetag::serde(tag = "_prefab")]
pub trait Prefab: Send + Sync + 'static {
    fn add_to_entity(&self, entity_commands: &mut EntityCommands);
}

#[derive(Default)]
pub struct PrefabBank{
    prefab_bank: HashMap<String, Vec<Box<(dyn Prefab)>>>,
}

impl std::fmt::Debug for PrefabBank{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.prefab_bank.keys(), f)
    }
}

impl PrefabBank{
    fn register_prefab(&mut self, name: &str, component_boxed: Box<dyn Prefab>) {
        match self.prefab_bank.get_mut(name) {
            Some(prefab_storage) => {
                prefab_storage.push(component_boxed);
            }
            None => {
                self.prefab_bank
                    .insert(name.to_string(), vec![component_boxed]);
            }
        };
    }

    pub fn load_prefab(&mut self, prefab_name: &str, path: &Path) -> Result<()> {
        for component_path in fs::read_dir(path)? {
            let prefab_content = fs::read_to_string(component_path.unwrap().path())?;
            let prefab: Box<dyn Prefab> = serde_yaml::from_str(&prefab_content)?;
            self.register_prefab(&prefab_name, prefab);
        }
        Ok(())
    }

    pub fn spawn_prefab(&self, name: &str, command: &mut Commands) {
        let mut entity = command.spawn();
        if let Some(prefab_collection) = self.prefab_bank.get(name) {
            prefab_collection
                .iter()
                .for_each(|prefab| prefab.add_to_entity(&mut entity))
        }
    }
}

pub struct PrefabPlugin;

impl Plugin for PrefabPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<PrefabBank>();
    }
}
