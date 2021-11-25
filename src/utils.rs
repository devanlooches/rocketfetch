#[macro_export]
macro_rules! handle_error {
    ( $err:expr, $err_msg:expr, $help_msg:literal ) => {
        if let Ok(v) = $err {
            v
        } else {
            let r = $err.unwrap_err();
            if r != "" {
                UserFacingError::new($err_msg)
                    .help($help_msg)
                    .reason(r.to_string())
                    .print_and_exit();
            } else {
                UserFacingError::new($err_msg)
                    .help($help_msg)
                    .print_and_exit();
            }
            unreachable!()
        }
    };
    ( $err:expr, $err_msg:expr ) => {
        if let Ok(v) = $err {
            v
        } else {
            let r = $err.unwrap_err();
            if r.to_string() != "" {
                UserFacingError::new($err_msg)
                    .reason(r.to_string())
                    .print_and_exit();
            } else {
                UserFacingError::new($err_msg).print_and_exit();
            }
            unreachable!()
        }
    };
}
