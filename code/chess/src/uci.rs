use std::{convert::Infallible, str::FromStr, process::Output};

#[derive(Debug)]
pub(crate) enum UciOutput {
    Bestmove(String),
    InfoDepth(i64, String),
    Ok(),
    Unknown(String),
}

impl FromStr for UciOutput {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Infallible> {
        let output = match s {
            "uciok" => UciOutput::Ok(),
            output if output.starts_with("bestmove") => {
                let tokens = output.split(" ").collect::<Vec<&str>>();

                UciOutput::Bestmove(tokens[1].to_string())
            }
            output if output.starts_with("info depth") && output.contains("seldepth") => {
                // println!("{}", &s);
                let tokens = output.split(" ").collect::<Vec<&str>>();
                let score :i64 = tokens[9].parse().unwrap();

                UciOutput::InfoDepth(score, s.to_string())
            }
            _ => UciOutput::Unknown(s.to_string()),
        };
        Ok(output)
    }
}

impl UciOutput {
    pub(crate) fn is_ok(&self) -> bool {
        match self {
            UciOutput::Ok() => true,
            _ => false,
        }
    }

    pub(crate) fn is_bestmove(&self) -> bool {
        match self {
            UciOutput::Bestmove(..) => true,
            _ => false,
        }
    }

    pub(crate) fn score(&self) -> i64 {
        match self {
            UciOutput::InfoDepth(score, _) => *score,
            _ => panic!("score"),
        }
    }

    pub(crate) fn bestmove(&self) -> String {
        match self {
            UciOutput::Bestmove(s) => s.to_string(),
            _ => panic!("score"),
        }
    }

}
