pub fn store_data() {
    let mut a = split_sentence("hello world my name");
    string_sort(&mut a);

    let mut b = split_sentence("help dear world");
    string_sort(&mut b);

    let mut coll: HashMap<String, Vec<String>> = HashMap::new();

    for kkey in a {
        let key = kkey.to_string();
        if !coll.contains_key(kkey) {
            let mut n = Vec::new();
            n.push("DOC_ONE".to_string());
            coll.insert(key, n);
        } else {
            (*coll.get_mut(kkey).unwrap()).push("DOC_ONE".to_string());
        }
    }

    for kkey in b {
        let key = kkey.to_string();
        if !coll.contains_key(kkey) {
            let mut n = Vec::new();
            n.push("DOC_TWO".to_string());
            coll.insert(key, n);
        } else {
            (*coll.get_mut(kkey).unwrap()).push("DOC_TWO".to_string());
        }
    }

    println!("{:?}", coll);
}