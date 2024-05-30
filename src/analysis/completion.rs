use crate::{logger, types::lsp_response::CompletionItem};

pub fn get_completion_items(file_contents: &str) -> Vec<CompletionItem> {
    let mut items: Vec<CompletionItem> = Vec::new();

    logger::print_logs(format!("{}", file_contents));

    for line in file_contents.lines() {
        match get_completion_for_line(line) {
            Some(item) => items.push(item),
            None => (),
        };
    }

    logger::print_logs(format!("{:?}", items));

    return items;
}

fn get_completion_for_line(line: &str) -> Option<CompletionItem> {
    if is_comment(line) {
        return None;
    }

    return get_assignment_completions(line);
}

fn is_comment(line: &str) -> bool {
    return line.trim().starts_with('#');
}

fn get_assignment_completions(line: &str) -> Option<CompletionItem> {
    let line_words: Vec<&str> = line.split_whitespace().collect();

    // We start looping from 1 because we are only looking
    // for assignments (they can never be at index 0)
    for index in 1..line_words.len() {
        let current_word = line_words[index];

        if current_word == "=" {
            let variable_name = line_words[index - 1];

            let completion_item = CompletionItem {
                label: Some(variable_name.to_string()),
                detail: Some("Variable".to_string()),
                documentation: None,
            };

            return Some(completion_item);
        }
    }

    return None;
}
