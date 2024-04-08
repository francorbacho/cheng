pub trait StaticEvaluationTracer {
    fn info(what: &str);

    fn trace(what: &str, ev: i32);
}

pub struct NoopTracer;

impl StaticEvaluationTracer for NoopTracer {
    #[inline(always)]
    fn info(_what: &str) {
        // Just hope this gets optimized away.
    }

    #[inline(always)]
    fn trace(_what: &str, _ev: i32) {
        // Just hope this gets optimized away.
    }
}

pub struct UciTracer;

impl StaticEvaluationTracer for UciTracer {
    #[inline(always)]
    fn info(_what: &str) {
        // Not used currently. Implement if necessary
        unreachable!()
    }

    #[inline(always)]
    fn trace(what: &str, ev: i32) {
        println!("{what} :: {ev}");
    }
}

pub struct LogTracer;

impl StaticEvaluationTracer for LogTracer {
    #[inline(always)]
    fn info(what: &str) {
        log::info!("{what}");
    }

    #[inline(always)]
    fn trace(what: &str, ev: i32) {
        log::info!(target: "ev", "{what} :: {ev}");
    }
}
