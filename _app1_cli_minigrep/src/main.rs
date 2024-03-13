use std::env;
use std::process; //will help us exit the program without panic
use _app1_cli_minigrep::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err |{
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = _app1_cli_minigrep::run(config) {   //if execution of run(conf) call an Err -> there is block {}
        eprintln!("Application error: {e}");
        process::exit(1)
    }
}

// cargo run lorem poem.txt
// cargo run lorem poem.txt > output.txt

//$env:CASE_INSENSITIVE = $true         linux// export CASE_INSENSITIVE=true -> set true
//Remove-Item Env:\CASE_INSENSITIVE     linux// unset CASE_INSENSITIVE