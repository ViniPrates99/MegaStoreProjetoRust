# Sistema de Busca Otimizado para Catálogo de Produtos

Este projeto é uma implementação de um sistema de busca de alta performance para o catálogo de produtos da "MegaStore", desenvolvido como parte da disciplina de Estratégia e Implementação de Estruturas de Dados. O objetivo principal é resolver os problemas de lentidão e imprecisão do sistema de busca existente, proporcionando uma experiência de compra rápida e eficiente para os clientes.

O sistema indexa um catálogo de produtos e oferece buscas quase instantâneas por múltiplos critérios, como ID, nome, marca e categoria, garantindo escalabilidade para lidar com milhões de itens.

## Tecnologias Utilizadas

* **Linguagem de Programação:** Rust (versão 1.xx)
* **Gestor de Pacotes e Build System:** Cargo
* **Crates (Bibliotecas) Principais:**
    * `std::collections::HashMap`: Utilizada como a estrutura de dados central para a criação de índices de busca eficientes.
* **Ferramentas de Teste:** Framework de testes unitários integrado do Rust (`cargo test`).

## Instruções de Execução

Para compilar e executar este projeto, certifique-se de que tem o Rust e o Cargo instalados.

  **Compile e Execute o Programa Principal:**
    O programa principal (`main.rs`) contém exemplos de como adicionar produtos e realizar buscas. Para executá-lo, use o comando:
    ```sh
    cargo run
    ```

## Instruções para Executar os Testes

O projeto inclui um conjunto de testes unitários para garantir a fiabilidade e o correto funcionamento do sistema de busca.

Para executar todos os testes, utilize o seguinte comando no terminal, a partir da raiz do projeto:
```sh
cargo test
```
Uma mensagem de sucesso indicará que todos os testes passaram, confirmando a integridade do código.

## Exemplos de Uso

O sistema de busca pode ser utilizado através dos métodos implementados na `struct Catalogo`. O ficheiro `src/main.rs` contém exemplos práticos, mas aqui estão alguns destaques:

```rust
// Criar um novo catálogo
let mut catalogo = Catalogo::new();

// Adicionar um novo produto
let produto1 = Produto { id: 101, nome: "Smartphone XPTO".to_string(), ... };
catalogo.adicionar_produto(produto1);

// Buscar um produto pelo seu ID
let resultado_id = catalogo.buscar_por_id(101);

// Buscar um produto pelo seu nome
let resultado_nome = catalogo.buscar_por_nome("Smartphone XPTO");

// Buscar todos os produtos de uma marca
let resultados_marca = catalogo.buscar_por_marca("Samsung");
```

## Arquitetura do Sistema

A arquitetura do sistema é centrada em duas `structs` principais:

* **`Produto`**: Representa um único item no catálogo, contendo campos como `id`, `nome`, `marca`, `categoria` e `preco`.
* **`Catalogo`**: Atua como o motor de busca. Internamente, contém múltiplos `HashMap`s que funcionam como "índices" para permitir buscas rápidas e eficientes.

## Algoritmos e Estruturas de Dados Utilizados

A espinha dorsal deste sistema de busca é a **Tabela Hash**, implementada em Rust através do `std::collections::HashMap`. Esta escolha foi fundamental para alcançar o desempenho desejado.

Foram criados os seguintes índices:
1.  **`todos_os_produtos: HashMap<u32, Produto>`**: O nosso "arquivo mestre", que mapeia o ID de um produto para a sua `struct` completa. Permite acesso direto em tempo `O(1)`.
2.  **`indice_por_nome: HashMap<String, u32>`**: Um índice que mapeia o nome de um produto ao seu ID.
3.  **`indice_por_marca: HashMap<String, Vec<u32>>`**: Um índice invertido que mapeia uma marca a uma lista de IDs de todos os produtos pertencentes a essa marca.
4.  **`indice_por_categoria: HashMap<String, Vec<u32>>`**: Similar ao índice de marcas, mas para categorias.

Esta abordagem de múltiplos índices permite que as buscas por diferentes critérios sejam extremamente rápidas, pois evitam a necessidade de percorrer linearmente o catálogo.

## Considerações sobre Desempenho e Escalabilidade

* **Desempenho:** Graças ao uso de `HashMap`, as operações de busca (por ID, nome, marca) têm uma complexidade de tempo média de `O(1)` (tempo constante), o que significa que o tempo de resposta permanece rápido mesmo com milhões de produtos no catálogo.
* **Escalabilidade:** A arquitetura é altamente escalável. Adicionar mais produtos aumenta o consumo de memória dos `HashMap`s, mas o tempo de busca permanece constante. O sistema é capaz de lidar com o crescimento contínuo do catálogo da "MegaStore".
