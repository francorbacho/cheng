pub trait StaticEvaluationTracer {
    fn trace(what: &str, ev: i32);
}

pub struct NoopTracer;

impl StaticEvaluationTracer for NoopTracer {
    #[inline(always)]
    fn trace(_what: &str, _ev: i32) {
        // Just hope this gets optimized away.
    }
}

pub struct UciTracer;

impl StaticEvaluationTracer for UciTracer {
    fn trace(what: &str, ev: i32) {
        println!("{what} :: {ev}");
    }
}

pub struct LogTracer;

impl StaticEvaluationTracer for LogTracer {
    fn trace(what: &str, ev: i32) {
        log::info!(target: "ev", "{what} :: {ev}");
    }
}
