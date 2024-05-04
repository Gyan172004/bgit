use dialoguer::{theme::ColorfulTheme, Confirm};
pub(crate) fn clone(clone_url: &str) {
    let confirmation = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you want to continue?")
        .default(false)
        .show_default(true)
        .wait_for_newline(true)
        .interact()
        .unwrap();

    if confirmation {
        println!("I will clone {clone_url}");
    } else {
        println!("nevermind then :(");
    }
}