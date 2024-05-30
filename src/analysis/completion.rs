use crate::{logger, types::lsp_response::CompletionItem};

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

const RESERVED_KEYWORDS: &'static [&str] = &[
    "var",
    "const",
    "enum",
    "func",
    "extends",
    "class_name",
    "var",
    "Vector2",
    "Vector2f",
    "Vector3",
    "Vector3f",
    "Vector4",
    "Vector4f",
    "String",
    "StrinName",
    "NodePath",
    "Node",
    "Color",
    "float",
    "bool",
    "int",
    "null",
    "false",
    "true",
];

fn get_assignment_completion(line: &str) -> Option<CompletionItem> {
    let line_words: Vec<&str> = line.split_whitespace().collect();

    // We start looping from 1 because we are looking
    // for '=' signs (they should never be at index 0)
    for index in 1..line_words.len() {
        let current_word = line_words[index];

        if current_word == "=" {
            let prev_word = line_words[index - 1];

            // if prev_word is in reserved_keyword, get the word before that
            if RESERVED_KEYWORDS.contains(&prev_word) {
                let prev_prev_word = line_words[index - 2];

                let variable_name = remove_suffix(prev_prev_word, ":");

                let completion_item = CompletionItem {
                    label: Some(variable_name.to_string()),
                    detail: Some("Test LSP".to_string()),
                    documentation: None,
                };

                logger::print_logs(format!(
                    "complex pair: {} --- {}",
                    variable_name,
                    line_words[index + 1]
                ));

                return Some(completion_item);
            } else {
                let completion_item = CompletionItem {
                    label: Some(prev_word.to_string()),
                    detail: Some("Test LSP".to_string()),
                    documentation: None,
                };
                return Some(completion_item);
            }
        }
    }

    return None;
}

fn remove_suffix<'a>(s: &'a str, suffix: &str) -> &'a str {
    match s.strip_suffix(suffix) {
        Some(s) => s,
        None => s,
    }
}
