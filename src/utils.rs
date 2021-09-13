use user_error::{UserFacingError, UFE};

pub fn handle_error_result<T, E: std::fmt::Display>(
    error: Result<T, E>,
    error_msg: Option<&str>,
    help_msg: Option<&str>,
) -> T {
    if let Err(real_error) = error {
        if let Some(user_error_msg) = error_msg {
            if let Some(user_help_msg) = help_msg {
                UserFacingError::new(user_error_msg)
                    .help(user_help_msg)
                    .print_and_exit();
                unreachable!()
            } else {
                UserFacingError::new(user_error_msg).print_and_exit();
                unreachable!()
            }
        } else {
            UserFacingError::new(real_error.to_string()).print_and_exit();
            unreachable!()
        }
    } else if let Ok(result) = error {
        result
    } else {
        unreachable!()
    }
}

pub fn handle_error_option<T>(
    error: Option<T>,
    error_msg: &str,
    help_msg: Option<&str>,
) -> T {
    if error.is_none() {
        if let Some(user_help_msg) = help_msg {
            UserFacingError::new(error_msg)
                .help(user_help_msg)
                .print_and_exit();
            unreachable!()
        } else {
            UserFacingError::new(error_msg).print_and_exit();
            unreachable!()
        }
    } else if let Some(result) = error {
        result
    } else {
        unreachable!()
    }
}
