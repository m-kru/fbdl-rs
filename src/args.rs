use std::fmt;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Args {
    main: String,
    consts_dump_path: Option<PathBuf>,
    reg_results_path: Option<PathBuf>,
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

fn is_valid_param(s: &str) -> bool {
    match s {
        "-main" | "-c" | "-r" => true,
        _ => false,
    }
}

impl State {
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
        main: "main".to_string(),
        consts_dump_path: None,
        reg_results_path: None,
    };
    let mut state = State::FlagOrParam;

    let args_count = std::env::args().len() - 2;
    for (i, arg) in std::env::args().skip(1).enumerate() {
        if i == args_count {
            if let State::Arg(param) = state {
                eprintln!(
                    "missing path to .fbd file or argument for parameter '{}'",
                    param
                );
                std::process::exit(1);
            }
        }

        if is_valid_param(&arg) {
            state.handle_param(arg)
        } else {
            state.handle_arg(arg, &mut args)
        }
    }

    args
}
