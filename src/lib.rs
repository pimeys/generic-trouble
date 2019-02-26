pub trait Itemizable {
    fn foreign_key(&self) -> usize;
}

#[derive(Default)]
pub struct Dataset {
    pub name: &'static str,
    pub items: Vec<Box<dyn Itemizable>>,
}

impl Dataset {
    pub fn new(name: &'static str) -> Self {
        Dataset {
            name,
            ..Default::default()
        }
    }
}

pub struct DogItem {
    pub name: &'static str,
    pub age: u8,
}

impl DogItem {
    pub fn new(name: &'static str, age: u8) -> Self {
        DogItem { name, age }
    }
}

impl Itemizable for DogItem {
    fn foreign_key(&self) -> usize {
        (self.age * 2) as usize
    }
}

pub struct CatItem {
    pub name: &'static str,
    pub meows: u16,
}

impl CatItem {
    pub fn new(name: &'static str, meows: u16) -> Self {
        CatItem { name, meows }
    }
}

impl Itemizable for CatItem {
    fn foreign_key(&self) -> usize {
        (self.meows / 2) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_datasets_are_the_same() {
        let mut dataset1 = Dataset::new("Cats and dogs");
        dataset1.items.push(Box::new(DogItem::new("woofie", 4)));
        dataset1.items.push(Box::new(CatItem::new("meowie", 10000)));

        let mut dataset2 = Dataset::new("Cats and dogs");
        dataset2.items.push(Box::new(DogItem::new("woofie", 4)));
        dataset2.items.push(Box::new(CatItem::new("meowie", 10000)));

        assert_eq!(dataset1, dataset2);
    }
}
