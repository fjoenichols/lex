use actix_web::{get, web, App, HttpServer, Responder};
use rand::Rng;
use titlecase::titlecase;

#[get("/")]
async fn index() -> impl Responder {
    "Hello,\n
    Please use one of the following endpoints\n
    /api/passphrase - to generate a passphrase
    /api/word - to generate an English word
    /api/word/<char> - to generate an English word beginning with the supplied character
    /api/words - to display a list of English words\n"
}

#[get("/api")]
async fn api() -> impl Responder {
    "Hello,\n
    Please use one of the following endpoints\n
    /api/passphrase - to generate a passphrase
    /api/word - to generate an English word
    /api/word/<char> - to generate an English word beginning with the supplied character
    /api/words - to display a list of English words\n"
}

#[get("/api/passphrase")]
async fn passphrase() -> impl Responder {
    let word1 = random_word::gen();
    let word2 = random_word::gen();
    let word3 = random_word::gen();
    let random_number = rand::thread_rng().gen_range(0..10);
    let selector = rand::thread_rng().gen_range(0..3);
    match selector {
        0 => format!("{{ \"passphrase\": \"{}{}-{}-{}\" }}\n", titlecase(&word1), &random_number, titlecase(&word2), titlecase(&word3)),
        1 => format!("{{ \"passphrase\": \"{}-{}{}-{}\" }}\n", titlecase(&word1), titlecase(&word2), &random_number, titlecase(&word3)),
        2 => format!("{{ \"passphrase\": \"{}-{}-{}{}\" }}\n", titlecase(&word1), titlecase(&word2), titlecase(&word3), &random_number),
        _ => panic!("error"),
    }
}

#[get("/api/word")]
async fn english_word() -> impl Responder {
    let word = random_word::gen();
    format!("{{ \"word\": \"{}\" }}\n", &word)
}

#[get("/api/words")]
async fn english_words() -> impl Responder {
    let words = random_word::all();
    format!("{{ \"words\": {:#?} }}\n", &words)
}

#[get("/api/word/{letter}")]
async fn english_word_starts_with(letter: web::Path<String>) -> impl Responder {
    if letter.len() > 1 {
        format!("{{ \"error\": \"please only supply one character\" }}\n")
    } else {
        let letter = letter.chars().next().expect("expected only 1 character");
        let word = random_word::gen_starts_with(letter).expect("invalid response");
        format!("{{ \"word\": {:?} }}\n", &word)
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new()
        .service(index)
        .service(api)
        .service(english_word)
        .service(english_words)
        .service(english_word_starts_with)
        .service(passphrase))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}