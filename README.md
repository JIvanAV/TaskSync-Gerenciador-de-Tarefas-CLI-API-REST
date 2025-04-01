# TaskSync-Gerenciador-de-Tarefas-CLI-API-REST
TaskSync é um sistema de gerenciamento de tarefas desenvolvido em Rust, com suporte a API RESTful, autenticação JWT, WebSocket para comunicação em tempo real e interface de linha de comando (CLI). O objetivo do projeto é fornecer uma plataforma eficiente para gerenciar tarefas, com integração entre front-end e back-end.

Funcionalidades
  API RESTful: Permite criar, listar, atualizar e excluir tarefas.
  Autenticação JWT: Garante que somente usuários autenticados possam acessar ou modificar as tarefas.
  WebSocket: Comunicação em tempo real para atualizações instantâneas sobre o estado das tarefas.
  CLI: Interface de linha de comando para interagir com o sistema de tarefas diretamente no terminal.

Tecnologias
  Rust: Linguagem principal do projeto.
  Actix-Web: Framework web assíncrono para criar a API.
  SQLx: Acesso assíncrono ao banco de dados SQLite.
  jsonwebtoken: Manipulação de tokens JWT.
  WebSocket: Comunicação em tempo real com o front-end.
  Clap: Criação de comandos para a interface de linha de comando.
  Chrono: Manipulação de datas e horários.
  Dotenv: Carregamento de variáveis de ambiente a partir de um arquivo .env.
  Serde: Serialização e desserialização de dados JSON.

Requisitos
  Antes de executar o projeto, certifique-se de ter o seguinte instalado:
  Rust (versão 1.60 ou superior)
  Cargo (gerenciador de pacotes do Rust)
  SQLite (Banco de dados)
  Docker (caso deseje usar um contêiner)

Instalação
Clone o repositório para sua máquina local:
  git clone https://github.com/seu-usuario/tasksync.git
  cd tasksync

Instale as dependências do projeto:
  cargo build
  Crie o arquivo .env na raiz do projeto e defina a chave secreta JWT:
    JWT_SECRET=seu_secreto_aqui

Configure o banco de dados SQLite (caso queira usar Docker, crie um contêiner):
docker run -d --name tasksync-db -v tasksync-db:/db -e SQLITE_DATABASE=tasksync.sqlite3 sqlite:latest

Execute o servidor:
  cargo run
  O servidor estará disponível na URL http://localhost:8080.

Endpoints da API
1. Criar Tarefa
  POST /tasks
    Cria uma nova tarefa.

Exemplo de corpo da requisição:
{
  "title": "Tarefa 1",
  "description": "Descrição da tarefa 1"
}

2. Listar Tarefas
  GET /tasks
  Retorna todas as tarefas.

3. Atualizar Tarefa
  PUT /tasks/{id}
  Atualiza uma tarefa existente.

Exemplo de corpo da requisição:
{
  "title": "Tarefa Atualizada",
  "description": "Descrição atualizada",
  "completed": true
}

4. Excluir Tarefa
  DELETE /tasks/{id}
  Exclui uma tarefa existente.

Interface de Linha de Comando (CLI)
Você pode interagir com o sistema de tarefas usando a interface de linha de comando.

Comandos disponíveis
Criar uma nova tarefa
  cargo run -- create --title "Nova Tarefa" --description "Descrição da nova tarefa"
Listar todas as tarefas
  cargo run -- list
Atualizar uma tarefa existente
  cargo run -- update --id 1 --title "Tarefa Atualizada" --description "Nova descrição" --completed true
  
WebSocket
O sistema também suporta WebSocket para comunicação em tempo real. Para começar, conecte-se ao WebSocket usando a URL ws://localhost:8080/ws e comece a receber atualizações sobre as tarefas.





CONTATO: joseivanabrantes@gmail.com
