use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(trailing_var_arg(true), required(true))]
    command: Vec<String>,

    #[arg(short, long, default_value_t = false)]
    /// toggle color, defaults to True
    color: bool,

    #[arg(short, long, default_value("./syscall.json"))]
    syscall_table_path: String,
}

impl Cli {
    pub fn args() {Cli::parse()};
}
