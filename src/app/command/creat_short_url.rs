use crate::id_provider::IDProvider;


pub trait CreateShortUrlRepository {
    fn save(&self, full_url: String, id: String) -> Result<(),String>;
}

pub struct CreateShortUrl <I, R>
where
    I: IDProvider,
    R: CreateShortUrlRepository
{
    id_provider: I,
    repo: R
}

impl <I,R> CreateShortUrl <I, R>
where
    I:IDProvider,
    R:CreateShortUrlRepository
{

    pub fn new(id_provider: I, repo: R) -> Self {
        Self { id_provider, repo }
    }
    pub async fn execute(&self, full_url: String) -> Result<String, String> {
        let id = self.id_provider.provide();

        self.repo.save(full_url, id.clone())?;

       Ok(id)
    }
}


#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use dashmap::DashMap;
    use crate::app::adapters::inmemory::InMemoryRepository;
    use super::*;

   #[tokio::test]
   async fn get_short_url() {
       let id_provider = crate::id_provider::FakeIDProvider::new("123".to_owned());
       let store = Arc::new(DashMap::new());
       let repo = InMemoryRepository::new(store);
        let command = CreateShortUrl::new(id_provider,repo);
        
        let result = command.execute("https://www.google.com".to_owned()).await;
        
        assert_ne!(result, Ok("".to_owned()));
    }

    #[tokio::test]
    async fn get_rwo_different_short_url() {
        let idp = crate::id_provider::NanoIDProvider;
        let store = Arc::new(DashMap::new());
        let repo = InMemoryRepository::new(store);
        let command = CreateShortUrl::new(idp, repo);

        let result = command.execute("https://www.google.com".to_owned()).await;

        let result2 =  command.execute("https://www.google.com".to_owned()).await;

        assert_ne!(result, result2);
    }

}