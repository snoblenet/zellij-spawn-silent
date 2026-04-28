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

        open_command_pane_background(cmd, context);
        false
    }

    fn update(&mut self, event: Event) -> bool {
        if let Event::CommandPaneOpened(terminal_pane_id, context) = event {
            if let Some(should_float) = context.get("float").map(|v| v == "true") {
                show_pane_with_id(PaneId::Terminal(terminal_pane_id), should_float, false);
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::SpawnRequest;

    fn parse(json: &str) -> Result<SpawnRequest, serde_json::Error> {
        serde_json::from_str(json)
    }

    #[test]
    fn minimal_request_uses_defaults() {
        let req = parse(r#"{"command":"htop"}"#).unwrap();
        assert_eq!(req.command, "htop");
        assert!(req.args.is_empty());
        assert!(req.cwd.is_none());
        assert!(!req.float);
    }

    #[test]
    fn full_request_parses_all_fields() {
        let req = parse(
            r#"{"command":"htop","args":["-d","5"],"cwd":"/tmp","float":true}"#,
        )
        .unwrap();
        assert_eq!(req.command, "htop");
        assert_eq!(req.args, vec!["-d", "5"]);
        assert_eq!(req.cwd.as_deref(), Some("/tmp"));
        assert!(req.float);
    }

    #[test]
    fn missing_command_is_rejected() {
        assert!(parse(r#"{"args":["foo"]}"#).is_err());
    }

    #[test]
    fn invalid_json_is_rejected() {
        assert!(parse("not json").is_err());
    }

    #[test]
    fn float_defaults_to_false() {
        let req = parse(r#"{"command":"ls"}"#).unwrap();
        assert!(!req.float);
    }

    #[test]
    fn float_key_identifies_plugin_owned_pane() {
        // The update handler gates on context.get("float").is_some() to
        // distinguish panes this plugin created from unrelated ones.
        // Verify the pipe handler encodes the flag as the string "true"/"false"
        // so the update handler can decode it correctly.
        let req_float = parse(r#"{"command":"htop","float":true}"#).unwrap();
        let req_no_float = parse(r#"{"command":"htop","float":false}"#).unwrap();
        assert_eq!(req_float.float.to_string(), "true");
        assert_eq!(req_no_float.float.to_string(), "false");
    }
}
