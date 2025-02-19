
# 📈 Yahoo Finance

Uma biblioteca em **Rust** para buscar **cotações e dividendos** de ativos financeiros usando a API do **Yahoo Finance**.  

🚀 Fácil de usar, sem necessidade de credenciais!  

---

## 📦 Instalação  

Adicione a biblioteca ao seu projeto **Cargo.toml**:  

```toml
[dependencies]
yahoo_finance = "0.1"
```

Se estiver testando **localmente**, use:  

```toml
[dependencies]
yahoo_finance_rust = { path = "../yahoo_finance" }
```

---

## 🔍 **Como Usar?**  

### 📌 1️⃣ Buscar Preços de Ações  

```rust
use yahoo_finance_rust::client::YahooFinanceClient;

fn main() {
    match YahooFinanceClient::get_stock_data("AAPL", "1d") {
        Ok(data) => println!("{:?}", data),
        Err(e) => eprintln!("Erro ao buscar dados: {}", e),
    }
}
```

---

### 📌 2️⃣ Buscar Dividendos de uma Empresa  

```rust
use yahoo_finance_rust::client::YahooFinanceClient;

fn main() {
    let symbol = "AAPL"; // Apple Inc.
    let interval = "1mo";

    match YahooFinanceClient::get_dividends(symbol, interval) {
        Ok(dividends) => {
            if dividends.is_empty() {
                println!("Nenhum dividendo encontrado para {}.", symbol);
            } else {
                println!("📈 Dividendos de {}:", symbol);
                for (date, amount) in dividends {
                    println!("📅 Data: {} - 💰 Valor: ${:.2}", date, amount);
                }
            }
        }
        Err(e) => eprintln!("Erro ao buscar dividendos: {}", e),
    }
}
```

---

## 🌍 **Exemplos de Ações Internacionais**  

A biblioteca suporta ações de **diversos países**, basta usar os **códigos corretos**:

| 🌍 País  | 📈 Bolsa       | 🎯 Exemplo de Ativo |
|---------|--------------|------------------|
| 🇺🇸 EUA      | Nasdaq/NYSE | `AAPL` (Apple) |
| 🇧🇷 Brasil   | B3          | `VALE3.SA` (Vale) |
| 🇩🇪 Alemanha | Xetra       | `BMW.DE` (BMW) |
| 🇯🇵 Japão    | TSE         | `7203.T` (Toyota) |

---

## ⚠️ **Erros Comuns e Soluções**  

### ❌ **Erro: "Too Many Requests" (429)**  
🔹 O Yahoo Finance pode bloquear IPs temporariamente se houver **muitas requisições seguidas**.  

✅ **Soluções:**  
- **Aguarde alguns minutos** para o bloqueio expirar  
- **Use uma VPN** ou **troque de rede**  
- **Aumente o tempo de espera entre requisições** (`thread::sleep(Duration::from_secs(5))`)  

---

## 🛠️ **Contribuindo**  

Se quiser contribuir com melhorias, siga os passos:  

```sh
git clone https://github.com/seu-usuario/yahoo_finance_rust.git
cd yahoo_finance_rust
cargo test
```

Sugestões são bem-vindas! 🚀  

---

## 📜 **Licença**  

Este projeto está licenciado sob a **MIT License**.  

---

## ⭐ **Gostou?**  

Se este projeto te ajudou, **deixe uma estrela no GitHub!** ⭐🚀  

[🔗 Repositório no GitHub](https://github.com/gui-g7/Rust-Yahoo-Finance)
