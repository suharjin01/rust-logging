fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod tests {
    use log::{debug, error, info, trace, warn};



    // Level
    #[test]
    fn test_log() {
        error!("This is an error");
        warn!("This is a warning");
        info!("This is an info");
        debug!("This is a debaug");
        trace!("This is a trace");
    }
}