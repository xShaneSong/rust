use template_engine::{ContentType, ExpressionData, TagType,
     get_content_type, check_symbol_string, check_matching_pair, get_expression_data, get_index_for_symbol};

#[cfg(test)]
mod tests {
}

#[test]
fn check_literal_test() {
    let s = "<h1>Hello World</h1>";
    assert_eq!(ContentType::Literal(s.to_string()), get_content_type(s));
}

#[test]
fn check_template_variable_test() {
    let content = ExpressionData {
        head: Some("Hi ".to_string()),
        variable: " name ".to_string(),
        tail: Some(" , welcome".to_string()),
    };
    assert_eq!(
        ContentType::TemplateVariable(content),
        get_content_type("Hi {{ name }} , welcome")
    );
}

#[test]
fn check_get_index_forx_symbol_test() {
    assert_eq!((true, 3), get_index_for_symbol("Hi {{ name }}, welcome", '{'));
}

#[test]
fn check_for_tag_test() {
    assert_eq!(ContentType::Tag(TagType::ForTag), get_content_type("{% for name in name %}, welcome"));
}

#[test]
fn check_if_tag_test() {
    assert_eq!(ContentType::Tag(TagType::IfTag), get_content_type("{% if name == Bob %}"));
}

#[test]
fn check_symbol_string_test() {
    assert_eq!(true, check_symbol_string("{{Hello}}", "{{"));
}

#[test]
fn check_symbol_pair_test() {
    assert_eq!(true, check_matching_pair("{{Hello}}", "{{", "}}"));
}

#[test]
fn check_get_expression_data_test() {
    let content = ExpressionData {
        head: Some("Hi ".to_string()),
        variable: " name ".to_string(),
        tail: Some(" , welcome".to_string()),
    };
    assert_eq!(content, get_expression_data("Hi {{ name }} , welcome"));
}

