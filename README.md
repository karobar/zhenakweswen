## Automated Tests
This project runs automated tests and lints before each
git push action. If you want to trigger tests manually,
run:

    rustup update
    cargo test -- --nocapture
