use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

// Wrap a future that is not `Send` or `Sync` and make it `Send` and `Sync`
pub struct SendSyncWrapper<F>(pub F);

unsafe impl<F> Send for SendSyncWrapper<F> {}
unsafe impl<F> Sync for SendSyncWrapper<F> {}

impl<F> Future for SendSyncWrapper<F>
where
    F: Future,
{
    type Output = F::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // SAFETY: Since we're in a single-threaded WASM environment, this is safe.
        unsafe {
            let this = self.get_unchecked_mut();
            Pin::new_unchecked(&mut this.0).poll(cx)
        }
    }
}
