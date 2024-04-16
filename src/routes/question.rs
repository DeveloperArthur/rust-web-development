use crate::profanity::check_profanity;
use crate::store::Store;
use crate::types::pagination::Pagination;
use crate::types::question::NewQuestion;
use crate::types::{pagination::extract_pagination, question::Question};
use std::collections::HashMap;
use warp::http::StatusCode;

pub async fn get_questions(
    params: HashMap<String, String>,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    println!("INFO: querying questions");
    let mut pagination = Pagination::default();

    if !params.is_empty() {
        println!("INFO: pagination = true");
        pagination = extract_pagination(params)?;
    }

    match store
        .get_questions(pagination.limit, pagination.offset)
        .await
    {
        Ok(res) => Ok(warp::reply::json(&res)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

pub async fn get_question_by_id(
    id: i32,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    match store.get_question_by_id(id).await {
        Ok(res) => Ok(warp::reply::json(&res)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

pub async fn add_question(
    store: Store,
    new_question: NewQuestion,
) -> Result<impl warp::Reply, warp::Rejection> {
    //invés de fazer as chamadas HTTP sequenciais e bloqueantes, usamos tokio para chamadas paralelas, I/O não bloqueante
    let title = tokio::spawn(check_profanity(new_question.title));
    let content = tokio::spawn(check_profanity(new_question.content));

    let (title, content) = (title.await.unwrap(), content.await.unwrap());

    if title.is_err() {
        return Err(warp::reject::custom(title.unwrap_err()));
    }

    if content.is_err() {
        return Err(warp::reject::custom(content.unwrap_err()));
    }

    let question = NewQuestion {
        title: title.unwrap(),
        content: content.unwrap(),
        tags: new_question.tags,
    };

    match store.add_question(question).await {
        Ok(question) => Ok(warp::reply::json(&question)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

pub async fn update_question(
    id: i32,
    store: Store,
    question: Question,
) -> Result<impl warp::Reply, warp::Rejection> {
    let title = tokio::spawn(check_profanity(question.title));
    let content = tokio::spawn(check_profanity(question.content));

    let (title, content) = (title.await.unwrap(), content.await.unwrap());

    if title.is_err() {
        return Err(warp::reject::custom(title.unwrap_err()));
    }

    if content.is_err() {
        return Err(warp::reject::custom(content.unwrap_err()));
    }

    let question = Question {
        id: question.id,
        title: title.unwrap(),
        content: content.unwrap(),
        tags: question.tags,
    };

    match store.update_question(question, id).await {
        Ok(res) => Ok(warp::reply::json(&res)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

pub async fn delete_question(id: i32, store: Store) -> Result<impl warp::Reply, warp::Rejection> {
    match store.delete_question(id).await {
        Ok(_) => Ok(warp::reply::with_status(
            format!("Question {} deleted", id),
            StatusCode::OK,
        )),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

// pub async fn get_question_by_id(
//     id: String,
//     store: Store,
// ) -> Result<impl warp::Reply, warp::Rejection> {
//     // esse await é para aguardar a função de leitura, pode ser que um bloqueio esteja
//     // em vigor porque outro processo está acessando os mesmos dados
//     match store.questions.read().await.get(&QuestionId(id)) {
//         Some(question) => Ok(warp::reply::json(question)),
//         None => Err(warp::reject::custom(Error::QuestionNotFound)),
//     }
// }
//
// pub async fn add_question(
//     store: Store,
//     question: Question,
// ) -> Result<impl warp::Reply, warp::Rejection> {
//     // esse await é pra aguardar a função de escrita, sempre que tivermos acesso, podemos
//     // inserir uma nova pergunta no mapa hash subjacente
//     store.questions.write().await.insert(question.id.clone(), question);
//     Ok(warp::reply::with_status("Question added", StatusCode::OK))
// }
//
// pub async fn update_question(
//     id: String,
//     store: Store,
//     question: Question,
// ) -> Result<impl warp::Reply, warp::Rejection> {
//     /* Em vez de apenas escrever no objeto HashMap como no manipulador de rota add_question , estamos solicitando uma
//     referência mutável para a question que estamos tentando acessar, para que possamos alterar o conteúdo dentro dela. */
//     match store.questions.write().await.get_mut(&QuestionId(id)) {
//         Some(q) => *q = question,
//         None => return Err(warp::reject::custom(Error::QuestionNotFound)),
//     }
//     Ok(warp::reply::with_status("Question updated", StatusCode::OK))
// }
