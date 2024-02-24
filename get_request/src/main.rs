
use std::io::Read;
fn main() ->Result<(),reqwest::Error> {

    // blocking ==> use to wait for response then move to next lines
    let mut result = reqwest::blocking::get("https://httpbin.org/get")?;
    let mut body = String::new();

    // read_to_string will append data from result to body
    result.read_to_string(&mut body);


    println!("status {:?}", result.status());
    println!("headers {:?}", result.headers());
    println!("body:\n{}", body);
    Ok(())



}
// -----------------------------------------------result-----------------------------------------------------
// status 200
// headers {"date": "Sat, 24 Feb 2024 11:42:10 GMT", "content-type": "application/json", "content-length": "221", "connection": "keep-alive", "server": "gunicorn/19.9.0", "access-control-allow-origin": "*", "access-control-allow-credentials": "true"}
// body:
// {
//   "args": {},
//   "headers": {
//     "Accept": "*/*",
//     "Host": "httpbin.org",
//     "X-Amzn-Trace-Id": "Root=1-65d9d612-5cd3509e432155146f4bc950"
//   },
//   "origin": "152.58.92.223",
//   "url": "https://httpbin.org/get"
// }