use grammers_client::types::chat::Chat;

pub fn print_dialog_info(dialog: &Chat) {
    let dialog_id = dialog.id();
    let dialog_name = dialog.name();
    let dialog_username = dialog.username().unwrap_or("NO_USERNAME");

    println!("Dialog ID: {}", dialog_id);
    println!("Dialog Name: {}", dialog_name);
    println!("Dialog Username: {}", dialog_username);
    println!("-----------------------------------------");
}
