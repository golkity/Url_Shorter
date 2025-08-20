pub trait GetFullUrlRepository {
    fn get(&self, id: &str) -> Result<String,String>;
}

pub struct GteFullUrlQuery <R>
where
R: GetFullUrlRepository,
{
    repo: R,
}

impl <R>GteFullUrlQuery<R> 
where
    R: GetFullUrlRepository,
{
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
    
    pub fn execute(&self, id: &str) -> Result<String,String> {
        self.repo.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_get_full_url() {
        
    }
}