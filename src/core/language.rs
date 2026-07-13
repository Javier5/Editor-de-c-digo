pub trait EditorPlugin {
    fn name(&self) -> String;
    fn execute(&self, buffer: &mut String);
}

pub struct PluginManager {
    pub plugins: Vec<Box<dyn EditorPlugin>>,
}

impl PluginManager {
    pub fn new() -> Self {
        Self { plugins: Vec::new() }
    }
}