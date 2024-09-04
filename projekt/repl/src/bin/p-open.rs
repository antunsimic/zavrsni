use std::collections::VecDeque;
use std::path::Path;
use grovedb::{operations::insert::InsertOptions, Element, GroveDb, PathQuery, Query, Transaction};
use grovedb::reference_path::ReferencePathType;
use rand::{distributions::Alphanumeric, Rng, };
use grovedb::element::SumValue;
use grovedb::replication::CURRENT_STATE_SYNC_VERSION;
use grovedb::replication::MultiStateSyncInfo;
use grovedb_version::version::GroveVersion;
use std::time::Instant;


const MAIN_ΚΕΥ: &[u8] = b"key_main";
const MAIN_ΚΕΥ_EMPTY: &[u8] = b"key_main_empty";

const KEY_INT_0: &[u8] = b"key_int_0";
const KEY_INT_1: &[u8] = b"key_int_1";
const KEY_INT_2: &[u8] = b"key_int_2";
const KEY_INT_REF_0: &[u8] = b"key_int_ref_0";
const KEY_INT_A: &[u8] = b"key_sum_0";
const ROOT_PATH: &[&[u8]] = &[];


const SEMESTER_1: &[u8] = b"semester_1";
const SEMESTER_2: &[u8] = b"semester_2";
const SEMESTER_3: &[u8] = b"semester_3";
const SEMESTER_4: &[u8] = b"semester_4";
const SEMESTER_5: &[u8] = b"semester_5";
const SEMESTER_6: &[u8] = b"semester_6";

const COURSES_SEMESTER_1: &[&[u8]] = &[b"maths", b"programming", b"physics"];
const COURSES_SEMESTER_2: &[&[u8]] = &[b"data_structures", b"algorithms", b"discrete_math"];
const COURSES_SEMESTER_3: &[&[u8]] = &[b"databases", b"operating_systems", b"computer_networks"];
const COURSES_SEMESTER_4: &[&[u8]] = &[b"software_engineering", b"theory_of_computation", b"artificial_intelligence"];
const COURSES_SEMESTER_5: &[&[u8]] = &[b"machine_learning", b"computer_graphics", b"distributed_systems"];
const COURSES_SEMESTER_6: &[&[u8]] = &[b"cyber_security", b"cloud_computing", b"big_data"];


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
    let db_path = String::from("../tutorial-storage/performance");
//    let db_path = generate_random_path("../performance-test-storage/", "/db", 24);
    let db = GroveDb::open(db_path).unwrap();

    // Setup
    let start = Instant::now();
    insert_empty_tree_db(&db, &[], b"tree1", &grove_version);
    insert_empty_tree_db(&db, &[b"tree1"], b"tree2", &grove_version);
    insert_empty_tree_db(&db, &[b"tree1", b"tree2"], b"tree3", &grove_version);
    let constant = "a".repeat(500);
    println!("Setup time: {:?}", start.elapsed());
/*
    // Insertion
    let start = Instant::now();
    for i in 0..500 {
        let key = (i as i32).to_be_bytes().to_vec();
        db.insert::<&[u8], _>(&[], &key, Element::new_item(constant.clone().into_bytes()), INSERT_OPTIONS, None, &grove_version).unwrap().expect("inserted");
        db.insert(&[b"tree1"], &key, Element::new_item(constant.clone().into_bytes()), INSERT_OPTIONS, None, &grove_version).unwrap().expect("inserted");
        db.insert(&[b"tree1", b"tree2"], &key, Element::new_item(constant.clone().into_bytes()), INSERT_OPTIONS, None, &grove_version).unwrap().expect("inserted");
        db.insert(&[b"tree1", b"tree2", b"tree3"], &key, Element::new_item(constant.clone().into_bytes()), INSERT_OPTIONS, None, &grove_version).unwrap().expect("inserted");
    }
    println!("Insertion time: {:?}", start.elapsed());

    // Querying
    let start = Instant::now();
    for _ in 0..3 {
        for i in 0..200 {
            let key = (i as i32).to_be_bytes().to_vec();
            query_db(&db, &[], key.clone(), &grove_version);
            query_db(&db, &[b"tree1"], key.clone(), &grove_version);
            query_db(&db, &[b"tree1", b"tree2"], key.clone(), &grove_version);
            query_db(&db, &[b"tree1", b"tree2", b"tree3"], key.clone(), &grove_version);
        }
    }
    println!("Querying time: {:?}", start.elapsed());
*/
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
    let path_vec: Vec<Vec<u8>> = path.iter().map(|&slice| slice.to_vec()).collect();
    let mut query = Query::new();
    query.insert_key(key.clone());
    let path_query = PathQuery::new_unsized(path_vec, query.clone());
    db.query_item_value(&path_query, true, false, true, None, grove_version).unwrap().expect("expected successful get_path_query");
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
