use clap::{App, Arg, SubCommand};
use rand::Rng;
use std::fs;
use std::path::Path;

fn main() {
    let matches = App::new("edtx_generate_a_secu")
        .version("1.0")
        .author("Your Name")
        .about("Generate a secure CLI tool")
        .subcommand(
            SubCommand::with_name("generate")
                .about("Generate a new CLI tool")
                .arg(
                    Arg::with_name("tool_name")
                        .help("The name of the tool")
                        .required(true)
                        .index(1),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("generate") {
        if let Some(tool_name) = matches.value_of("tool_name") {
            let mut rng = rand::thread_rng();
            let password: String = rng
                .sample_iter(&rand::distributions::Alphanumeric)
                .take(12)
                .map(char::from)
                .collect();

            let tool_path = format!("{}.rs", tool_name);
            let tool_content = format!(
                r#"
use clap::{App, Arg};
fn main() {
    let matches = App::new("{}")
        .version("1.0")
        .author("Your Name")
        .about("My new CLI tool")
        .arg(Arg::with_name("password")
             .long("password")
             .takes_value(true)
             .required(true))
        .get_matches();

    if let Some(password) = matches.value_of("password") {{
        if password == "{}" {{
            println!("Welcome to your new CLI tool!");
        }} else {{
            println!("Incorrect password");
        }}
    }}
}
"#,
                tool_name, password
            );

            fs::write(tool_path, tool_content).expect("Unable to write file");
            println!("Tool generated: {}", tool_path);
            println!("Password: {}", password);
        }
    }
}