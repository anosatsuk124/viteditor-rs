use std::ffi::OsString;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Args {
    pub filepath: Option<OsString>,
    pub help: Option<bool>,
}

pub fn args_parse() -> Result<Args, ()> {
    let args: Vec<String> = std::env::args().collect();

    match args.len() - 1 {
        1 => Ok(Args {
            filepath: Some(OsString::from(&args[1])),
            help: None,
        }),
        _ => Err(()),
    }
}
