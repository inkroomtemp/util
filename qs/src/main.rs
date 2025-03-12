use std::{
    fmt::{Display, Write},
    io::{BufRead, BufReader, Read},
    ops::Deref,
    thread::sleep,
    time::Duration,
};
use selenium::{
    driver::{self, Driver},
    option::{FirefoxBuilder, Proxy},
    By,
};


fn new_driver() -> Result<Driver, selenium::SError> {
    let mut d = FirefoxBuilder::new()
            .url("http://127.0.0.1:38472")
            .head_less()
            .private()
            // .drivers("/app/geckodriver")
            .add_pref_i32("permissions.default.stylesheet", 2)
            // .add_pref_i32("permissions.default.image", 2)
            ;
    if let Ok(proxy) = std::env::var("PROXY") {
        d = d.proxy(Proxy::manual().ssl_proxy(&proxy).http_proxy(&proxy));
    }
    if let Ok(proxy) = std::env::var("URL") {
        d = d.url(&proxy);
        if let Ok(binary) = std::env::var("BINARY") {
            d = d.binary(&binary);
        }
    } else {
        // d = d.head_less();
    }
    Ok(Driver::new(d.build())?)
}
fn run2(driver: &Driver) -> Result<(), selenium::SError> {
    // 登陆

    driver.get("https://www.qsbtxt.net/member.php?mod=logging&action=login")?;
    driver.find_elements(By::TagName("input"))?[8].send_keys("2rZjOIUj4u")?;
    driver.find_elements(By::TagName("input"))?[9].send_keys("ZcCjyn9Hk0")?;
    driver.find_elements(By::TagName("button"))?[1].click()?;
    sleep(Duration::from_secs(5));

        driver.get("https://www.qsbtxt.net/f51-1.html")?;

        // let a = driver
        //     .find_element(By::Id("portal_block_101_content"))?
        //     .find_elements(By::TagName("a"))?;
        // let hrefs = driver.find_elements(By::TagName("tbody"))?[7..].iter().map(|f|f.find_elements(By::TagName("a")).unwrap()[1].get_property("href").unwrap().unwrap()).collect::<Vec<String>>();
        let a = &driver.find_elements(By::Class("xst"))?[4..];
        let hrefs = a
            .iter()
            .map(|f| f.get_property("href").unwrap().unwrap())
            .collect::<Vec<String>>();
        for ele in hrefs {
            let href = ele;
            //    let href = ele.unwrap().unwrap();
            println!("href {}", href);
            driver.get(&href)?;
            driver
                .find_element(By::Id("fastpostmessage"))?
                .send_keys("感谢分享")?;
            sleep(Duration::from_secs(5));

            // driver.find_elements(By::TagName("strong"))?[6].click()?;

            // driver.actions().click(Some(&driver.find_element(By::Id("fastpostsubmit"))?)).perform()?;
            driver.find_element(By::Id("fastpostsubmit"))?.click()?;
        }

    Ok(())
}

fn main() {

    match new_driver() {
        Ok(driver) => {
            let v: Vec<String> = std::env::args().collect();
            match run2(&driver) {
                Ok(()) => {
                    let _ = driver.quit();
                    drop(driver);
                    // gen::gen_epub(&book).unwrap();
                }
                Err(e) => {
                    match driver.take_screenshot() {
                        Ok(f) => {
                            std::fs::write("c.png", f).unwrap();
                        }
                        Err(_) => {}
                    }
                    println!("{}", driver.get_current_url().unwrap());
                    //    let _ =  driver.take_screenshot().and_then(|f|std::fs::write("c.png", f));

                    let _ = driver.quit();

                    drop(driver);

                    Err::<(), selenium::SError>(e).unwrap();
                }
            }
        }
        Err(e) => {
            eprintln!("new driver");
            Err::<(), selenium::SError>(e).unwrap();
            // eprintln!("{e}");
        }
    }
}