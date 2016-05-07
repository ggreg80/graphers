extern crate graphers;
extern crate serde;
extern crate serde_json;

#[path = "../fixtures/schema.rs"]
mod schema;

use schema::*;
use std::borrow::Cow;

#[derive(Debug)]
struct QueryRoot;

#[derive(Debug)]
struct Person {
  id: String,
  first_name: String,
  last_name: String,
  age: i32,
}

impl ResolvePerson for Person {
    type Person = Person;

    fn id(&self) -> Cow<str> {
        self.id.as_str().into()
    }

    fn first_name(&self) -> Cow<str> {
        self.first_name.as_str().into()
    }

    fn last_name(&self) -> Cow<str> {
        self.last_name.as_str().into()
    }

    fn friend(&self) -> Option<Person> {
        None
    }

    fn age(&self) -> i32 {
        self.age
    }

    fn tags(&self) -> Cow<[Cow<str>]> {
        vec!["foo".into(), "bar".into()].into()
    }
}

impl ResolveQueryRoot for QueryRoot {
    type Person = Person;

    fn person(&self, id: Cow<str>) -> Person {
        Person {
            id: id.to_string(),
            first_name: String::from("Jonas"),
            last_name: String::from("Nicklas"),
            age: 30,
        }
    }
}

fn main() {
    let doc = "
        query {
            person(id: \"12345\") {
                first_name,
                last_name,
                tags,
            }
        }
    ";
    let context = graphers::parse(doc);
    let query = context.query().expect("should define a query");

    let result = serde_json::to_string_pretty(&schema::query(QueryRoot, query)).expect("failed to serialize");

    println!("{}", result);
}
