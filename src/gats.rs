use std::fmt::Debug;

/// Public-facing API – users configure the analysis
/// through this trait.
pub trait Config {}

/// If they have a reference to the config, its still
/// a config.
impl<P: Config + ?Sized> Config for &P {}

#[derive(Debug)]
pub struct State;
/// A factory to produce an analysis given some config (self) and state.
pub trait Factory {
    type Analysis<'a>
    where
        Self: 'a;
    fn manufacture<'a>(&'a self, state: &'a State) -> Self::Analysis<'a>;
}

#[derive(Debug)]
pub struct Analysis<'a, Config> {
    state: &'a State,
    config: &'a Config,
}

/// Given any user-defined `Config`, manufacture an effectful analysis.
impl<C: Config> Factory for C {
    type Analysis<'a> = Analysis<'a, Self> where Self: 'a;
    fn manufacture<'a>(&'a self, state: &'a State) -> Self::Analysis<'a> {
        Analysis {
            state,
            config: self,
        }
    }
}

/// No Config for this analysis, meaning it will not run.
pub struct NoConfig;
pub struct NoAnalysis;
impl Factory for NoConfig {
    type Analysis<'a> = NoAnalysis;
    fn manufacture<'a>(&'a self, _: &'a State) -> Self::Analysis<'a> {
        NoAnalysis
    }
}

// pub fn run_analysis<'a, F>(code: &[u8], f: F)
// where
//     <F as Factory>::Analysis<'a>: Debug,
//     F: Config + 'a,
// {
//     let state = State;
//     for _function in code {
//         let _analysis = f.manufacture(&state);
//         // todo!("run analysis and collect results");
//         // println!("{:?}", _analysis);
//     }
// }

pub fn run_analysis<F: Factory>(code: &[u8], f: F) {
    let state = State;
    for _function in code {
        let _analysis = f.manufacture(&state);
        // todo!("run analysis and collect results");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factory_gat_pattern() {
        #[derive(Debug)]
        struct MyConfig;

        impl Config for MyConfig {}

        let my_config = MyConfig;
        run_analysis(b"\0wasm", &my_config as &dyn Config);
    }
}
