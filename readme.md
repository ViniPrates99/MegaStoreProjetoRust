# Sistema de Busca Otimizado para Catálogo de Produtos

Este projeto é uma implementação de um sistema de busca de alta performance para o catálogo de produtos da "MegaStore", desenvolvido como parte da disciplina Estratégia e Implementação de Estruturas de Dados. O objetivo principal é resolver os problemas de lentidão e imprecisão do sistema de busca existente, proporcionando uma experiência de compra rápida e eficiente para os clientes.

O sistema indexa um catálogo de produtos e oferece buscas quase instantâneas por vários critérios, como ID, nome, marca e categoria, garantindo escalabilidade para lidar com milhões de itens.

## Tecnologias Utilizadas

* **Linguagem de Programação:** Rust (versão 1.xx)
* **Gestor de Pacotes e Build System:** Cargo
* **Crates (Bibliotecas) Principais:**
    * `std::collections::HashMap`: Utilizada como a estrutura de dados central para a criação de índices de busca eficientes.
* **Ferramentas de Teste:** Framework de teste integrado do Rust (`cargo test`).

## Instruções de Execução

Para compilar e executar este projeto, certifique-se de que tem o Rust e o Cargo instalados.

2.  **Compile e Execute o Programa Principal:**
    O programa principal (`main.rs`) contém exemplos de como adicionar produtos e realizar buscas. 

## Instruções para Executar os Testes

O projeto inclui um conjunto de testes unitários para garantir o correto funcionamento do sistema de busca.

Uma mensagem de sucesso indicará que todos os testes passaram, confirmando a integridade do código.


## Arquitetura do Sistema

A arquitetura do sistema é centrada em duas `structs` principais:

* **`Produto`**: Representa um único item no catálogo, contendo campos como `id`, `nome`, `marca`, `categoria` e `preco`.
* **`Catalogo`**: Atua como o motor de busca. Internamente, contém múltiplos `HashMap`s que funcionam como "índices" para permitir buscas rápidas e eficientes.

## Algoritmos e Estruturas de Dados Utilizados

A parte principal deste sistema de busca é a **Tabela Hash**, implementada em Rust através do `std::collections::HashMap`. Esta escolha foi fundamental para alcançar o desempenho desejado.

Foram criados os seguintes índices:
1.  **`todos_os_produtos: HashMap<u32, Produto>`**: O nosso "arquivo mestre", que mapeia o ID de um produto para a sua `struct` completa. Permite acesso direto em tempo `O(1)`.
2.  **`indice_por_nome: HashMap<String, u32>`**: Um índice que mapeia o nome de um produto ao seu ID.
3.  **`indice_por_marca: HashMap<String, Vec<u32>>`**: Um índice invertido que mapeia uma marca a uma lista de IDs de todos os produtos pertencentes a essa marca.
4.  **`indice_por_categoria: HashMap<String, Vec<u32>>`**: Similar ao índice de marcas, mas para categorias.

Esta abordagem de múltiplos índices permite que as buscas por diferentes critérios sejam extremamente rápidas, pois evitam a necessidade de percorrer linearmente o catálogo.

## Considerações sobre Desempenho e Escalabilidade

* **Desempenho:** Graças ao uso de `HashMap`, as operações de busca (por ID, nome, marca) têm uma complexidade de tempo média de `O(1)` (tempo constante), o que significa que o tempo de resposta permanece rápido mesmo com milhões de produtos no catálogo.
* **Escalabilidade:** A arquitetura é altamente escalável. Adicionar mais produtos aumenta o consumo de memória dos `HashMap`s, mas o tempo de busca permanece constante. O sistema é capaz de lidar com o crescimento contínuo do catálogo da "MegaStore".