use radix_trie::Trie;

#[test]
fn test_exists() {
    let mut t = Trie::new();
    t.insert(".", "1");
    t.insert(".com.", "2");
    t.insert(".com.a.", "3");
    t.insert(".com.a.h.", "4");
    t.insert(".com.b.", "5");

    let val = t.get_ancestor_value(".com.");
    assert_eq!(val.is_some(), true);
    assert_eq!(val.unwrap(), &"2");

    let val = t.get_ancestor_value(".com.ah.");
    assert_eq!(val.is_some(), true);
    assert_eq!(val.unwrap(), &"2");

    let val = t.get_ancestor_value(".com.a.");
    assert_eq!(val.is_some(), true);
    assert_eq!(val.unwrap(), &"3");

    let val = t.get_ancestor_value(".com.a.h.");
    assert_eq!(val.is_some(), true);
    assert_eq!(val.unwrap(), &"4");
}

#[test]
fn test_does_not_exist() {
    let mut t = Trie::new();
    t.insert(".com.", "1");

    let val = t.get_ancestor_value(".net");
    assert_eq!(val.is_none(), true);

    let val = t.get_ancestor_value(".net.example");
    assert_eq!(val.is_none(), true);
}

fn main() {
    let mut trie = Trie::new();
    trie.insert("hello", 19u32);
    trie.insert("hellcat", 35u32);
    trie.insert("not related", 1u32);
    trie.insert("handle nested", 5u32);

    println!("{:#?}", trie);
}
