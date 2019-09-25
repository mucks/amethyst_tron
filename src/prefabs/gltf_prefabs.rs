use std::collections::HashMap;
use std::fs::read_dir;

use crate::prefabs::GltfScenePrefabData;
use amethyst::{
    assets::{AssetStorage, Handle, Prefab, PrefabLoader, ProgressCounter, RonFormat},
    ecs::World,
    utils::application_root_dir,
};

#[derive(Default)]
pub struct GltfScenePrefabs {
    prefabs: HashMap<String, Handle<Prefab<GltfScenePrefabData>>>,
}

impl GltfScenePrefabs {
    pub fn insert(
        &mut self,
        creature_type: String,
        prefab_handle: Handle<Prefab<GltfScenePrefabData>>,
    ) {
        self.prefabs.insert(creature_type, prefab_handle);
    }

    pub fn get_prefab(&self, gltf_scene: &str) -> Option<&Handle<Prefab<GltfScenePrefabData>>> {
        self.prefabs.get(gltf_scene)
    }

    pub fn get_prefabs(&self) -> &HashMap<String, Handle<Prefab<GltfScenePrefabData>>> {
        &self.prefabs
    }

    pub fn set_prefabs(&mut self, prefabs: HashMap<String, Handle<Prefab<GltfScenePrefabData>>>) {
        self.prefabs = prefabs;
    }
}

fn make_name(subdirectory: &str, entry: &std::fs::DirEntry) -> String {
    let path_buffer = entry.path();
    let filename = path_buffer.file_name().unwrap();
    format!("{}{}", subdirectory, filename.to_str().unwrap())
}

pub fn init_prefabs(world: &mut World) -> ProgressCounter {
    let mut progress_counter = ProgressCounter::new();
    let prefab_iter = {
        let prefab_dir_path = application_root_dir()
            .unwrap()
            .join("assets")
            .join("prefabs")
            .join("gltf_scenes");
        let prefab_iter = read_dir(prefab_dir_path).unwrap();
        prefab_iter.map(|prefab_dir_entry| {
            world.exec(|loader: PrefabLoader<'_, GltfScenePrefabData>| {
                loader.load(
                    make_name("prefabs/gltf_scenes/", &prefab_dir_entry.unwrap()),
                    RonFormat,
                    &mut progress_counter,
                )
            })
        })
    };

    let mut gltf_scene_prefabs = GltfScenePrefabs::default();
    for (count, prefab) in prefab_iter.enumerate() {
        gltf_scene_prefabs.insert("temp_prefab_".to_string() + &count.to_string(), prefab);
    }
    world.add_resource(gltf_scene_prefabs);

    progress_counter
}
pub fn update_prefabs(world: &mut World) {
    let updated_prefabs = {
        let gltf_scene_prefabs = world.read_resource::<GltfScenePrefabs>();
        let prefabs = gltf_scene_prefabs.get_prefabs();
        let mut prefab_resource =
            world.write_resource::<AssetStorage<Prefab<GltfScenePrefabData>>>();
        let mut new_prefabs = HashMap::new();
        for (_key, handle) in prefabs.iter() {
            if let Some(prefab) = prefab_resource.get_mut(handle) {
                if let Some(prefab_data) = prefab.entity(0) {
                    let name = prefab_data
                        .data()
                        .unwrap()
                        .name
                        .as_ref()
                        .unwrap()
                        .name
                        .to_string();
                    new_prefabs.insert(name, handle.clone());
                }
            }
        }
        new_prefabs
    };
    world
        .write_resource::<GltfScenePrefabs>()
        .set_prefabs(updated_prefabs);
}
