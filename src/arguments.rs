use std::collections::HashMap;

// Todo: find a better way to convert array to Hashmap
pub fn array_to_hashmap<'a>(args: &'a [(&str, &str)]) -> HashMap<&'a str, &'a str> {
    let mut hmp = HashMap::new();
    hmp.extend(args.iter().map(|a| a.clone()));
    hmp
}
