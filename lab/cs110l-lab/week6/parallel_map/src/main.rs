use crossbeam_channel::{self, Receiver, Sender};
use std::{thread, time, vec};

fn parallel_map<T, U, F>(mut input_vec: Vec<T>, num_threads: usize, f: F) -> Vec<U>
where
    F: FnOnce(T) -> U + Send + Copy + 'static,
    T: Send + 'static,
    U: Send + 'static + Default,
{
    let vec_length = input_vec.len();
    let mut output_vec: Vec<U> = Vec::with_capacity(vec_length);
    // TODO: implement parallel map!
    let (raw_sender, raw_receiver)= crossbeam_channel::unbounded();
    let (cooked_sender, cooked_receiver)= crossbeam_channel::unbounded();
    // let (status_sender, status_receiver) = crossbeam_channel::unbounded();
    
    let mut threads = Vec::new();
    // let vec_length = input_vec.len();
    for _ in 0..num_threads {
        let thread_receiver:Receiver<(usize, T)> = raw_receiver.clone();
        let processed_sender: Sender<(usize, U)> = cooked_sender.clone();
        threads.push(thread::spawn(move || {
            
            while let Ok((i, ent)) = thread_receiver.recv() {        
                processed_sender.send((i, f(ent))).expect("Failed to send f(data)");
                // if i == 0 {
                //     drop(processed_sender);
                // }
            }
            drop(processed_sender);


            // while let Ok(has_finished) = status_receiver.recv() {
            //     drop(processed_sender);
            // }
        }
        
        ))
    }
    drop(cooked_sender);

    
    // assert_eq!(vec_length, output_vec.len());

    let ori_data_sender = raw_sender.clone();
    for index in 0..vec_length{
        output_vec.push(Default::default());
        let entry = input_vec.pop().unwrap();
        ori_data_sender.send((vec_length - 1 - index, entry)).expect("Failed to send raw data");
    }

    drop(ori_data_sender);
    drop(raw_sender);
    

    while let Ok((i, ent)) = cooked_receiver.recv() {        
        output_vec[i] = ent;       
    }


    // drop(status_sender);

    for thread in threads {
        thread.join().expect("Failed to launch a thread");
    }
    
    output_vec
}

fn main() {
    let v = vec![6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 12, 18, 11, 5, 20];
    let squares = parallel_map(v, 10, |num| {
        println!("{} squared is {}", num, num * num);
        thread::sleep(time::Duration::from_millis(500));
        num * num
    });
    println!("squares: {:?}", squares);
}
