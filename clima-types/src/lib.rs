use std::collections::HashSet;

#[derive(Clone, Default, Debug)]
pub struct ClientInfo {
    name: String,
    email: String,
    postcode: String,
    city: String,
    phone: String,
    assortment: HashSet<String>,
}

impl ClientInfo {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn email(&self) -> &str {
        &self.email
    }
    pub fn postcode(&self) -> &str {
        &self.postcode
    }
    pub fn city(&self) -> &str {
        &self.city
    }
    pub fn phone(&self) -> &str {
        &self.phone
    }
    pub fn new(
        name: String,
        email: String,
        postcode: String,
        city: String,
        phone: String,
        assortment: HashSet<String>,
    ) -> Self {
        Self {
            name,
            email,
            postcode,
            city,
            phone,
            assortment,
        }
    }
}
