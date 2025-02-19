pub mod models;
pub mod client;


pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use crate::client::YahooFinanceClient;
    #[test]
    fn test_get_stock_data_usa() {
        let symbol = "AAPL";
        let interval = "1d";
        let result = YahooFinanceClient::get_stock_data(symbol, interval);
        assert!(result.is_ok(), "Erro ao buscar dados de AAPL");
    }

    #[test]
    fn test_get_stock_data_brazil() {
        let symbol = "VALE3.SA";
        let interval = "1d";
        let result = YahooFinanceClient::get_stock_data(symbol, interval);
        assert!(result.is_ok(), "Erro ao buscar dados de VALE3.SA");
    }

    #[test]
    fn test_get_stock_data_germany() {
        let symbol = "BMW.DE";
        let interval = "1wk";
        let result = YahooFinanceClient::get_stock_data(symbol, interval);
        assert!(result.is_ok(), "Erro ao buscar dados de BMW.DE");
    }

    #[test]
    fn test_get_stock_data_invalid() {
        let symbol = "INVALID123";
        let interval = "1mo";
        let result = YahooFinanceClient::get_stock_data(symbol, interval);
        assert!(result.is_err(), "A API deveria falhar para um símbolo inválido");
    }

    #[test]
    fn test_get_dividends_usa() {
        let symbol = "AAPL";
        let interval = "1mo";
        let result = YahooFinanceClient::get_dividends(symbol, interval);
        assert!(result.is_ok(), "Erro ao buscar dividendos de AAPL");

        if let Ok(dividends) = result {
            println!("Dividendos de AAPL: {:?}", dividends);
        }
    }

    #[test]
    fn test_get_dividends_brazil() {
        let symbol = "VALE3.SA";
        let interval = "1mo";
        let result = YahooFinanceClient::get_dividends(symbol, interval);
        assert!(result.is_ok(), "Erro ao buscar dividendos de VALE3.SA");

        if let Ok(dividends) = result {
            println!("Dividendos de VALE3.SA: {:?}", dividends);
        }
    }

    #[test]
    fn test_get_dividends_germany() {
        let symbol = "BMW.DE";
        let interval = "1mo";
        let result = YahooFinanceClient::get_dividends(symbol, interval);
        assert!(result.is_ok(), "Erro ao buscar dividendos de BMW.DE");

        if let Ok(dividends) = result {
            println!("Dividendos de BMW.DE: {:?}", dividends);
        }
    }

    #[test]
    fn test_get_dividends_invalid() {
        let symbol = "INVALID123";
        let interval = "1mo";
        let result = YahooFinanceClient::get_dividends(symbol, interval);
        assert!(result.is_ok(), "Erro ao buscar dividendos de um ativo inválido");

        if let Ok(dividends) = result {
            assert!(dividends.is_empty(), "Ativo inválido deveria retornar lista vazia");
        }
    }
}
