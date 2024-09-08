use std::collections::VecDeque;
use std::path::Path;
use grovedb::{operations::insert::InsertOptions, Element, GroveDb, PathQuery, Query, Transaction};
use grovedb::reference_path::ReferencePathType;
use rand::{distributions::Alphanumeric, Rng, };
use grovedb::element::SumValue;
use grovedb::replication::CURRENT_STATE_SYNC_VERSION;
use grovedb::replication::MultiStateSyncInfo;
use grovedb_version::version::GroveVersion;

const ROOT_PATH: &[&[u8]] = &[];

// Allow insertions to overwrite trees
// This is necessary so the tutorial can be rerun easily
const INSERT_OPTIONS: Option<InsertOptions> = Some(InsertOptions {
    validate_insertion_does_not_override: false,
    validate_insertion_does_not_override_tree: false,
    base_root_storage_is_free: true,
});

fn populate_db(grovedb_path: String, grove_version: &GroveVersion) -> GroveDb {
    let db = GroveDb::open(grovedb_path).unwrap();

    insert_empty_tree_db(&db, ROOT_PATH, MAIN_ΚΕΥ, &grove_version);
    insert_empty_tree_db(&db, ROOT_PATH, MAIN_ΚΕΥ_EMPTY, &grove_version);
    insert_empty_tree_db(&db, &[MAIN_ΚΕΥ], KEY_INT_0, &grove_version);
    insert_empty_tree_db(&db, &[MAIN_ΚΕΥ], KEY_INT_1, &grove_version);
    insert_empty_tree_db(&db, &[MAIN_ΚΕΥ], KEY_INT_2, &grove_version);

    let tx = db.start_transaction();
    let batch_size = 50;
    for i in 0..=5 {
        insert_range_values_db(&db, &[MAIN_ΚΕΥ, KEY_INT_0], i * batch_size, i * batch_size + batch_size - 1, &tx, &grove_version);
    }
    let _ = db.commit_transaction(tx);

    let tx = db.start_transaction();
    let batch_size = 50;
    for i in 0..=5 {
        insert_range_values_db(&db, &[MAIN_ΚΕΥ, KEY_INT_1], i * batch_size, i * batch_size + batch_size - 1, &tx, &grove_version);
    }
    let _ = db.commit_transaction(tx);

    let tx = db.start_transaction();
    let batch_size = 50;
    for i in 0..=5 {
        insert_range_values_db(&db, &[MAIN_ΚΕΥ, KEY_INT_2], i * batch_size, i * batch_size + batch_size - 1, &tx, &grove_version);
    }
    let _ = db.commit_transaction(tx);

    insert_empty_tree_db(&db, &[MAIN_ΚΕΥ], KEY_INT_REF_0, &grove_version);

    let tx_2 = db.start_transaction();
    insert_range_ref_double_values_db(&db, &[MAIN_ΚΕΥ, KEY_INT_REF_0], KEY_INT_0, 1, 50, &tx_2, &grove_version);
    let _ = db.commit_transaction(tx_2);

    insert_empty_sum_tree_db(&db, &[MAIN_ΚΕΥ], KEY_INT_A, &grove_version);

    let tx_3 = db.start_transaction();
    insert_range_values_db(&db, &[MAIN_ΚΕΥ, KEY_INT_A], 1, 500, &tx_3, &grove_version);
    insert_sum_element_db(&db, &[MAIN_ΚΕΥ, KEY_INT_A], 501, 550, &tx_3, &grove_version);
    let _ = db.commit_transaction(tx_3);
    db
}

fn create_empty_db(grovedb_path: String) -> GroveDb   {
    let db = GroveDb::open(grovedb_path).unwrap();
    db
}

fn main() {
    let grove_version = GroveVersion::latest();

    // 1. Creating base 1
    let path_base1 = generate_random_path("../tutorial-storage/", "/db_base1", 24);
    let db_base1 = create_empty_db(path_base1.clone());

    // 2. Creating base 2
    let path_base2 = generate_random_path("../tutorial-storage/", "/db_base2", 24);
    let db_base2 = create_empty_db(path_base2.clone());

    // 3. Populate base 1 with STUDENT, SUBJECT, and GRADE tables
    // Create subtrees for each table
    insert_empty_tree_db(&db_base1, ROOT_PATH, b"student", &grove_version);
    insert_empty_tree_db(&db_base1, ROOT_PATH, b"subject", &grove_version);
    insert_empty_tree_db(&db_base1, ROOT_PATH, b"grade", &grove_version);

    // Insert data into STUDENT table
    let students = vec![
        (b"1", "Ivo Ivić".as_bytes()),
        (b"2", "Pero Perić".as_bytes()),
    ];
    for (student_id, student_name) in students {
        db_base1.insert(
            &[b"student"],
            student_id,
            Element::new_item(student_name.to_vec()),
            INSERT_OPTIONS,
            None,
            &grove_version,
        ).unwrap().expect("successfully inserted student");
    }

    // Insert data into SUBJECT table
    let subjects = vec![
        (b"1", "Matematika".as_bytes()),
        (b"2", "Algoritmi".as_bytes()),
    ];
    for (subject_id, subject_name) in subjects {
        db_base1.insert(
            &[b"subject"],
            subject_id,
            Element::new_item(subject_name.to_vec()),
            INSERT_OPTIONS,
            None,
            &grove_version,
        ).unwrap().expect("successfully inserted subject");
    }

    // Insert data into GRADE table
    let grades = vec![
        (b"1_1", 4i32), // Ivo - matematika
        (b"1_2", 5i32), // Ivo - algoritmi 
        (b"2_1", 3i32), // Pero - matematika
        (b"2_2", 4i32), // Pero - algoritmi 
    ];
    for (grade_key, grade_value) in grades {
        db_base1.insert(
            &[b"grade"],
            grade_key,
            Element::new_item(grade_value.to_be_bytes().to_vec()),
            INSERT_OPTIONS,
            None,
            &grove_version,
        ).unwrap().expect("successfully inserted grade");
    }

    // 4. Print the root hash data of both databases
    println!("\n######### root_hashes before replication:");
    let root_hash_base1 = db_base1.root_hash(None, grove_version).unwrap().unwrap();
    println!("root_hash_base1: {:?}", hex::encode(root_hash_base1));
    let root_hash_base2 = db_base2.root_hash(None, grove_version).unwrap().unwrap();
    println!("root_hash_base2: {:?}", hex::encode(root_hash_base2));

    // 5. Replication of base 1 (source) to base 2 (destination)
    let state_info = MultiStateSyncInfo::default();
    let tx = db_base2.start_transaction();
    sync_db_demo(&db_base1, &db_base2, state_info, &tx, &grove_version).unwrap();
    db_base2.commit_transaction(tx).unwrap().expect("expected to commit transaction");

    // 6. Print the root hash data of both databases
    println!("\n######### root_hashes after replication:");
    let root_hash_base1 = db_base1.root_hash(None, grove_version).unwrap().unwrap();
    println!("root_hash_base1: {:?}", hex::encode(root_hash_base1));
    let root_hash_base2 = db_base2.root_hash(None, grove_version).unwrap().unwrap();
    println!("root_hash_base2: {:?}", hex::encode(root_hash_base2));

    // 7. Printing the corresponding value for the same key from both databases
    let query_path_student: &[&[u8]] = &[b"student"];
    let query_key_student = b"1".to_vec(); // Query for John Doe
    let query_path_subject: &[&[u8]] = &[b"subject"];
    let query_key_subject = b"2".to_vec(); // Query for Math
    let query_path_grade: &[&[u8]] = &[b"grade"];
    let query_key_grade = b"1_2".to_vec(); // Query for John Doe's grade in Math
    println!("\n######## Query on source:");
    query_db(&db_base1, query_path_student, query_key_student.clone(), &grove_version);
    query_db(&db_base1, query_path_subject, query_key_subject.clone(), &grove_version);
    query_db(&db_base1, query_path_grade, query_key_grade.clone(), &grove_version);

    println!("\n######## Query on replika:");
    query_db(&db_base2, query_path_student, query_key_student.clone(), &grove_version);
    query_db(&db_base2, query_path_subject, query_key_subject.clone(), &grove_version);
    query_db(&db_base2, query_path_grade, query_key_grade.clone(), &grove_version);
}

fn insert_empty_tree_db(db: &GroveDb, path: &[&[u8]], key: &[u8], grove_version: &GroveVersion)
{
    db.insert(path, key, Element::empty_tree(), INSERT_OPTIONS, None, grove_version)
        .unwrap()
        .expect("successfully inserted tree");
}
fn insert_range_values_db(db: &GroveDb, path: &[&[u8]], min_i: u32, max_i: u32, transaction: &Transaction, grove_version: &GroveVersion)
{
    for i in min_i..=max_i {
        let i_vec = i.to_be_bytes().to_vec();
        db.insert(
            path,
            &i_vec,
            Element::new_item(i_vec.to_vec()),
            INSERT_OPTIONS,
            Some(&transaction),
            grove_version,
        )
            .unwrap()
            .expect("successfully inserted values");
    }
}

fn insert_range_ref_double_values_db(db: &GroveDb, path: &[&[u8]], ref_key: &[u8], min_i: u32, max_i: u32, transaction: &Transaction, grove_version: &GroveVersion)
{
    for i in min_i..=max_i {
        let i_vec = i.to_be_bytes().to_vec();
        let value = i * 2;
        let value_vec = value.to_be_bytes().to_vec();
        db.insert(
            path,
            &i_vec,
            Element::new_reference(ReferencePathType::AbsolutePathReference(vec![
                MAIN_ΚΕΥ.to_vec(),
                ref_key.to_vec(),
                value_vec.to_vec()
            ])),
            INSERT_OPTIONS,
            Some(&transaction),
            grove_version,
        )
            .unwrap()
            .expect("successfully inserted values");
    }
}

fn insert_empty_sum_tree_db(db: &GroveDb, path: &[&[u8]], key: &[u8], grove_version: &GroveVersion)
{
    db.insert(path, key, Element::empty_sum_tree(), INSERT_OPTIONS, None, grove_version)
        .unwrap()
        .expect("successfully inserted tree");
}
fn insert_sum_element_db(db: &GroveDb, path: &[&[u8]], min_i: u32, max_i: u32, transaction: &Transaction, grove_version: &GroveVersion)
{
    for i in min_i..=max_i {
        //let value : u32 = i;
        let value = i as u64;
        //let value: u64 = 1;
        let i_vec = i.to_be_bytes().to_vec();
        db.insert(
            path,
            &i_vec,
            Element::new_sum_item(value as SumValue),
            INSERT_OPTIONS,
            Some(&transaction),
            grove_version,
        )
            .unwrap()
            .expect("successfully inserted values");
    }
}
fn generate_random_path(prefix: &str, suffix: &str, len: usize) -> String {
    let random_string: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect();
    format!("{}{}{}", prefix, random_string, suffix)
}

fn query_db(db: &GroveDb, path: &[&[u8]], key: Vec<u8>, grove_version: &GroveVersion) {
    let path_vec: Vec<Vec<u8>> = path.iter()
        .map(|&slice| slice.to_vec())
        .collect();

    let mut query = Query::new();
    query.insert_key(key.clone());

    let path_query = PathQuery::new_unsized(path_vec, query.clone());

    let (elements, _) = db
        .query_item_value(&path_query, true, false, true, None, grove_version)
        .unwrap()
        .expect("expected successful get_path_query");

    for value in elements.into_iter() {
        if path.iter().any(|&p| p == b"grade") {
            // Convert the byte array back to an integer
            let grade = i32::from_be_bytes(value.try_into().unwrap());
            println!("Value: {}", grade);
        } else {
            // Convert the byte array back to a string
            let name = String::from_utf8(value).unwrap();
            println!("Value: {}", name);
        }
    }

    let proof = db.prove_query(&path_query, None, grove_version).unwrap().unwrap();
    // Get hash from query proof and print to terminal along with GroveDB root hash.
    let (verify_hash, _) = GroveDb::verify_query(&proof, &path_query, grove_version).unwrap();
    println!("verify_hash: {:?}", hex::encode(verify_hash));
    if verify_hash == db.root_hash(None, grove_version).unwrap().unwrap() {
        println!("Query verified");
    } else {
        println!("Verification FAILED");
    }
}

fn sync_db_demo(
    source_db: &GroveDb,
    target_db: &GroveDb,
    state_sync_info: MultiStateSyncInfo,
    target_tx: &Transaction,
    grove_version: &GroveVersion,
) -> Result<(), grovedb::Error> {
    let app_hash = source_db.root_hash(None, grove_version).value.unwrap();
    let mut state_sync_info = target_db.start_snapshot_syncing(state_sync_info, app_hash, target_tx, CURRENT_STATE_SYNC_VERSION, grove_version)?;

    let mut chunk_queue : VecDeque<Vec<u8>> = VecDeque::new();

    // The very first chunk to fetch is always identified by the root app_hash
    chunk_queue.push_back(app_hash.to_vec());

    while let Some(chunk_id) = chunk_queue.pop_front() {
        let ops = source_db.fetch_chunk(chunk_id.as_slice(), None, CURRENT_STATE_SYNC_VERSION, grove_version)?;
        let (more_chunks, new_state_sync_info) = target_db.apply_chunk(state_sync_info, chunk_id.as_slice(), ops, target_tx, CURRENT_STATE_SYNC_VERSION, grove_version)?;
        state_sync_info = new_state_sync_info;
        chunk_queue.extend(more_chunks);
    }

    Ok(())
}
