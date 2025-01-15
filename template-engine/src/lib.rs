#[derive(PartialEq, Debug)]
pub enum ContentType {
    Literal(String),
    TemplateVariable(ExpressionData),
    Tag(TagType),
    Unrecognized,
}

#[derive(PartialEq, Debug)]
pub enum TagType {
    ForTag,
    IfTag,
}

#[derive(PartialEq, Debug)]
pub struct ExpressionData {
    pub head: Option<String>,
    pub variable: String,
    pub tail: Option<String>,
}

pub fn get_content_type(input_line: &str) -> ContentType {
    let is_tag_expression = check_matching_pair(&input_line, "{%", "%}");

    let is_for_tag = (check_symbol_string(&input_line, "for")
        && check_symbol_string(&input_line, "in"))
        || check_symbol_string(&input_line, "endfor");

    let is_if_tag =
        check_symbol_string(&input_line, "if") || check_symbol_string(&input_line, "endif");

    let is_template_variable = check_matching_pair(&input_line, "{{", "}}");
    let return_val;

    if is_tag_expression && is_for_tag {
        return_val = ContentType::Tag(TagType::ForTag);
    } else if is_tag_expression && is_if_tag {
        return_val = ContentType::Tag(TagType::IfTag);
    } else if is_template_variable {
        let content = get_expression_data(&input_line);
        return_val = ContentType::TemplateVariable(content);
    } else if !is_tag_expression && !is_template_variable {
        return_val = ContentType::Literal(input_line.to_string());
    } else {
        return_val = ContentType::Unrecognized;
    }
    return_val
}

pub fn check_symbol_string(input: &str, symbol: &str) -> bool {
    input.contains(symbol)
}

pub fn check_matching_pair(input: &str, symbol1: &str, symbol2: &str) -> bool {
    input.contains(symbol1) && input.contains(symbol2)
}

pub fn get_index_for_symbol(input: &str, symbol: char) -> (bool, usize) {
    let mut characters = input.char_indices();
    let mut does_exist = false;
    let mut index = 0;

    while let Some((c, d)) = characters.next() {
        if d == symbol {
            does_exist = true;
            index = c;
            break;
        }
    }
    (does_exist, index)
}

pub fn get_expression_data(input_line: &str) -> ExpressionData {
    let (_h, i) = get_index_for_symbol(input_line, '{');
    let head = input_line[0..i].to_string();
    let (_j, k) = get_index_for_symbol(input_line, '}');
    let variable = input_line[i + 1 + 1..k].to_string();
    let tail = input_line[k + 1 + 1..].to_string();

    ExpressionData {
        head: Some(head),
        variable: variable,
        tail: Some(tail),
    }
}

use std::collections::HashMap;

pub fn generate_html_template_var(
    content: ExpressionData,
    context: HashMap<String, String>,
) -> String {
    let mut html = String::new();
    // println!("expression data is:{:?}", content);
    if let Some(h) = content.head {
        html.push_str(&h);
    }

    if let Some(val) = context.get(&content.variable) {
        html.push_str(&val);
    }

    if let Some(t) = content.tail {
        html.push_str(&t);
    }

    html
}
