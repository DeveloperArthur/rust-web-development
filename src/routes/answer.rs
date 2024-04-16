use crate::profanity::check_profanity;
use crate::store::Store;
use crate::types::answer::NewAnswer;
use reqwest::StatusCode;
use warp::reply::Reply;

pub async fn add_answer(
    store: Store,
    new_answer: NewAnswer,
) -> Result<impl Reply, warp::Rejection> {
    let content = match check_profanity(new_answer.content).await {
        Ok(res) => res,
        Err(e) => return Err(warp::reject::custom(e)),
    };

    let new_answer = NewAnswer {
        content,
        question_id: new_answer.question_id,
    };

    match store.add_answer(new_answer).await {
        Ok(_) => Ok(warp::reply::with_status("Answer added", StatusCode::OK)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

// pub async fn add_answer(
//     id: String,
//     store: Store,
//     params: HashMap<String, String>,
// ) -> Result<impl warp::Reply, warp::Rejection> {
//     let content_answer: String = match params.get("content") {
//         Some(c) => c.to_string(),
//         None => return Err(warp::reject::custom(Error::MissingParameters)),
//     };

//     match store.questions.read().await.get(&QuestionId(id.clone())) {
//         Some(_) => (),
//         None => return Err(warp::reject::custom(Error::QuestionNotFound)),
//     }

//     let answer = Answer {
//         id: AnswerId((store.answers.read().await.len() + 1).to_string()),
//         content: content_answer,
//         question_id: QuestionId(id),
//     };

//     store
//         .answers
//         .write()
//         .await
//         .insert(answer.id.clone(), answer.clone());

//     Ok(warp::reply::json(&answer))
// }
