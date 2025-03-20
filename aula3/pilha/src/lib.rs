use super::Stack;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nova_pilha_esta_vazia() {
        let pilha: Stack<i32> = Stack::nova();
        assert!(pilha.esta_vazia());
    }

    #[test]
    fn empilhar_elemento() {
        let mut pilha = Stack::nova();
        pilha.empilhar(1);
        assert!(!pilha.esta_vazia());
    }

    #[test]
    fn desempilhar_elemento() {
        let mut pilha = Stack::nova();
        pilha.empilhar(1);
        let elemento = pilha.desempilhar();
        assert_eq!(elemento, Some(1));
        assert!(pilha.esta_vazia());
    }

    #[test]
    fn capacidade_maxima() {
        let mut pilha = Stack::<i32> { 
            elementos: Vec::new(), 
            capacidade_maxima: Some(2) 
        };
        
        pilha.empilhar(1);
        pilha.empilhar(2);
        
        // Testa se a capacidade máxima impede mais inserções
        let resultado = std::panic::catch_unwind(|| {
            pilha.empilhar(3);
        });
        
        assert!(resultado.is_err());
    }
}