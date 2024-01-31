slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let main_window = MainWindow::new()?;

    main_window.on_request_modify_expression({
        let main_window_handle = main_window.as_weak();
        move || {
            let main_window = main_window_handle.unwrap();
            let mut expression = main_window.get_expression().to_string();
            expression.pop();
            main_window.set_expression(expression.into());
        }
    });

    main_window.run()
}
