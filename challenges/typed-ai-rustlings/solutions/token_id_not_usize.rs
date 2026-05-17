use category_theory_transformer_rs::TokenId;

pub fn visible_token_index() -> usize {
    let raw_index = 3;
    let token = TokenId::new(raw_index);
    lookup_token(token)
}

fn lookup_token(token: TokenId) -> usize {
    token.index()
}
