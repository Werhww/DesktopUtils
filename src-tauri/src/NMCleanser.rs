extern crate rust_search;
use rust_search::SearchBuilder;

use tauri::Manager;


#[tauri::command]
fn NMCleanser(){
    /* let search: Vec<String> = SearchBuilder::default()
        .location("~/")
        .search_input("what to search")
        .limit(1000) // results to return
        .ext("extension")
        .strict()
        .depth(1)
        .ignore_case()
        .hidden()
        .build()
        .collect();

    for path in search {
        println!("{}", path);
    } */

    println!("Hello from NMCleanser.rs");
    return 123
}