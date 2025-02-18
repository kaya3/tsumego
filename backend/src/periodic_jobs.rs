use actix_web::rt::{spawn, time};
use std::time::Duration;

use crate::{
    result::Result,
    state::State,
};

pub fn start(state: State) {
    spawn(async move {
        // Once per hour
        let mut interval = time::interval(Duration::from_secs(60 * 60));
        
        loop {
            interval.tick().await;
            
            state.delete_all_expired_sessions()
                .await
                .report_if_err();
            
            state.delete_expired_challenges()
                .await
                .report_if_err();
        }
    });
}

trait ReportIfError {
    fn report_if_err(self);
}

impl <T> ReportIfError for Result<T> {
    fn report_if_err(self) {
        if let Err(e) = self {
            log::error!("Error in periodic job: {e:?}");
        }
    }
}
