use actix_web::rt::{spawn, time};
use std::time::Duration;

use crate::{
    model::{Session, User},
    result::Result,
    state::State,
};

pub fn start(state: State) {
    spawn(async move {
        // Once per hour
        let mut interval = time::interval(Duration::from_secs(60 * 60));
        
        loop {
            interval.tick().await;
            
            Session::delete_all_expired(&state)
                .await
                .report_if_err();
            
            User::delete_unverified_expired(&state)
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
