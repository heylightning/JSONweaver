
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
    let mut proto_jsonlist: Vec<String> = Vec::new();
    
    for content in contentarray {
        let head: Head = Head {
            index: format!("{}", content[0]),
            organization_id: format!("{}", content[1]),
            name: format!("{}", content[2]),
            website: format!("{}", content[3]),
            country: format!("{}", content[4]),
            description: format!("{}", content[5]),
            founded: format!("{}", content[6]),
            industry: format!("{}", content[7]),
            number_of_employees: format!("{}", content[8]), 
        };

        let json = serde_json::to_string(&head).unwrap();
        proto_jsonlist.push(json.clone());
    }

    let xconst = proto_jsonlist.join(", ");
    let weave = "[ ".to_owned() + &xconst + " ]";

    weave  
}