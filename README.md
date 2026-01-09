
# 🦀 Remind.rs

> A próxima geração do seu bloco de notas, forjada em Rust.

O **Remind.rs** é um ecossistema completo para gestão de lembretes. Ele utiliza uma arquitetura de **segurança máxima** e **performance nativa**, garantindo que seus pensamentos estejam sincronizados, seguros e acessíveis através de uma interface reativa e uma API robusta.

---

## ✨ Funcionalidades Principais

* **🔐 Autenticação Blindada:** Sistema de usuários com hashing **Argon2** e sessões via **JWT**.
* **📝 Gestão de Notas (CRUD):** Criação, leitura, edição e exclusão de notas com suporte a Markdown.
* **👤 Isolamento de Dados:** Cada usuário possui seu próprio cofre de notas, garantido pelo Repository Pattern.
* **🚀 Full-Stack Rust:** Tipagem compartilhada entre o Frontend (Dioxus) e o Backend (Actix), eliminando erros de contrato.
* **💾 Persistência Confiável:** Banco de dados SurrealDB com migrações versionadas.
* **🎨 UI Reativa:** Interface moderna, leve e performática construída inteiramente em Rust.

---

## 🛠️ Roadmap de Desenvolvimento: A Jornada do Ferro

Aqui está o plano de ataque para transformar o **Remind.rs** em realidade.

### 🏁 Fase 1: O Gênese (Fundação)

* [x] **Setup do Workspace:** Configurar o `Cargo.toml` raiz e as crates `Repository`, `Serivoces` e `etc...`.
* [x] **Contratos de Domínio:** Definir as structs `User` e `Note` na crate `models` para uso universal.
* [ ] **Esquema de Dados:** Criar as migrações SQL para tabelas de Usuários e Notas.

### 🧠 Fase 2: A Alma (Lógica e Abstração)

* [x] **Traits de Repositório:** Definir as interfaces assíncronas para manipulação de dados.
* [ ] **Criptografia:** Implementar o módulo de segurança para hashing de senhas.
* [ ] **Infrastructure:** Codar a implementação concreta para o Surreal.

### 🛡️ Fase 3: O Escudo (API e Segurança)

* [ ] **Motor do Backend:** Configurar o servidor Actix e a injeção de dependência via `AppState`.
* [ ] **Guardas de Autenticação:** Criar o Middleware de JWT para proteger as rotas sensíveis.
* [ ] **Handlers de CRUD:** Implementar os endpoints da API com validação rigorosa.

### 🖥️ Fase 4: O Rosto (Interface Dioxus)

* [ ] **Fluxo de Auth:** Criar as telas de Login e Registro que consomem a API.
* [ ] **Dashboard de Notas:** Desenvolver a visualização principal com gerenciamento de estado.
* [ ] **Sincronia Total:** Integrar o frontend com o backend usando os tipos da crate `DTOs`.

---

## 📡 Endpoints da API (V1)

| Categoria | Método | Rota | Protegido |
| --- | --- | --- | --- |
| **Auth** | `POST` | `/api/auth/register` | ❌ |
| **Auth** | `POST` | `/api/auth/login` | ❌ |
| **Notes** | `GET` | `/api/notes` | ✅ |
| **Notes** | `POST` | `/api/notes` | ✅ |
| **Notes** | `PUT` | `/api/notes/:id` | ✅ |
| **Notes** | `DELETE` | `/api/notes/:id` | ✅ |

---
