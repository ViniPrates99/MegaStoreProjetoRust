use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Produto {
    id: u32,
    nome: String,
    marca: String,
    categoria: String,
    preco: f64,
}

#[derive(Debug)]
pub struct Catalogo {
    todos_os_produtos: HashMap<u32, Produto>,
    indice_por_nome: HashMap<String, u32>,
    indice_por_marca: HashMap<String, Vec<u32>>,
    indice_por_categoria: HashMap<String, Vec<u32>>,
}

impl Catalogo {
    pub fn new() -> Self {
        Catalogo {
            todos_os_produtos: HashMap::new(),
            indice_por_nome: HashMap::new(),
            indice_por_marca: HashMap::new(),
            indice_por_categoria: HashMap::new(),
        }
    }

    pub fn adicionar_produto(&mut self, produto: Produto) {
        let produto_id = produto.id;
        self.todos_os_produtos.insert(produto_id, produto.clone());
        self.indice_por_nome.insert(produto.nome.clone(), produto_id);
        self.indice_por_marca.entry(produto.marca.clone()).or_insert_with(Vec::new).push(produto_id);
        self.indice_por_categoria.entry(produto.categoria.clone()).or_insert_with(Vec::new).push(produto_id);
    }

    pub fn buscar_por_id(&self, id: u32) -> Option<&Produto> {
        self.todos_os_produtos.get(&id)
    }
    
    pub fn buscar_por_nome(&self, nome: &str) -> Option<&Produto> {
        match self.indice_por_nome.get(nome) {
            Some(produto_id) => self.todos_os_produtos.get(produto_id),
            None => None,
        }
    }

    pub fn buscar_por_marca(&self, marca: &str) -> Vec<&Produto> {
        let mut produtos_encontrados = Vec::new();
        if let Some(lista_de_ids) = self.indice_por_marca.get(marca) {
            for id in lista_de_ids {
                if let Some(produto) = self.buscar_por_id(*id) {
                    produtos_encontrados.push(produto);
                }
            }
        }
        produtos_encontrados
    }
    
    pub fn buscar_por_categoria(&self, categoria: &str) -> Vec<&Produto> {
        let mut produtos_encontrados = Vec::new();
        if let Some(lista_de_ids) = self.indice_por_categoria.get(categoria) {
            for id in lista_de_ids {
                if let Some(produto) = self.buscar_por_id(*id) {
                    produtos_encontrados.push(produto);
                }
            }
        }
        produtos_encontrados
    }
}

fn main() {
    println!("Sistema de Busca da MegaStore.");
    println!("Execute 'cargo run' para ver exemplos de uso ou 'cargo test' para verificar a integridade do código.");
}


// --- MÓDULO DE TESTES COMPLETO ---
#[cfg(test)]
mod tests {
    use super::*;

    // --- Testes que já tínhamos ---
    #[test]
    fn test_adicionar_e_buscar_produto_com_sucesso() {
        let mut catalogo = Catalogo::new();
        let produto = Produto { id: 1, nome: String::from("Teste"), marca: String::from("Marca Teste"), categoria: String::from("Categoria Teste"), preco: 10.0 };
        catalogo.adicionar_produto(produto.clone());
        let resultado = catalogo.buscar_por_id(1);
        assert!(resultado.is_some());
        assert_eq!(resultado.unwrap(), &produto);
    }

    #[test]
    fn test_buscar_id_inexistente_retorna_none() {
        let catalogo = Catalogo::new();
        let resultado = catalogo.buscar_por_id(999);
        assert!(resultado.is_none());
    }

    // --- NOSSOS NOVOS TESTES ---

    #[test]
    fn test_buscar_por_nome_funciona() {
        let mut catalogo = Catalogo::new();
        let produto = Produto { id: 101, nome: String::from("Smartphone"), marca: String::from("Samsung"), categoria: String::from("Eletrônicos"), preco: 10.0 };
        catalogo.adicionar_produto(produto.clone());

        // Testa o caso de sucesso
        let resultado = catalogo.buscar_por_nome("Smartphone");
        assert_eq!(resultado.unwrap().id, 101);

        // Testa o caso de falha
        let resultado_falha = catalogo.buscar_por_nome("Produto Inexistente");
        assert!(resultado_falha.is_none());
    }

    #[test]
    fn test_buscar_por_marca_retorna_lista_correta() {
        let mut catalogo = Catalogo::new();
        catalogo.adicionar_produto(Produto { id: 101, nome: String::from("Galaxy S25"), marca: String::from("Samsung"), categoria: String::from("Eletrônicos"), preco: 10.0 });
        catalogo.adicionar_produto(Produto { id: 102, nome: String::from("TV QLED"), marca: String::from("Samsung"), categoria: String::from("Eletrônicos"), preco: 20.0 });
        catalogo.adicionar_produto(Produto { id: 201, nome: String::from("iPhone 17"), marca: String::from("Apple"), categoria: String::from("Eletrônicos"), preco: 30.0 });

        // Testa uma marca com múltiplos produtos
        let resultados_samsung = catalogo.buscar_por_marca("Samsung");
        assert_eq!(resultados_samsung.len(), 2);

        // Testa uma marca com um produto
        let resultados_apple = catalogo.buscar_por_marca("Apple");
        assert_eq!(resultados_apple.len(), 1);
        
        // Testa uma marca inexistente
        let resultados_lg = catalogo.buscar_por_marca("LG");
        assert_eq!(resultados_lg.len(), 0); // Esperamos um vetor vazio
    }
    
    #[test]
    fn test_buscar_por_categoria_retorna_lista_correta() {
        let mut catalogo = Catalogo::new();
        catalogo.adicionar_produto(Produto { id: 101, nome: String::from("Galaxy S25"), marca: String::from("Samsung"), categoria: String::from("Eletrônicos"), preco: 10.0 });
        catalogo.adicionar_produto(Produto { id: 301, nome: String::from("Camiseta"), marca: String::from("Loja"), categoria: String::from("Vestuário"), preco: 20.0 });

        // Testa a categoria Eletrônicos
        let resultados_eletronicos = catalogo.buscar_por_categoria("Eletrônicos");
        assert_eq!(resultados_eletronicos.len(), 1);
        assert_eq!(resultados_eletronicos[0].id, 101);

        // Testa uma categoria inexistente
        let resultados_moveis = catalogo.buscar_por_categoria("Móveis");
        assert!(resultados_moveis.is_empty());
    }
}