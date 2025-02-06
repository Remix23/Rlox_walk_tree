
fn report (line : i32, loc : String, msg : String) {
    println!("[line {}] Error {}: {}", line, loc, msg);

}

pub fn err (line : i32, msg : String) -> bool {
    report(line,  "".to_string(), msg);
    // TODO: Rewrite the 
    return true;
}  
