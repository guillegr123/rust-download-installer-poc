use web_view::*;
use std::io;
use std::fs::File;
use std::thread;
use std::env;

fn main() {
    let html_content = format!(
        r#"
		<!doctype html>
		<html>
			<head>
				{styles}
			</head>
            <body>
                {scripts}
                <div class="page_wrap">
                    <div id="progress_bar" class="meter red">
                        <span style="width: 100%"></span>
                    </div>
                    <div>
                        <button id="btn_install">Install</button>
                    </div>
                </div>
			</body>
		</html>
		"#,
        styles = inline_style(include_str!("styles.css")),
        scripts = inline_script(include_str!("app.js")),
    );
	
    web_view::builder()
        .title("Installer")
        .content(Content::Html(html_content))
        .size(480, 200)
        .resizable(false)
        .debug(true)
        .user_data(())
        .invoke_handler(|webview, arg| {
            {
                match arg {
                    "install" => {
                        //println!("hello there!");

                        //let (sender, receiver) = futures::channel::oneshot::channel();

                        thread::spawn(|| {
                            // The complete method resolves a values.
                            expensive_computation();

                            //sender.send(0).unwrap();
                            std::process::exit(0);
                        });

                        /*
                        receiver.map(|result| async move {
                            webview.eval(&format!("downloadCompleted()")).expect("Error invoking JS");
                            result
                        });
                        */
                    }
                    _ => unimplemented!(),
                };
            }

            Ok(())
        })
        .run()
        .unwrap();
}

fn inline_style(s: &str) -> String {
    format!(r#"<style type="text/css">{}</style>"#, s)
}

fn inline_script(s: &str) -> String {
    format!(r#"<script type="text/javascript">{}</script>"#, s)
}

fn expensive_computation() {
    let mut reader = ureq::get("https://mauricio-is-testing.s3.amazonaws.com/comm.io.app.zip")
    .call()
    .expect("Error sending request")
    .into_reader();

    let home: String = env::var("HOME").expect("Error reading ennvironment var").to_owned();
    let relative_path: &str = "/Downloads/comm.io.app.zip";
    let full_path = home + relative_path;
    let mut out = File::create(full_path)
        .expect("Error creating file");
    io::copy(&mut reader, &mut out).expect("Error writing file");
}
