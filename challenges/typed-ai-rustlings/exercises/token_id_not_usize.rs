use category_theory_transformer_rs::TokenId;

fn main() {
    let raw_index = 3;

    println!("visible token index: {}", lookup_token(raw_index));
}

fn lookup_token(token: TokenId) -> usize {
    token.index()
}
