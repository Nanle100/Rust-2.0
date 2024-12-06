// Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company; 
//for example, “Add Sally to Engineering” or “Add Amir to Sales.” 
//Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.


use std::collections::HashMap;

fn main(){
    let mut company: HashMap<&str, Vec<&str>> = HashMap::new();


    company.insert("Engineering", vec!["Sally", "Grace", "Anabel"]);
    company.insert("medicine", vec!["Anna", "Felix", "Sarah", "Moses"]);
    company.insert("Marketing", vec!["Leah", "Zilphah", "Collins"]);



    for (key, value) in &company {
        println!("{key}: {:?}", value);
    }
}



