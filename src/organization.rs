use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize)]
struct Head {
    index: String, 
    organization_id: String, 
    name: String, 
    website: String, 
    country: String, 
    description: String, 
    founded: String, 
    industry: String, 
    number_of_employees: String,
}

pub fn fill_json(contentarray: Vec<Vec<String>>) -> String {
    let head: Head = Head {
        index: String::from("Test one passed"),
        organization_id: String::from("Test two passed"),
        name: String::from("Test three passed"),
        website: String::from("Test four passed"),
        country: String::from("Test five passed"),
        description: String::from("Test six passed"),
        founded: String::from("Test seven passed"),
        industry: String::from("Test eight passed"),
        number_of_employees: String::from("Test nine passed")
    };

    let json = serde_json::to_string(&head).unwrap();
    json
}