use ::std::collections::HashMap;

pub(crate) fn _main() {
    let mut arr = vec![
        1, 2, 3, 4, 5, 5, 3, 4, 56, 4, 6, 7, 4, 23, 34, 56, 4, 65, 4, 5, 4, 5, 42, 2, 3, 4, 5, 6,
        7, 7, 7, 5, 4, 3, 3, 3,
    ];

    arr = sort_vec_arr(arr);
    println!("Sorted Array: {:?}", arr);
    let median = find_median(&arr);
    let mode = my_mode(&arr);

    println!("Median: {}", median);
    println!("Mode: {}", mode);
    println!("Done!");

    //sort the arr
    fn sort_vec_arr(mut arr: Vec<i32>) -> Vec<i32> {
        arr.sort();
        arr
    }
    //find the median
    fn find_median(sorted_arr: &Vec<i32>) -> i32 {
        let len = sorted_arr.len();
        let mid = len / 2;
        if len % 2 == 0 {
            (sorted_arr[mid] + sorted_arr[mid - 1]) / 2
        } else {
            sorted_arr[mid]
        }
    }
    //find the mode
    fn my_mode(arr: &Vec<i32>) -> i32 {
        let mut map: HashMap<i32, i32> = HashMap::new();
        for num in arr {
            let count = map.entry(*num).or_insert(0);
            *count += 1;
        }

        let mut max = 0;
        let mut mode = 0;
        for (key, value) in map {
            if value > max {
                max = value;
                mode = key;
            }
        }
        mode
    }
}
