use headless_chrome::Browser;
use lazy_static::lazy_static;

lazy_static!{
    pub static ref CHROME:Browser = Browser::default().unwrap();
}

pub fn new_chrome(){
    let browser = Browser::default().unwrap();
    let tab = browser.new_tab().unwrap();
    tab.navigate_to("https://www.lesterhnu.top").unwrap();
    
    tab.wait_for_element("body").unwrap();
    let res = tab.get_content().unwrap();
    println!("{}",res);
}



#[test]
fn test_new_chrome(){
    let _res = new_chrome();
    // assert!()
}