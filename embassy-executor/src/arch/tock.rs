#[cfg(feature = "executor-interrupt")]
compile_error!("`executor-interrupt` is not supported with `arch-tock`.");

#[cfg(feature = "executor-thread")]
pub use thread::*;

#[cfg(feature = "executor-thread")]
mod thread {
    use core::{marker::PhantomData, sync::atomic::Ordering};
    pub use embassy_executor_macros::main_tock as main;
    use portable_atomic::AtomicBool;

    use crate::{raw, Spawner};
    use libtock_platform::Syscalls;

    static SIGNAL_WORK: AtomicBool = AtomicBool::new(false);

    #[export_name = "__pender"]
    fn __pender(_context: *mut ()) {
        SIGNAL_WORK.store(true, Ordering::SeqCst);
    }

    /// Tock Executor
    pub struct Executor<S: Syscalls> {
        inner: raw::Executor,
        syscalls: PhantomData<S>,
        not_send: PhantomData<*mut ()>,
    }

    impl<S: Syscalls> Executor<S> {
        /// Thread mode executor, using `yield` and `upcalls`.
        ///
        /// This executor uses `yield` to pass control to the kernel when there is no more
        /// work to do, allowing the event handlers to run, and awake tasks.
        pub fn new() -> Self {
            Self {
                inner: raw::Executor::new(&mut () as *mut ()),
                syscalls: PhantomData,
                not_send: PhantomData,
            }
        }

        /// Run the executor.
        ///
        /// The `init` closure is called with a [`Spawner`] that spawns tasks on
        /// this executor. Use it to spawn the initial task(s). After `init` returns,
        /// the executor starts running the tasks.
        ///
        /// To spawn more tasks later, you may keep copies of the [`Spawner`] (it is `Copy`),
        /// for example by passing it as an argument to the initial tasks.
        ///
        /// This function requires `&'static mut self`. This means you have to store the
        /// Executor instance in a place where it'll live forever and grants you mutable
        /// access. There's a few ways to do this:
        ///
        /// - a [StaticCell](https://docs.rs/static_cell/latest/static_cell/) (safe)
        /// - a `static mut` (unsafe)
        /// - a local variable in a function you know never returns (like `fn main() -> !`), upgrading its lifetime with `transmute`. (unsafe)
        ///
        /// This function never returns.
        pub fn run(&'static mut self, init: impl FnOnce(Spawner)) -> ! {
            init(self.inner.spawner());

            loop {
                if !SIGNAL_WORK.swap(false, Ordering::SeqCst) {
                    S::yield_wait();
                } else {
                    unsafe { self.inner.poll() };
                    S::yield_no_wait();
                }
            }
        }
    }
}
