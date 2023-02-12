use clap::arg;
use clap::crate_authors;
use clap::crate_version;
use clap::crate_description;
use clap::crate_name;
use clap::Command as ClapCommand;
use std::process::Command;
use std::process::Stdio;
use std::time::Instant;

fn app() -> ClapCommand {
    ClapCommand::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .about(crate_description!())
        .arg(arg!([command] ... "The command-line you want to execute.").required(true))
        .arg(arg!(--format <FORMAT> "Set output time format. Possible values: full, s, ms, us, µs, ns").default_value("full"))
}

fn main() {
    let app = app();

    let args = app.get_matches();

    let format: &str = args.get_one::<String>("format").map(|s| s.as_str()).unwrap();

    if !validate_format(format.into()) {
        println!("Unsupported format \"{}\".", format);
        std::process::exit(1);
    }

    let mut command_line: Vec<&str> = args.get_many::<String>("command").unwrap().map(|s| s.as_str()).collect();
    let (first_command, command_line) = command_line.split_first_mut().unwrap();

    let mut process = Command::new(first_command);
    process
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .args(command_line)
    ;

    let start = Instant::now();
    let child = process.spawn().unwrap().wait().unwrap();
    let duration = start.elapsed();

    let display: String = match format {
        "full" => humantime::format_duration(duration).to_string(),
        "s" => duration.as_secs().to_string(),
        "ms" => duration.as_millis().to_string(),
        "us" | "µs" => duration.as_micros().to_string(),
        "ns" => duration.as_nanos().to_string(),
        _ => panic!("Unsupported format \"{}\".", format),
    };

    println!("\n{}", display);

    std::process::exit(child.code().unwrap_or(255));
}

fn validate_format(format: String) -> bool {
    match format.as_str() {
        "full" | "s" | "ms" | "us" | "µs" | "ns" => true,
        _ => false
    }
}
