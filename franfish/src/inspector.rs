static mut NODES_EVALUATED: usize = 0;

pub trait Inspector {
    #[inline(always)]
    fn on_evaluate() {}
    #[inline(always)]
    fn on_new_best_move() {}
    #[inline(always)]
    fn on_pruning() {}

    #[inline(always)]
    fn on_start() {}
    #[inline(always)]
    fn on_end() {}
}

pub struct NoInspector;

impl Inspector for NoInspector {}

pub struct DebugInspector;

impl Inspector for DebugInspector {
    fn on_evaluate() {
        unsafe { NODES_EVALUATED += 1 }
    }

    fn on_new_best_move() {
        println!("new best move found");
    }

    fn on_pruning() {
        println!("pruning");
    }

    fn on_start() {
        unsafe { NODES_EVALUATED = 0 }
    }

    fn on_end() {
        println!("finished evaluating nodes, total of {}", unsafe {
            NODES_EVALUATED
        });
    }
}
