use zellij_tile::prelude::*;

#[derive(Default)]
struct SpawnSilent;

register_plugin!(SpawnSilent);

#[derive(serde::Deserialize)]
struct SpawnRequest {
    command: String,
    #[serde(default)]
    args: Vec<String>,
    cwd: Option<String>,
    #[serde(default)]
    float: bool,
    direction: Option<String>,
}

impl ZellijPlugin for SpawnSilent {
    fn load(&mut self, _configuration: BTreeMap<String, String>) {
        subscribe(&[EventNameList::CommandPaneOpened]);
    }

    fn pipe(&mut self, pipe_message: PipeMessage) -> bool {
        let Some(payload) = pipe_message.payload else {
            return false;
        };

        let req: SpawnRequest = match serde_json::from_str(&payload) {
            Ok(r) => r,
            Err(_) => return false,
        };

        let mut cmd = CommandToRun::new(std::path::PathBuf::from(&req.command));
        cmd.args = req.args;
        cmd.cwd = req.cwd.map(std::path::PathBuf::from);

        let mut context = BTreeMap::new();
        context.insert("float".to_string(), req.float.to_string());
        if let Some(dir) = req.direction {
            context.insert("direction".to_string(), dir);
        }

        open_command_pane_background(cmd, context);
        false
    }

    fn update(&mut self, event: Event) -> bool {
        if let Event::CommandPaneOpened(terminal_pane_id, context) = event {
            let should_float = context
                .get("float")
                .map(|v| v == "true")
                .unwrap_or(false);
            show_pane_with_id(PaneId::Terminal(terminal_pane_id), should_float, false);
        }
        false
    }
}
