# 🔍 Log Analyser

> Analisador de arquivos de log de alta performance escrito em Rust — processa milhões de linhas por segundo, detecta padrões, extrai métricas e dispara alertas configuráveis.

[![Rust](https://img.shields.io/badge/Rust-1.75%2B-orange?logo=rust)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Build](https://img.shields.io/badge/build-passing-brightgreen)]()

---

## ✨ Funcionalidades

* ⚡ **Processamento paralelo** via Rayon — utiliza todos os núcleos da CPU
* 🧠 **10+ padrões nomeados** — erros de conexão, falhas de autenticação, erros SQL, picos de recurso e mais
* 📊 **Métricas detalhadas** — contagem por nível, erros por hora, principais IPs e usuários
* 🚨 **Alertas configuráveis** — limites personalizados de taxa de erro via flags da CLI
* 💾 **Exportação em JSON** — envie resultados para dashboards ou outras ferramentas
* 🎨 **Saída colorida no terminal** — leitura clara e rápida
* 🧪 **Gerador de exemplo embutido** — teste sem precisar de um log real

---

## 📦 Instalação

### Pré-requisitos

* Rust 1.75+

```bash
# Clonar o repositório
git clone https://github.com/yourusername/log-analyser.git
cd log-analyser

# Gerar binário otimizado
cargo build --release

# (Opcional) instalar globalmente
cargo install --path .
```

---

## 🚀 Uso

```bash
# Executar com seu próprio arquivo de log
cargo run --release -- -f /var/log/app.log

# Gerar um log de exemplo e analisar
cargo run --release -- --sample

# Exportar resultados em JSON
cargo run --release -- -f app.log -o report.json

# Limites personalizados de alerta (aviso em 5%, crítico em 15%)
cargo run --release -- -f app.log --warn-threshold 5 --error-threshold 15
```

### Todas as opções

```
Uso: log-analyser [OPÇÕES]

Opções:
  -f, --file <FILE>               Caminho do arquivo de log a ser analisado
      --sample                    Gera e analisa um log de exemplo embutido
  -o, --output <FILE>             Exporta os resultados em JSON para este caminho
      --error-threshold <FLOAT>   % de taxa de erro para alerta crítico  [padrão: 20]
      --warn-threshold <FLOAT>    % de taxa de erro para alerta de aviso [padrão: 10]
      --top-ips <N>               Mostra os N principais IPs             [padrão: 5]
      --top-users <N>             Mostra os N principais usuários        [padrão: 5]
  -q, --quiet                     Suprime saídas informativas
  -h, --help                      Exibe ajuda
  -V, --version                   Exibe versão
```

---

## 📊 Exemplo de Saída

```
🔍 Log Analyser
═══════════════════════════════════════

📊  Métricas do Arquivo
─────────────────────────────
  Total de linhas : 30
  Linhas válidas  : 30
  Tempo de análise: 1.42ms
  Linhas/segundo  : 21126

📈  Por Nível de Log
─────────────────────────────
  INFO    :   13 (43.3%)
  WARNING :    7 (23.3%)
  ERROR   :    9 (30.0%)
  DEBUG   :    4 (13.3%)

🎯  Padrões Detectados
─────────────────────────────
  erros de conexão        → 3 ocorrências
  falhas de autenticação  → 2 ocorrências
  picos de recurso        → 2 ocorrências
  consultas lentas        → 1 ocorrência
  erros SQL               → 1 ocorrência

⏱️  Erros por Hora
─────────────────────────────
  2024-02-02 10 │ ███████████ 9

🌐  Principais IPs
─────────────────────────────
  192.168.1.100        3 requisições
  10.0.0.99            2 requisições
  203.0.113.42         1 requisição

👤  Principais Usuários
─────────────────────────────
  user123              2 eventos
  admin                2 eventos
  alice                1 evento

🚨  Alertas
─────────────────────────────
  ❌ CRÍTICO  Taxa de erro crítica: 30.0% (limite: 20%)
  ⚠️  Falhas de autenticação detectadas (2)
  📈  Picos repetidos de recurso detectados (2)
```

---

## 🗂️ Estrutura do Projeto

```
log-analyser/
├── src/
│   ├── main.rs       # Ponto de entrada & configuração da CLI
│   ├── cli.rs        # Definição dos argumentos (clap derive)
│   ├── analyser.rs   # Parsing principal & agregação paralela
│   ├── output.rs     # Impressão no terminal & cores
│   └── sample.rs     # Gerador de log de exemplo
├── Cargo.toml
├── .gitignore
├── LICENSE
└── README.md
```

---

## 🧩 Padrões Detectados

| Padrão                | Descrição                                     |
| --------------------- | --------------------------------------------- |
| `connection_errors`   | Conexão perdida / falhou / timeout / recusada |
| `connection_failures` | Mensagens "Failed to connect"                 |
| `rate_limit_hits`     | Eventos de limite de requisição               |
| `auth_failures`       | Falhas de autenticação / login                |
| `unauthorized_access` | 401 / 403 / Não autorizado                    |
| `slow_queries`        | Consultas lentas ao banco                     |
| `disk_warnings`       | Avisos de pouco espaço em disco               |
| `resource_spikes`     | Picos de CPU / memória                        |
| `critical_crashes`    | Panics, ponteiro nulo, OOM                    |
| `sql_errors`          | Exceções / erros SQL                          |

---

## 📐 Formato de Log

O analisador espera linhas neste formato (comum na maioria dos frameworks):

```
YYYY-MM-DD HH:MM:SS LEVEL mensagem...
```

Exemplo:

```
2024-02-02 10:00:45 ERROR Failed to connect to payment gateway: timeout
2024-02-02 10:01:10 WARNING Slow query detected: 2.5s
2024-02-02 10:02:00 INFO  User login: alice
```

Níveis suportados: `INFO`, `WARNING`, `WARN`, `ERROR`, `CRITICAL`, `DEBUG`

---

## 🛠️ Desenvolvimento

```bash
# Executar em modo debug
cargo run -- --sample

# Rodar testes
cargo test

# Verificar avisos
cargo clippy

# Formatar código
cargo fmt

# Gerar binário de release
cargo build --release
# Binário em: ./target/release/log-analyser
```

---

## 📄 Licença

MIT © Kauã Gabriel
[https://github.com/Kaua-KGzin/](https://github.com/Kaua-KGzin/)
