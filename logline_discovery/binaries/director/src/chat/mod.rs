//! Conversational chat interface

use std::io::{self, Write};
use crate::agents::Director;

// a) sua função real:
pub async fn start() {
    println!("🤖 Director: Ready. What do you need?");
    loop {
        print!("👤 You: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.is_empty() {
            continue;
        }
        if input == "exit" || input == "quit" {
            println!("👋 Goodbye!");
            break;
        }

        let answer = Director::handle(input, "logline-id://user", "admin").await;
        println!("🤖 Director: {}", answer);
    }
}

// b) casca p/ compatibilidade com o bin:
pub mod shell {
    pub async fn start() {
        super::start().await;
    }
}
