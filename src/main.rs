use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Resp {
  quoteText: String,
  quoteAuthor: String,
}

#[derive(Debug)]
enum CustomError {
 JsonError(serde_json::error::Error),
 HttpError(reqwest::Error)
}

impl From<serde_json::error::Error> for CustomError {
  fn from(error: serde_json::error::Error) -> Self {
    CustomError::JsonError(error)
  }
}

impl From<reqwest::Error> for CustomError {
  fn from(error: reqwest::Error) -> Self {
    CustomError::HttpError(error)
  }
}


fn call_api() -> Result<Resp, CustomError> {
    let body = reqwest::get("http://api.forismatic.com/api/1.0/?method=getQuote&format=json&lang=en")?.text()?;
    let json:Resp = serde_json::from_str(&body)?;
    Ok(json)
}

fn main() {
    match call_api(){
      Err(custom_error) => {
        println!("{:?}", custom_error);
      }
      Ok(resp) => {
        println!("\n#\n# {}\n#\n", resp.quoteText);
      }
    }
}
