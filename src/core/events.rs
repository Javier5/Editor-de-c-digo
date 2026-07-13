pub enum EditorEvent { TextChanged(usize), FileLoaded(String), FileSaved, } pub trait EventListener { fn on_event(&self, event: EditorEvent); }
