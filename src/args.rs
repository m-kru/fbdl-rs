use std::fmt;
use std::path::PathBuf;

fn print_help() {
    println!(
        "Functional Bus Description Language compiler front-end.
Version: {}

Usage:
  fbdl [flags] [parameters] /path/to/main/fbd/file

Flags:
  -help           Display help.
  -version        Display version.
  -debug          Print debug messages.
  -add-timestamp  Add bus generation timestamp.
                  The timestamp is not included in the ID calculation.
                  The timestamp is always placed at the end of the bus address space.

Parameters:
  -main name  Name of the main bus. Useful for testbenches.
  -p [path]   Dump parse results to a file (default path is prs.txt).
  -i [path]   Dump instantiation results to a file (default path is ins.txt).
  -r [path]   Dump registerification results to a file (default path is reg.json).
  -c [path]   Dump packages constants to a file (default path is const.json). ",
        env!("CARGO_PKG_VERSION")
    );
}

fn is_valid_flag(f: &str) -> bool {
    match f {
        "-add-timestamp" => true,
        _ => false,
    }
}

fn is_valid_param(p: &str) -> bool {
    match p {
        "-main" | "-c" | "-r" => true,
        _ => false,
    }
}

#[derive(Debug)]
pub struct Args {
    // Flags
    add_timestamp: bool,

    // Parameters
    main: String,
    consts_dump_path: Option<PathBuf>,
    reg_results_path: Option<PathBuf>,

    main_file_path: PathBuf,
}

enum Param {
    Main,
    RegResultsPath,
    ConstsDumpPath,
}

impl fmt::Display for Param {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Param::Main => write!(f, "-main"),
            Param::ConstsDumpPath => write!(f, "-c"),
            Param::RegResultsPath => write!(f, "-r"),
        }
    }
}

enum State {
    FlagOrParam,
    Arg(Param),
    MaybeArg(Param),
}

impl State {
    fn handle_flag(&mut self, flag: String, args: &mut Args) {
        match self {
            State::FlagOrParam | State::MaybeArg(_) => match flag.as_str() {
                "-add-timestamp" => {
                    *self = State::FlagOrParam;
                    args.add_timestamp = true;
                }
                _ => todo!(),
            },
            State::Arg(p) => {
                eprintln!(
                    "expected argument for parameter '{}', found flag '{}'",
                    p, flag
                );
                std::process::exit(1);
            }
        }
    }

    fn handle_param(&mut self, param: String) {
        match self {
            State::FlagOrParam | State::MaybeArg(_) => match param.as_str() {
                "-main" => *self = State::Arg(Param::Main),
                "-c" => *self = State::MaybeArg(Param::ConstsDumpPath),
                "-r" => *self = State::MaybeArg(Param::RegResultsPath),
                _ => todo!(),
            },
            State::Arg(p) => {
                eprintln!(
                    "expected argument for parameter '{}', found parameter '{}'",
                    p, param
                );
                std::process::exit(1);
            }
        }
    }

    fn handle_arg(&mut self, arg: String, args: &mut Args) {
        match self {
            State::FlagOrParam => {
                eprintln!("expected flag or paramter, found '{}'", arg);
                std::process::exit(1);
            }
            State::Arg(param) => match param {
                Param::Main => args.main = arg,
                _ => todo!(),
            },
            State::MaybeArg(param) => match param {
                Param::Main => panic!("should never happen"),
                Param::ConstsDumpPath => args.consts_dump_path = Some(PathBuf::from(arg)),
                Param::RegResultsPath => args.reg_results_path = Some(PathBuf::from(arg)),
            },
        }
    }
}

pub fn parse() -> Args {
    let mut args = Args {
        add_timestamp: false,
        main: "main".to_string(),
        consts_dump_path: None,
        reg_results_path: None,
        main_file_path: PathBuf::new(),
    };
    let mut state = State::FlagOrParam;

    let args_len = std::env::args().len();

    if args_len == 1 {
        eprintln!("fbdl expects at least one argument, check 'fbdl -help'");
        std::process::exit(1);
    }

    for (i, arg) in std::env::args().skip(1).enumerate() {
        if arg == "-help" {
            print_help();
            std::process::exit(0);
        }

        // Handle last argument in a different way.
        if i == args_len - 2 {
            if let State::Arg(param) = state {
                eprintln!(
                    "missing path to .fbd file or argument for parameter '{}'",
                    param
                );
                std::process::exit(1);
            } else {
                if is_valid_flag(&arg) {
                    eprintln!("expected path to .fbd file, found flag '{}'", arg);
                    std::process::exit(1);
                } else if is_valid_param(&arg) {
                    eprintln!("expected path to .fbd file, found parameter '{}'", arg);
                    std::process::exit(1);
                } else {
                    args.main_file_path = PathBuf::from(arg);
                }
            }
        } else {
            if is_valid_flag(&arg) {
                state.handle_flag(arg, &mut args)
            } else if is_valid_param(&arg) {
                state.handle_param(arg)
            } else {
                state.handle_arg(arg, &mut args)
            }
        }
    }

    args
}
