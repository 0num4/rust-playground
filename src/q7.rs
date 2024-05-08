pub fn main() {
    println!("q7!!!!!!!!!!!!!!!!!!");
    let p = Person {
        name: "0num4".to_string(),
        age: 20,
    };
    let rp: PersonRef = PersonRef::new(&p);
    println!("{}", rp.greet());
}

struct Person {
    name: String,
    age: u32,
}

struct PersonRef<'a> {
    name: &'a str,
    age: &'a u32,
}
impl<'a> PartialEq for PersonRef<'a> {
    fn eq(&self, other: &Self) -> bool {
        return self.age == other.age;
    }
}

impl<'a> PartialOrd for PersonRef<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return self.age.partial_cmp(other.age);
    }
}

trait TraitPersonRef<'a> {
    fn new(person: &'a Person) -> PersonRef<'a>;
    fn greet(&self) -> String;
}
impl<'a> TraitPersonRef<'a> for PersonRef<'a> {
    fn new(person: &'a Person) -> PersonRef<'a> {
        return PersonRef {
            name: &person.name,
            age: &person.age,
        };
    }
    fn greet(&self) -> String {
        return format!(
            "Hello, my name is {} and I am {} years old.",
            self.name, self.age
        );
    }
}
