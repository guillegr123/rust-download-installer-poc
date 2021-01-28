use web_view::*;
use std::io;
use std::fs::File;
use std::thread;
use std::env;

fn main() {
    web_view::builder()
        .title("Installer")
        .content(Content::Url("https://www.google.com"))
        .size(480, 200)
        .resizable(false)
        .debug(true)
        .user_data(())
        .invoke_handler(|_webview, arg| {
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
