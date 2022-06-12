use colored::Colorize;
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref LOG: Mutex<Vec<Type>> = Mutex::new(vec![]);
}

pub enum Sender {
    Backend,
    Frontend,
    None,
}

impl Sender {
    const fn to_text(&self) -> &'static str {
        match self {
            Sender::Backend => "[BACKEND] ",
            Sender::Frontend => "[FRONTEND] ",
            Sender::None => "",
        }
    }
}

enum Type {
    Warning(String),
    Info(String),
}

pub fn info(sender: Sender, msg: impl ToString) {
    add_to_log(Type::Info(format!(
        "{}{}",
        sender.to_text(),
        msg.to_string()
    )));
}

pub fn warning(sender: Sender, msg: impl ToString) {
    add_to_log(Type::Warning(format!(
        "{}{}",
        sender.to_text(),
        msg.to_string()
    )));
}

pub fn fatal(sender: Sender, msg: impl ToString) {
    panic!("{}{}", sender.to_text(), msg.to_string());
}

#[inline]
fn add_to_log(msg: Type) {
    println!(
        "{}",
        match &msg {
            Type::Warning(msg) => msg.yellow(),
            Type::Info(msg) => msg.white(),
        },
    );
    LOG.lock().unwrap().push(msg);
}
