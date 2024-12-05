// Convert strings to pig latin. The first consonant of each word is moved to the end of the word and ay is added, so first becomes irst-fay. 
// Words that start with a vowel have hay added to the end instead
// (apple becomes apple-hay). Keep in mind the details about UTF-8 encoding!


fn main() {
    let s1 = String::from("zebra");

    let vowels = ['a', 'e', 'i', 'o', 'u'];

    // Check if the first character is a vowel
    if vowels.contains(&s1.chars().next().unwrap()) {
        let new_word = format!("{}-hay", s1); // Corrected here
        println!("{}", new_word);
    } else {
        // Find the first consonant and rearrange
        let mut chars = s1.chars();
        let first_consonant = chars.next().unwrap(); // Get the first character
        let rest: String = chars.collect(); // Collect the remaining characters
        let new_word = format!("{}-{}ay", rest, first_consonant); // Corrected here
        println!("{}", new_word);
    }
}
