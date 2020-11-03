# logged-test

Proc-macro for setting up env-filter logger in tests. Works for normal 'fn' as well as 'async fn' with tokio runtime in tests. 

Very simple macro, but if you want it to replace the #[test] macro with #[logtest] then it has to be a procedural macro.
