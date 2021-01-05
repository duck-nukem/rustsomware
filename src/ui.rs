use web_view::*;

pub fn render_ui(machine_id: &str) {
    let html_content = include_str!("../assets/ui.html");

    web_view::builder()
        .title("Bad luck :/")
        .content(Content::Html(html_content))
        .size(480, 400)
        .resizable(true)
        .debug(true)
        .user_data(())
        .invoke_handler(|webview, arg| {
            match arg {
                "render_machine_info" => webview.eval(&format!("renderMachineInfo('{}')", machine_id)),
                "attempt_unlock" => {
                    println!("Doing it ;)");
                    webview.eval(&format!("showUnlockResults('{}')", "nope"))
                }
                _ => unimplemented!(),
            }
        })
        .run()
        .unwrap();
}
