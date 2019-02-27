pub trait Itemizable {
    fn foreign_key(&self) -> usize;
}

#[derive(Debug, Default, PartialEq)]
pub struct Dataset {
    pub name: &'static str,
    pub items: Vec<Item>,
}

impl Dataset {
    pub fn new(name: &'static str) -> Self {
        Dataset {
            name,
            ..Default::default()
        }
    }

    pub fn push<T>(&mut self, item: T)
    where
        T: Into<Item>,
    {
        self.items.push(item.into());
    }
}

#[derive(Debug, Default, PartialEq)]
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

#[derive(Debug, Default, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub enum Item {
    Cat(CatItem),
    Dog(DogItem),
}

impl From<CatItem> for Item {
    fn from(cat: CatItem) -> Item {
        Item::Cat(cat)
    }
}

impl From<DogItem> for Item {
    fn from(cat: DogItem) -> Item {
        Item::Dog(cat)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_datasets_are_the_same() {
        let mut dataset1 = Dataset::new("Cats and dogs");
        dataset1.push(DogItem::new("woofie", 4));
        dataset1.push(CatItem::new("meowie", 10000));

        let mut dataset2 = Dataset::new("Cats and dogs");
        dataset2.push(DogItem::new("woofie", 4));
        dataset2.push(CatItem::new("meowie", 10000));

        assert_eq!(dataset1, dataset2);
    }
}
