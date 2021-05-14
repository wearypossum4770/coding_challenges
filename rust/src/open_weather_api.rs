use error_chain::error_chain;
use std::io::Read;
use reqwest;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}
pub async fn open_weather_api() -> Result<()> {
    // let latitude = 35.0897936; 
    // let longitude= -85.1820351;
    // let api_key = "0d379932ab8f74b262973b9886276755";
    
    // let api = "api.openweathermap.org/data/2.5/weather?lat=35.0897936&lon=-85.1820351&appid=0d379932ab8f74b262973b9886276755";
    // let res = reqwest::get().await?;
    let mut res = reqwest::blocking::get("api.openweathermap.org/data/2.5/weather?lat=35.0897936&lon=-85.1820351&appid=0d379932ab8f74b262973b9886276755")?;
    let mut body = String::new();
    res.read_to_string(&mut body)?;

    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());
    println!("Body:\n{}", body);

    Ok(())
}
