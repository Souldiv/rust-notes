use std::{cell::{RefCell}, collections::{HashMap}};
use maplit::hashmap;
use std::rc::Rc;

use std::io;

fn dfs(list: &RefCell<Vec<isize>>, i: isize){
    if i == 9 {
        ()
    }
    else {
        list.borrow_mut().push(i);
        dfs(list, i + 1);
    }
}

fn combinatorial_example(result: &RefCell<Vec<String>>, n: usize, cur_path: String) {
    if cur_path.len() == n {
        result.borrow_mut().push(cur_path);
        return;
    }

    for path in vec!["a", "b"].iter() {
        let new_path = cur_path.clone() + path;
        combinatorial_example(result, n, new_path);
    }
}

fn create_phone_number_hm<'a>() -> HashMap<char, Vec<&'a str>> {

    let my_hashmap: HashMap<char, Vec<&str>> = hashmap! {
        '2' => vec!["a", "b", "c"],
        '3' => vec!["d", "e", "f"],
        '4' => vec!["g", "h", "i"],
        '5' => vec!["j", "k", "l"],
        '6' => vec!["m", "n", "o"],
        '7' => vec!["p", "q", "r", "s"],
        '8' => vec!["t", "u", "v"],
        '9' => vec!["w", "x", "y","z"]

    };

    my_hashmap

}

fn combo_phno(mhp: &HashMap<char, Vec<&str>>, cur_path: String, result: &RefCell<Vec<String>>, input: &String){

    if cur_path.len() == input.len() {
        result.borrow_mut().push(cur_path);
        return;
    }

    for path in mhp.get(&input.chars().nth(cur_path.len()).unwrap()).unwrap() {
        let new_path = cur_path.clone() + path;
        combo_phno(mhp, new_path, result, input);
    }
}

fn check_brackets(open_bracket_count: usize, 
                close_bracket_count: usize, 
                start_index: usize,
                n: usize,
                cur_path: String,
                res: &RefCell<Vec<String>>) {

            if start_index == 2*n {
                res.borrow_mut().push(cur_path.clone());
                return;
            }
            if open_bracket_count < n {  
                check_brackets(open_bracket_count + 1, 
                    close_bracket_count,
                    start_index+1,
                    n,
                    cur_path.clone() + &String::from("("),
                    res);
            }
            if close_bracket_count < open_bracket_count {

                check_brackets(open_bracket_count,
                    close_bracket_count + 1,
                    start_index + 1,
                    n,
                    cur_path.clone() + &String::from(")"),
                    res);
            }
            return;
}

fn generate_permutations(cur_path: &RefCell<Vec<char>>, res: &RefCell<Vec<String>>, used: &RefCell<Vec<bool>>, s: Rc<String>) {
    if cur_path.borrow().len() == s.len(){
        let val_to_string: String = cur_path.borrow().iter().collect();
        res.borrow_mut().push(val_to_string);
    }

    for (i, ch) in s.chars().enumerate(){
        if used.borrow()[i]{
            continue;
        }
        used.borrow_mut()[i] = true;
        cur_path.borrow_mut().push(ch);
        generate_permutations(cur_path, res, used, s.clone());
        cur_path.borrow_mut().pop();
        used.borrow_mut()[i] = false;
    }

}

fn min_of_ways(sum: isize, input: &RefCell<Vec<isize>>, amount: isize) -> isize{
    if sum == amount {
        return 0;
    }
    else if sum > amount {
        return isize::MAX;
    }
    
    let mut ans = isize::MAX;
    for i in input.borrow().iter() {
        let result = min_of_ways(sum + *i, input, amount);
        if result == isize::MAX {
            continue;
        }
        
        ans = std::cmp::min(result + 1, ans);
    }

    
    ans

}

fn no_of_ways(s: &String, start_index: usize) -> usize{
    if start_index == s.len() {
        return 1;
    }


    let mut ways: usize = 0;
    if s[start_index..start_index+1].parse::<usize>().unwrap() == 0{
        return ways;
    }

    ways += no_of_ways(&s, start_index + 1);

    if start_index + 2 <= s.len(){
        let num = s[start_index..start_index+2].parse::<usize>().unwrap();
        if 10 <=  num && num <= 26 {
            ways += no_of_ways(s, start_index + 2 );
        }
    }

    return ways;
}

pub fn backtracking(){
    let result: RefCell<Vec<String>> = RefCell::new(vec![]);

    // combo problem example
    // combinatorial_example(&result, 4, String::from(""));
    // println!("{:?}", result);

    //combo problem phone number
    // let my_hashmap = create_phone_number_hm();
    // let input = String::from("56");

    // combo_phno(&my_hashmap, String::from(""), &result, &input);
    // print!("{:?}\n", result);

    // generating
    // check_brackets(0, 0, 0, 3, String::from(""), &result);
    // print!("{:?}\n", result);

    // generating all permutations
    // let s = Rc::new(String::from("abca"));
    // let cur_path: RefCell<Vec<char>> = RefCell::new(vec![]);
    // let used: RefCell<Vec<bool>> = RefCell::new(vec![]);

    // for _ in 0..s.clone().len(){
    //     used.borrow_mut().push(false);
    // }

    // generate_permutations(&cur_path, &result, &used, s.clone());
    // println!("Result after generating permutations: {:?}", result);

    // fewest number of ways to generate denomination
    // let input: RefCell<Vec<isize>> = RefCell::new(vec![1, 2, 3]);
    // let amount = 9;
    // let ans = min_of_ways(0, &input, amount);
    // println!("Answer for {:?} is {ans}", input);

    // no of ways to decode a message
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");
    
    input = input.trim().to_string();
    println!("{}", no_of_ways(&input, 0));

}