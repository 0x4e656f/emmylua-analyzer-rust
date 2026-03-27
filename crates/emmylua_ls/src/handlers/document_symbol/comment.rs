use emmylua_parser::{LuaAstNode, LuaComment, LuaSyntaxId, LuaTokenKind};
use lsp_types::SymbolKind;
use rowan::NodeOrToken;

use super::builder::{DocumentSymbolBuilder, LuaSymbol};

pub fn build_doc_region_symbol(
    builder: &mut DocumentSymbolBuilder,
    comment: LuaComment,
    parent_id: LuaSyntaxId,
) -> Option<LuaSyntaxId> {
    let mut region_token = None;
    for child in comment.syntax().children_with_tokens() {
        if let NodeOrToken::Token(token) = child {
            if token.kind() == LuaTokenKind::TkDocRegion.into() {
                region_token = Some(token);
                break;
            }
        }
    }

    let region_token = region_token?;

    let description = comment
        .get_description()
        .map(|desc| desc.get_description_text())
        .map(|text| {
            text.lines()
                .next()
                .map(|line| line.trim())
                .unwrap_or_default()
                .to_string()
        })
        .filter(|text| !text.is_empty())
        .unwrap_or_else(|| "region".to_string());

    let range = comment.get_range();
    let selection_range = region_token.text_range();
    let symbol = LuaSymbol::with_selection_range(
        description,
        None,
        SymbolKind::NAMESPACE,
        range,
        selection_range,
    );

    let symbol_id = builder.add_node_symbol(comment.syntax().clone(), symbol, Some(parent_id));

    Some(symbol_id)
}

pub fn build_mark_symbol(
    builder: &mut DocumentSymbolBuilder,
    comment: LuaComment,
    parent_id: LuaSyntaxId,
) -> Option<LuaSyntaxId> {
    let mark_name = extract_mark_name(&comment)?;
    let range = comment.get_range();
    let symbol = LuaSymbol::new(mark_name, None, SymbolKind::FUNCTION, range);
    let mark_token = comment
        .syntax()
        .children_with_tokens()
        .filter_map(|child| child.into_token())
        .find(|token| token.text().trim_start().starts_with("---MARK"));

    let symbol_id = if let Some(token) = mark_token {
        builder.add_token_symbol(token, symbol, Some(parent_id))
    } else {
        builder.add_node_symbol(comment.syntax().clone(), symbol, Some(parent_id))
    };
    Some(symbol_id)
}

fn extract_mark_name(comment: &LuaComment) -> Option<String> {
    let comment_text = comment.syntax().text().to_string();
    let first_line = comment_text.lines().next()?.trim_start();

    let mark_body = first_line.strip_prefix("---MARK")?.trim();
    if mark_body.is_empty() {
        return Some("MARK".to_string());
    }

    Some(mark_body.to_string())
}
