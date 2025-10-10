use raylib::prelude::*;
use glob::glob;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug)]
pub struct Assets<'a> {
    pub textures: HashMap<Arc<str>, Texture2D>,
    pub sounds: HashMap<Arc<str>, Sound<'a>>,
    pub music: HashMap<Arc<str>, Music<'a>>,
    pub models: HashMap<Arc<str>, Model>,
}

impl<'a> Assets<'a> {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
            sounds: HashMap::new(),
            music: HashMap::new(),
            models: HashMap::new(),
        }
    }

    pub fn with_textures(mut self, dir: &str, rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        for entry in glob(&format!("{}/*.png", dir)).expect("Failed to load textures") {
            match entry {
                Ok(path) => {
                    let tex = rl.load_texture(thread, path.to_str().unwrap()).unwrap();
                    self.textures.insert(Arc::from(path.file_stem().unwrap().to_str().unwrap()), tex);
                },
                Err(e) => println!("{:?}", e),
            }
        }

        self
    }

    pub fn with_sounds(mut self, dir: &str, ra: &'a RaylibAudio) -> Self {
        for entry in glob(&format!("{}/*.wav", dir)).expect("Failed to load sounds") {
            match entry {
                Ok(path) => {
                    let sound = ra.new_sound(dir).unwrap();
                    self.sounds.insert(Arc::from(path.file_stem().unwrap().to_str().unwrap()), sound);
                },
                Err(e) => println!("{:?}", e),
            }
        }

        self
    }

    pub fn with_music(mut self, dir: &str, ra: &'a RaylibAudio) -> Self {
        for entry in glob(&format!("{}/*.wav", dir)).expect("Failed to load music") {
            match entry {
                Ok(path) => {
                    let music = ra.new_music(dir).unwrap();
                    self.music.insert(Arc::from(path.file_stem().unwrap().to_str().unwrap()), music);
                },
                Err(e) => println!("{:?}", e),
            }
        }

        self
    }

    pub fn with_models(mut self, dir: &str, rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        for entry in glob(&format!("{}/*.glb", dir)).expect("Failed to load models") {
            match entry {
                Ok(path) => {
                    let model = rl.load_model(thread, dir).unwrap();
                    self.models.insert(Arc::from(path.file_stem().unwrap().to_str().unwrap()), model);
                },
                Err(e) => println!("{:?}", e),
            }
        }

        self
    }

    pub fn get_tex(&self, name: &Arc::<str>) -> &Texture2D {
        &self.textures.get(name).unwrap_or(&self.textures["missing"])
    }

    pub fn get_tex_str(&self, name: &str) -> &Texture2D {
        self.get_tex(&Arc::from(name))
    }
}