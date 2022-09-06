use clap::Arg;
use clap::Command as ClapCommand;
use std::process::Command;
use std::process::Stdio;
use std::time::Instant;

fn main() {
    let app = ClapCommand::new("processtime")
        .arg(
            Arg::new("format")
                .long("format")
                .multiple_occurrences(false)
                .multiple_values(false)
                .takes_value(true)
                .default_value("full")
                .help("Set output time format. Possible values: full, s, ms, us, µs, ns")
        )
        .arg(
            Arg::new("command")
                .required(true)
                .multiple(true)
                .takes_value(true)
                .help("The command-line you want to execute.")
        )
    ;

    let args = app.get_matches();

    let format = args.value_of("format").unwrap_or_default();

    if !validate_format(format.into()) {
        println!("Unsupported format \"{}\".", format);
        std::process::exit(1);
    }

    let mut command_line = args.values_of("command").unwrap().collect::<Vec<&str>>();
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
