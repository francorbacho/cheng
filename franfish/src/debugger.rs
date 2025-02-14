use crate::Evaluation;
use cheng::PseudoMove;

pub trait Debugger {
    fn on_leaf(&mut self, _ev: Evaluation) {}
    fn on_feed(&mut self, _pseudomove: &PseudoMove, _depth: usize) {}
}

#[derive(Default)]
pub struct NoDebugger;

impl Debugger for NoDebugger {}

#[derive(Default)]
pub struct LogAllDebugger {
    pub line: Vec<PseudoMove>,
    pub max_depth: Option<usize>,
}

impl Debugger for LogAllDebugger {
    fn on_leaf(&mut self, evaluation: Evaluation) {
        let line = self
            .line
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("  ");
        log::trace!("{line} :: {evaluation}");
    }

    fn on_feed(&mut self, pseudomove: &PseudoMove, depth: usize) {
        let max_depth = match self.max_depth {
            Some(max_depth) => max_depth,
            None => {
                self.max_depth = Some(depth);
                depth
            }
        };

        while max_depth - depth < self.line.len() {
            self.line.pop();
        }

        self.line.push(pseudomove.clone());
    }
}
