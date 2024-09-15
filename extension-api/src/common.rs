use std::vec::Vec;
use std::sync::{Mutex, Condvar};
use std::ops::DerefMut;
pub fn wait_job<T>(container_mutex:&Mutex<Vec<T>>, condvar: &Condvar) -> Vec<T> 
    where T: Clone {

    let items_added_since_previous_run = get_existing_items(container_mutex);

    if !items_added_since_previous_run.is_empty() {

        return items_added_since_previous_run;
    }

    let mut new_items = condvar.wait(container_mutex.lock().unwrap()).unwrap();

    return copy_and_clear_container_items(new_items.deref_mut());

}

fn get_existing_items<T>(container_mutex: &Mutex<Vec<T>>) -> Vec<T> 
where T: Clone {

    let mut items = container_mutex.lock().unwrap();
    let contents: &mut Vec<T> = items.deref_mut();
    return copy_and_clear_container_items(contents);

}

fn copy_and_clear_container_items<T>(contents: &mut Vec<T>) -> Vec<T> where T: Clone {

    let mut new_vector: Vec<T> = Vec::new();
    new_vector.clone_from(contents);
    contents.clear();
    return new_vector;
}
