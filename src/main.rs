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

fn infix_to_postfix(expression: String) -> Vec<String> {
    let mut operator_stack: Vec<String> = Vec::new();
    let mut output_stack: Vec<String> = Vec::new();
    for token in expression.split_whitespace().map(String::from) {
        if let Ok(_number) = token.parse::<i32>() {
            output_stack.push(token);
        } else if operator_stack.is_empty()
            || precedence(&token)
                > precedence(
                    &operator_stack
                        .last()
                        .map(|s: &String| s.as_str())
                        .unwrap_or_default(),
                )
            || token == "("
        {
            operator_stack.push(token);
        } else if token == ")" {
            while operator_stack
                .last()
                .map(|s: &String| s.as_str())
                .unwrap_or_default()
                != "("
            {
                output_stack.push(operator_stack.pop().unwrap_or_default());
            }
            operator_stack.pop();
        } else {
            while !operator_stack.is_empty()
                && precedence(&token)
                    <= precedence(
                        &operator_stack
                            .last()
                            .map(|s: &String| s.as_str())
                            .unwrap_or_default(),
                    )
            {
                output_stack.push(operator_stack.pop().unwrap_or_default());
            }
            operator_stack.push(token);
        }
    }

    while !operator_stack.is_empty() {
        output_stack.push(operator_stack.pop().unwrap_or_default());
    }
    return output_stack;
}

fn postfix_eval(postfix_stack: Vec<String>) -> i32 {
    let mut operand_stack: Vec<String> = Vec::new();
    for token in postfix_stack {
        if let Ok(_number) = token.parse::<i32>() {
            operand_stack.push(token);
        } else {
            let num2: i32 = operand_stack
                .pop()
                .map(|s: String| s.parse().unwrap_or_default())
                .unwrap_or_default();
            let num1: i32 = operand_stack
                .pop()
                .map(|s: String| s.parse().unwrap_or_default())
                .unwrap_or_default();
            operand_stack.push(eval(num1, num2, &token).to_string());
        }
    }
    return operand_stack.get(0).unwrap_or(&String::from("0")).parse::<i32>().unwrap_or_default();
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

            let postfix_stack: Vec<String> = infix_to_postfix(expression);
            let ans: i32 = postfix_eval(postfix_stack);        

            main_window.set_expression(ans.to_string().into());
        }
    });

    main_window.run()
}
