slint::include_modules!();

fn precedence(operator: &str) -> i8 {
    match operator {
        "+" | "-" => 1,
        "*" | "/" => 2,
        _ => -1,
    }
}

fn eval(num1: i32, num2: i32, operator: &str) -> i32 {
    match operator {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => num1 / num2,
        _ => 0,
    }
}

fn main() -> Result<(), slint::PlatformError> {
    let main_window: MainWindow = MainWindow::new()?;

    main_window.on_request_modify_expression({
        let main_window_handle: slint::Weak<MainWindow> = main_window.as_weak();
        move || {
            let main_window: MainWindow = main_window_handle.unwrap();
            let mut expression: String = main_window.get_expression().to_string();
            expression.pop();
            main_window.set_expression(expression.into());
        }
    });

    main_window.on_request_evaluate_expression({
        let main_window_handle: slint::Weak<MainWindow> = main_window.as_weak();
        move || {
            let main_window: MainWindow = main_window_handle.unwrap();
            let expression: String = main_window.get_expression().to_string();

            let mut operator_stack: Vec<String> = Vec::new();
            let mut output_stack: Vec<String> = Vec::new();

            for token in expression.split_whitespace().map(String::from) {
                if let Ok(_number) = token.parse::<i32>() {
                    output_stack.push(token);
                } else if operator_stack.is_empty() || precedence(&token) > precedence(&operator_stack.last().unwrap().to_string()) || token == "(" {
                    operator_stack.push(token);
                } else if token == ")" {
                    while operator_stack.last().unwrap().to_string() != "(" {
                        output_stack.push(operator_stack.pop().unwrap().to_string());
                    }
                    operator_stack.pop();
                } else {
                    while !operator_stack.is_empty() && precedence(&token) <= precedence(&operator_stack.last().unwrap().to_string()) {
                        output_stack.push(operator_stack.pop().unwrap().to_string());
                    }
                    operator_stack.push(token);
                }
            }

            while !operator_stack.is_empty() {
                output_stack.push(operator_stack.pop().unwrap().to_string());
            }

            let mut operand_stack: Vec<String> = operator_stack;

            for token in output_stack {
                if let Ok(_number) = token.parse::<i32>() {
                    operand_stack.push(token);
                } else {
                    let num2: i32 = operand_stack.pop().unwrap().to_string().parse::<i32>().unwrap();
                    let num1: i32 = operand_stack.pop().unwrap().to_string().parse::<i32>().unwrap();
                    operand_stack.push(eval(num1, num2, &token).to_string());
                }
            }
            main_window.set_expression(operand_stack[0].clone().into());
        }
    });

    main_window.run()
}
