use cursive::{align::{HAlign}, view::{Nameable, Resizable}, 
    views::{ Button, Dialog, EditView, LinearLayout, TextView}, Cursive};
use postgres::{Client, NoTls};
use dotenv::dotenv;
use std::env;
fn main(){
    dotenv().ok();
    let mut siv = cursive::default();

    siv.add_layer(
        Dialog::around(TextView::new("CLEXIT"))
        .button("login",|s| login(s))
        .button("quit", |s| s.quit())
        .full_screen()
    );    



    siv.run();
}


fn login(siv: &mut Cursive){
    siv.pop_layer();
    
    siv.add_layer(
        Dialog::around(LinearLayout::vertical()
        .child(TextView::new("Login"))
        .child(TextView::new("Username: "))
        .child(EditView::new().with_name("username"))
        .child(TextView::new("Password: "))
        .child(EditView::new().with_name("password"))
        .child(Button::new("Login", |s| {
            let username = s.call_on_name("username", |v: &mut EditView| v.get_content()).unwrap();
            let password = s.call_on_name("password", |v: &mut EditView| v.get_content()).unwrap();
            if check_user(&username, &password) {
                menu(s);
            }else{
                s.add_layer(TextView::new("User not found!"));
            }
        }))
        .child(Button::new("Quit", |s| s.quit()))
        .full_screen()
    ));
}

fn check_user(username: &String, password: &String) -> bool {
    let database_url = env::var("DB_URL").unwrap().to_string();
   match Client::connect(&database_url, NoTls){
        Ok(mut client) => {
            let rows = client.query(
                "SELECT * FROM users WHERE username = $1 AND password = $2",
                &[&username, &password]).unwrap();
            client.close().unwrap();
            rows.len() > 0
        }
        Err(e) => {
            println!("Error: {}", e);
            false
        }
    }
}

fn menu(siv:&mut Cursive){
    siv.pop_layer();
    siv.add_layer(
        Dialog::around(LinearLayout::vertical()
        .child(TextView::new("Menu").h_align(HAlign::Center)))
        .full_screen()
    );
}

