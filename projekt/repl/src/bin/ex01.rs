use grovedb::{operations::insert::InsertOptions, Element, GroveDb, PathQuery, Query, Transaction};
use grovedb_version::version::GroveVersion;

const TREE1: &[u8] = b"tree1";
const root_path: &[&[u8]] = &[];

// Allow insertions to overwrite trees
// This is necessary so the tutorial can be rerun easily
const INSERT_OPTIONS: Option<InsertOptions> = Some(InsertOptions {
    validate_insertion_does_not_override: false,
    validate_insertion_does_not_override_tree: false,
    base_root_storage_is_free: true,
});

fn main() {
    // Specify a path and open GroveDB at the path as db
    let path = String::from("../tutorial-storage");
    let db = GroveDb::open(path).unwrap();

    let grove_version = GroveVersion::latest();


    // Insert key-value 1 into the root tree
    db.insert(
        root_path,
        b"key1",
        Element::new_item(b"vrijednost-1.1".to_vec()),
        None,
        None,
        grove_version,
    )
    .unwrap()
    .expect("successful key1 insert");

    insert_empty_tree_db(&db, root_path, TREE1, &grove_version);

    // Insert key-value 2 into the tree1
    db.insert(
        &[TREE1],
        b"key2",
        Element::new_item(b"vrijednost2_1".to_vec()),
        None,
        None,
        grove_version,
    )
    .unwrap()
    .expect("successful key2 insert");

  // Insert key-value 3 into the tree1
    db.insert(
        &[TREE1],
        b"key3",
        Element::new_item(b"vrijednost2_2".to_vec()),
        None,
        None,
        grove_version,
    )
    .unwrap()
    .expect("successful key2 insert");

    // At this point the Items are fully inserted into the database.
    // No other steps are required.

    // To show that the Items are there, we will use the get()
    // function to get them from the RocksDB backing store.

    // Get value 1
    let result1 = db.get(root_path, b"key1", None, grove_version).unwrap();

    // Get value 2
    let result2 = db.get(&[TREE1], b"key2", None, grove_version).unwrap();

    // Print the values to terminal
    println!("{:?}", result1);
    println!("{:?}", result2);
}

fn insert_empty_tree_db(db: &GroveDb, path: &[&[u8]], key: &[u8], grove_version: &GroveVersion)
{
    db.insert(path, key, Element::empty_tree(), INSERT_OPTIONS, None, grove_version)
        .unwrap()
        .expect("successfully inserted tree");
}
