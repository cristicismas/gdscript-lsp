use crate::{logger, types::lsp_response::CompletionItem};

pub fn get_completion_items(file_contents: &str) -> Vec<CompletionItem> {
    let mut items: Vec<CompletionItem> = Vec::new();

    for line in file_contents.lines() {
        items.extend(get_completions_for_line(line))
    }

    return items;
}

fn get_completions_for_line(line: &str) -> Vec<CompletionItem> {
    let mut completion_items: Vec<CompletionItem> = vec![];

    if is_comment(line) {
        return vec![];
    }

    let assignment_completion = get_assignment_completion(line);
    match assignment_completion {
        Some(item) => completion_items.push(item),
        None => (),
    }

    let parameter_completions = get_parameter_completions(line);

    completion_items.extend(parameter_completions);

    logger::print_logs(format!("completion items: {:?}", completion_items));
    return completion_items;
}

fn is_comment(line: &str) -> bool {
    return line.trim().starts_with('#');
}

fn get_assignment_completion(line: &str) -> Option<CompletionItem> {
    let line_words: Vec<&str> = line.split_whitespace().collect();

    for index in 0..line_words.len() {
        let current_word = line_words[index];

        if index + 1 < line_words.len() {
            let next_word = line_words[index + 1];

            if let Some(variable_name) = try_get_variable(current_word, next_word) {
                let completion_item = CompletionItem {
                    label: Some(variable_name.to_string()),
                    detail: Some("Variable".to_string()),
                    documentation: None,
                };

                return Some(completion_item);
            }

            if let Some(function_name) = try_get_function(current_word, next_word) {
                let completion_item = CompletionItem {
                    label: Some(function_name.to_string()),
                    detail: Some("Function".to_string()),
                    documentation: None,
                };

                return Some(completion_item);
            }
        }
    }

    return None;
}

fn try_get_variable<'a>(current_word: &'a str, next_word: &'a str) -> Option<&'a str> {
    if current_word == "var" || current_word == "const" {
        let variable_name = remove_suffix(next_word, ":");

        return Some(variable_name);
    }

    return None;
}

fn try_get_function<'a>(current_word: &'a str, next_word: &'a str) -> Option<&'a str> {
    if current_word == "func" {
        let mut split_function_declaration = next_word.split('(');

        return split_function_declaration.next();
    }

    return None;
}

fn get_parameter_completions(line: &str) -> Vec<CompletionItem> {
    let mut completion_items: Vec<CompletionItem> = vec![];

    if !line.trim().starts_with("func ") {
        return vec![];
    }

    let open_parenthesis_index = match line.find('(') {
        Some(v) => v + 1,
        None => return vec![],
    };
    let closed_parenthesis_index = match line.find(')') {
        Some(v) => v,
        None => return vec![],
    };

    let parameters_substring = &line[open_parenthesis_index..closed_parenthesis_index];

    let parameters = parameters_substring.split(',');

    for parameter in parameters {
        let split_parameter: Vec<&str> = parameter.split(':').collect();

        let completion_item = CompletionItem {
            label: Some(split_parameter[0].to_string()),
            detail: Some("Parameter".to_string()),
            documentation: None,
        };

        completion_items.push(completion_item);
    }

    return completion_items;
}

fn remove_suffix<'a>(s: &'a str, suffix: &str) -> &'a str {
    match s.strip_suffix(suffix) {
        Some(s) => s,
        None => s,
    }
}
