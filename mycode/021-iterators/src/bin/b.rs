use std::collections::HashMap;

fn main() {
    let my_vector_int = vec![1, 2, 3, 4, 5];
    let my_iterator_int = &my_vector_int.into_iter();

    let my_vector_bool = vec![true, false, true];
    let my_iterator_bool = &my_vector_bool.into_iter();

    let mut my_hashmap = HashMap::new();
    my_hashmap.insert("CBS", 2);
    let my_iterator_hashmap = &my_hashmap.into_iter();
}
