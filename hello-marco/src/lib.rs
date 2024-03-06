/*A Marco Polo Game

If the name Marco is given, the response is Polo. 
If the name Marco is not given, the response is What?.

*/

pub fn marco_polo(name: &str) -> String {
    if name == "Marco" {
        "Polo".to_string()
    } else {
        "What is you name?".to_string()
    }
}