use crate::types::{
    answer::{Answer, AnswerId},
    question::{Question, QuestionId},
};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct Store {
    pub questions: Arc<RwLock<HashMap<QuestionId, Question>>>,
    pub answers: Arc<RwLock<HashMap<AnswerId, Answer>>>,
}

impl Store {
    pub fn new() -> Self {
        Store {
            /* Sempre que o método init() é chamado, ele vai ler um json e retornar
            um novo HashMap, como estamos em um servidor multithreading, pra evitar
            de criar um novo HashMap para cada request, usamos o Arc, o Arc garante
            que apenas uma instância do HashMap seja criada e que seja compartilhada
            entre as threads/requests, ou seja, o Arc permite que você tenha vários
            ponteiros para o mesmo map, cada uma delas é uma referência segura e
            independente, gerenciada pelo Arc. Isso significa que as threads podem
            acessar o mesmo conjunto de dados, mas cada uma tem sua própria
            referência segura, porque se uma request add um elemento no map isso tem
            que ser refletido nas outras requests, e sem o Arc, cada thread teria seu
            próprio map, a thread A iria deletar, e isso não iria se refletir na thread B...

            O Arc mantém uma contagem de ponteiros, enquanto a contagem for menor que
            zero, o valor não será destruído, se a contagem chegar a zero valor é
            destruído, o valor sai de escopo, é o conceito de ownership do Rust.

            E estamos usando o mutex de read-write porque, como temos diversos ponteiros
            para o mesmo valor (graças ao Arc), precisamos evitar que race conditions
            aconteçam... basta envolver um RwLock em torno de um HashMap e você está
            configurando a estrutura para permitir operações de leitura concorrentes (múltiplas
            threads podem ler ao mesmo tempo) e operações de escrita exclusivas (apenas uma
            thread pode escrever por vez, e outras threads são bloqueadas durante a escrita). */
            questions: Arc::new(RwLock::new(Self::init())),
            answers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    fn init() -> HashMap<QuestionId, Question> {
        let file = include_str!("../questions.json");
        serde_json::from_str(file).expect("can't read questions.json")
    }
}
