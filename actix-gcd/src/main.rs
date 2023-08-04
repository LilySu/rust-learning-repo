use actix_web::{web, App, HttpResponse, HttpServer};
use serde::Deserialize;

#[derive(Deserialize)] // attribute above a type definition tells serde crate to examine the type when the program is compiled
// and automatically generate code to parse a value of this type from data in teh format that HTML forms use for POST requests.
// the Deserialize attribute will let you parse a GcdParameters value from almost any sort of structured data. 
// serde crate provides a Serialize attribute that generates code to the reverse, taking Rust values and writing them out in a structured format.
struct GcdParameters { // defines a new type with 2 fields, the argument type our gcd function expects
    n: u64,
    m: u64,
}

fn main() {
    let server = HttpServer::new(|| { // || is the argument we are passing to HttpServer::new
        // closure expression - value that can be called as if it were a function, this one takes no arguments but if it did, their names would appear between ||
        App::new() // creates a new empty App, calls its route method
            .route("/", web::get().to(get_index))
            // call to route establishing web::post().to(post_gcd) as the handler for the path "/gcd"
            .route("/gcd", web::post().to(post_gcd))
    });
    // when a server is started, Actix starts a pool of threads to handle incoming requests.
    // each thread calls closure to get a fresh copy of the App value to tell it how to route or handle requests.
    println!("Serving on http://localhost:3000...");
    server
        .bind("127.0.0.1:3000").expect("error binding server to address")
        .run().expect("error running server");
}

fn get_index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(
            r#"
                <title>GCD Calculator</title>
                <form action="/gcd" method="post">
                <input type = "text" name="n"/>
                <input type = "text" name="m"/>
                <button type = "submit">Compute GCD</button>
                </form>
            "#,
        )
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0); // if not true, terminates the program with helpful message
    while m != 0 { // requires brackets not parentesis
        if m < n {
            let t: u64 = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n // return
}

// handler function to take care of the "Compute GCD" button to do something.
// function post_gcd takes the argument form of type web::Form<GcdParameters>
// any type web::Form<T> can be extracted from an HTTP request, since HTML form Post can be deserialized
// because Actix can deserialize it from form data with #[derive(Deserialize)] attribute
fn post_gcd(form: web::Form<GcdParameters>) -> HttpResponse {
    // HTTP 400 BAD REQUEST error if both fields are 0
    if form.n == 0 || form.m == 0{
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("Computing the GCD with zero is boring.");
    }
    // constructs a response to the request
    let response = 
        // macro instead of writing text to standard output like print!, it returns a string
        format!("The greatest common divisor of the numbers {} and {} \
                is <b>{}</b>\n",
                form.n, form.m, gcd(form.n, form.m));
    // Once text of the response is obtained, the text is wrapped in a HTTP 200 OK response
    HttpResponse::Ok()
        // set content type
        .content_type("text/html")
        // returns the message to be delivered to the sender
        .body(response)
}
