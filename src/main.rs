mod ast;
mod cli;
mod lsp;
mod syn;

#[tokio::main]
async fn main() {
    cli::run().await;
}
