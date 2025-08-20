pub struct CreateShortUrl;

impl CreateShortUrl{
    pub async fn execute(&self, full_url: String) -> Result<String, String> {
       Ok("".to_owned())
    }
}


#[cfg(test)]
mod tests {}