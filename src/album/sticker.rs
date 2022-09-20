use serde::{Serialize, Deserialize};
#[derive(Clone, Copy)]
#[derive(Serialize, Deserialize)]
pub struct Sticker<'a> {
    pub id: &'a str,
    name: &'a str,
    collected: bool,
    pub quantity: u32,
}

impl std::fmt::Display for Sticker<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(
            f,
            "id: {}, name: {} \n\t\tcollected: {}, \n\t\trepeated: {}, \n\t\tquantity: {}",
            self.id,
            self.name,
            self.collected,
            self.have_repeated(),
            self.quantity
        )
    }
}

impl Sticker<'_> {
    pub fn new<'a>(id: &'a str, name: &'a str) -> Sticker<'a> {
        Sticker {
            id,
            name,
            collected: false,
            quantity: 0,
        }
    }

    pub fn collect(&mut self) {
        self.collected = true;
        self.quantity += 1;
    }

    pub fn same_id(&self, id: &String) -> bool {
        self.id.eq(id)
    }

    pub fn have_repeated(&self) -> bool {
        self.quantity > 1
    }

    pub fn is_missing(&self) -> bool{
        self.quantity == 0
    }

    pub fn trade(&mut self) -> bool {
        if self.have_repeated() {
            self.quantity -= 1;
            return true
        }
        println!("Não foi possível trocar esta figurinha pois você não tem repetida: {} {}", self.id, self.name);
        return false
    }

    pub fn clean(&mut self) {
        self.quantity = 1
    }
}

impl Eq for Sticker<'_> {}

impl PartialEq for Sticker<'_> {
    fn eq(&self, another: &Self) -> bool {
        self.same_id(&another.id.to_string())
    }
}
