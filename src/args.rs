use std::path::PathBuf;

#[derive(Debug)]
pub struct Args {
    main: String,
    consts_dump_path: Option<PathBuf>,
}

enum State<'a> {
    FlagOrParam,
    Arg(&'a str),
    MaybeArg(&'a str),
}

fn is_valid_param(s: &str) -> bool {
    match s {
        "-main" | "-c" => true,
        _ => false
    }
}

impl<'a> State<'a> {
    fn handle_param(&mut self, param: String) {
        match self {
            State::FlagOrParam | State::MaybeArg(_) => {
                match param.as_str() {
                    "-main" =>  {
                        *self = State::Arg("-main")
                    }
                    "-c" =>  {
                        *self = State::MaybeArg("-c")
                    }
                    _ => todo!(),
                }
            }
            State::Arg(p) => {
                eprintln!("expecting argument for parameter '{}', found parameter '{}'", p, param);
                std::process::exit(1);
            }
        }
    }

    fn handle_arg(&mut self, arg: String, args: &mut Args) {
        match self {
            State::Arg(param) => {
                match param {
                    &mut "-main" => args.main = arg,
                    _ => todo!(),
                }
            }
            State::MaybeArg(param) => {
                match param {
                    &mut "-c" => args.consts_dump_path = Some(PathBuf::from(arg)),
                    _ => todo!(),
                }
            }
            _ => todo!(),
        }
    }

}

pub fn parse() -> Args {
    let mut args = Args{main: "main".to_string(), consts_dump_path: None};
    let mut state = State::FlagOrParam;

    let args_count = std::env::args().len() - 2;
    for (i, arg) in std::env::args().skip(1).enumerate() {
        if i == args_count {
            if let State::Arg(param) = state {
                eprintln!("missing path to .fbd file or argument for parameter '{}'", param);
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
