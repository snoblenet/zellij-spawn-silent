use zellij_tile::prelude::*;

#[derive(Default)]
struct SpawnSilent;

register_plugin!(SpawnSilent);

impl ZellijPlugin for SpawnSilent {
    fn load(&mut self, _configuration: BTreeMap<String, String>) {
        subscribe(&[EventNameList::CommandPaneOpened]);
    }

    fn pipe(&mut self, _pipe_message: PipeMessage) -> bool {
        false
    }

    fn update(&mut self, _event: Event) -> bool {
        false
    }
}
