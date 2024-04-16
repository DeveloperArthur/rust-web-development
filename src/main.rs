//cargo new project

#![warn(clippy::all)]

use warp::{http::Method, Filter};

//o rust busca um arquivo com mesmo nome do modulo
/* para que os módulos e submódulos sejam visíveis e utilizáveis
em main.rs ou em outros arquivos, você precisa fazer a declaração
mod de todos os módulos e submódulos no main.rs ou no arquivo que
serve como ponto de entrada da sua aplicação. Isso permite que você
use use crate::nome_do_modulo::nome_do_submodulo::nome_da_funcao para
acessar funções e outros itens dentro desses módulos e submódulos. */
mod routes;
mod types;

/*Incluímos store e profanity com base no nome do arquivo que fornecemos aos arquivos nos quais
a lógica está armazenada e está no mesmo nível de hierarquia do arquivo main.rs. Portanto, não
precisamos de um pub mod especial {} dentro de profanity.rs ou store.rs. */
mod profanity;
mod store;

//importando lib de erro local
use handle_errors::return_error;

//Tokio é o Runtime que iremos utilizar, é um runtime assincrono
//serve tanto para fazer servidor ser multithreading (nosso caso)
//como também para fazer I/O assincrono, entre outras coisas
//E estamos utilizando warp para ser o nosso framework web
#[tokio::main]
async fn main() {
    //temos que colocar uma espera por trás da nova função, porque a abertura da conexão
    //com o banco de dados é assíncrona e pode falhar
    let store =
        store::Store::new("postgresql://root:root@localhost:5432/rustwebdev?sslmode=disable").await;
    let store_filter = warp::any().map(move || store.clone());

    //Configuração de CORS
    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods(&[Method::PUT, Method::DELETE, Method::GET, Method::POST]);

    let get_questions = warp::get()
        .and(warp::path("questions"))
        //Usa path::end para sinalizar que ouvimos exatamente /question (e não /question/further/params, por exemplo)
        .and(warp::path::end())
        .and(warp::query()) //Parâmetro params do método get_questions
        .and(store_filter.clone()) //Parâmetro store do método get_questions
        .and_then(routes::question::get_questions);

    let get_question_by_id = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(routes::question::get_question_by_id);

    let add_question = warp::post()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::question::add_question);

    let update_question = warp::put()
        .and(warp::path("questions"))
        //Adiciona um parâmetro String, para que o filtro seja acionado para /questions/1234, por exemplo
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::question::update_question);

    let delete_question = warp::delete()
        .and(warp::path("questions"))
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(routes::question::delete_question);

    let add_answer = warp::post()
        .and(warp::path("answers"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::form())
        .and_then(routes::answer::add_answer);

    let routes = get_questions
        .or(get_question_by_id)
        .or(add_question)
        .or(update_question)
        .or(delete_question)
        .or(add_answer)
        .with(cors)
        .recover(return_error);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
