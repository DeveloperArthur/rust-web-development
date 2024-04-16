use handle_errors::Error;
use std::collections::HashMap;

/// Struct Pagination que está sendo extraída dos parâmetros de consulta
#[derive(Default, Debug, PartialEq)]
pub struct Pagination {
    /// Índice do primeiro item que deve ser retornado
    pub limit: Option<u32>,
    /// Índice do último item que deve ser retornado
    pub offset: u32,
}

/// Extração dos parâmetros da query da rota `/questions`
/// # Exemplo
/// Requests do tipo GET para essa rota podem ter uma paginação anexada
/// então só retorna as questions que precisamos
/// `/questions?start=1&end=10`
pub fn extract_pagination(params: HashMap<String, String>) -> Result<Pagination, Error> {
    if params.contains_key("limit") && params.contains_key("offset") {
        return Ok(Pagination {
            limit: Some(
                params
                    .get("limit")
                    .unwrap()
                    .parse::<u32>()
                    .map_err(Error::ParseError)?,
            ),
            offset: params
                .get("offset")
                .unwrap()
                .parse::<u32>()
                .map_err(Error::ParseError)?,
        });
    }
    Err(Error::MissingParameters)
}

//É uma prática comum em Rust ter os testes no mesmo arquivo que o código real
//pelo menos no caso de testes unitários
#[cfg(test)]
mod pagination_test {
    use std::collections::HashMap;

    use super::{extract_pagination, Error, Pagination};

    #[test]
    fn valid_pagination() {
        let mut params = HashMap::new();
        params.insert(String::from("limit"), String::from("1"));
        params.insert(String::from("offset"), String::from("1"));

        let pagination_result = extract_pagination(params);

        let expected = Pagination {
            limit: Some(1),
            offset: 1,
        };

        assert_eq!(pagination_result.unwrap(), expected);
    }

    #[test]
    fn missing_offset_parameter() {
        let mut params = HashMap::new();
        params.insert(String::from("limit"), String::from("1"));

        let pagination_result = format!("{}", extract_pagination(params).unwrap_err());

        let expected = format!("{}", Error::MissingParameters);

        assert_eq!(pagination_result, expected)
    }
}
