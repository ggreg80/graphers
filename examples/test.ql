schema {
    query: QueryRoot
}

type QueryRoot {
    person: Person
}

type Person {
    first_name: String
    last_name: String
    friend: Person
    age: Int
}
