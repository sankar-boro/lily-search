const DOC_ONE: &str = "Hello World! My name is Sankar boro";
const DOC_TWO: &str = "Today is a very hot day, isn't it Sankar. The World is increasing in temperature day by day.";
const IGNORE: [&str; 4] = ["a", "is", "it", "in"];

// Function to perform sort
pub fn string_sort(doc: &mut Vec<&str>) {
    let mut i = 0;
    let mut j = 0;
    while i < doc.len() {
        j = i + 1;
        while j < doc.len() {
            if doc[j] < doc[i] {
                let temp = doc[i];
                doc[i] = doc[j];
                doc[j] = temp;
            }
            j += 1;
        }
        i += 1;
    }
}

pub fn split_sentence(doc: &str) -> Vec<&str> {
    doc.split(' ').collect()
}

