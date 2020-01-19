#[cfg(test)]
mod tests {
    use super::*;
    //use log::info;
    use logged_test;

    #[logged_test::logtest]
    fn test_logged() {
        //info!("Test test test...");
    }
}
