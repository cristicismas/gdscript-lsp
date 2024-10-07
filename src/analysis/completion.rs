use crate::types::lsp_response::CompletionItem;

pub fn get_completion_items(file_contents: &str) -> Vec<CompletionItem> {
    let mut items: Vec<CompletionItem> = Vec::new();

    for line in file_contents.lines() {
        match get_completion_for_line(line) {
            Some(item) => items.push(item),
            None => (),
        };
    }

    return items;
}

fn get_completion_for_line(line: &str) -> Option<CompletionItem> {
    if is_comment(line) {
        return None;
    }

    return get_assignment_completion(line);
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

fn remove_suffix<'a>(s: &'a str, suffix: &str) -> &'a str {
    match s.strip_suffix(suffix) {
        Some(s) => s,
        None => s,
    }
}
