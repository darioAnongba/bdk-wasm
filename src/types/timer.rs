use std::time::Duration;

use bdk_esplora::esplora_client::Sleeper;
use gloo_timers::future::{sleep, TimeoutFuture};

use crate::utils::SendSyncWrapper;

#[derive(Clone)]
pub struct BrowserSleeper;

impl Sleeper for BrowserSleeper {
    type Sleep = SendSyncWrapper<TimeoutFuture>;

    fn sleep(dur: Duration) -> Self::Sleep {
        SendSyncWrapper(sleep(dur))
    }
}
