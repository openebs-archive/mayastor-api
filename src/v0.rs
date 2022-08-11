use std::str::FromStr;

#[derive(Debug)]
pub enum Error {
    ParseError,
}

impl From<()> for Null {
    fn from(_: ()) -> Self {
        Self {}
    }
}

impl FromStr for NvmeAnaState {
    type Err = Error;
    fn from_str(state: &str) -> Result<Self, Self::Err> {
        match state {
            "optimized" => Ok(Self::NvmeAnaOptimizedState),
            "non_optimized" => Ok(Self::NvmeAnaNonOptimizedState),
            "inaccessible" => Ok(Self::NvmeAnaInaccessibleState),
            _ => Err(Error::ParseError),
        }
    }
}

include!(concat!(env!("OUT_DIR"), "/mayastor.rs"));
