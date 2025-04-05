#[allow(dead_code)]
struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

#[allow(dead_code)]
struct BST {
    root: Option<Box<Node>>,
}

#[allow(dead_code)]
impl BST {
    // esse comando ele cria uma nova árvore vazia
    fn new() -> Self {
        BST { root: None }
    }
    
    // vai ver se a árvore está vazia
    fn is_empty(&self) -> bool {
        self.root.is_none()
    }
    
    // aqui 👇 insere um valor na árvore
    fn insert(&mut self, value: i32) {
        self.root = Self::insert_node(self.root.take(), value);
    }
    
    // Função recursiva para inserir um valor na árvore
    fn insert_node(node: Option<Box<Node>>, value: i32) -> Option<Box<Node>> {
        match node {
            None => Some(Box::new(Node {
                value,
                left: None,
                right: None,
            })),
            Some(mut node) => {
                if value < node.value {
                    node.left = Self::insert_node(node.left.take(), value);
                } else if value > node.value {
                    node.right = Self::insert_node(node.right.take(), value);
                }
                Some(node)
            }
        }
    }
    
    // Buscar um valor na árvore
    fn search(&self, value: i32) -> bool {
        Self::search_node(&self.root, value)
    }
    
    // Função recursiva para buscar um valor na árvore
    fn search_node(node: &Option<Box<Node>>, value: i32) -> bool {
        match node {
            None => false,
            Some(node) => {
                if value == node.value {
                    true
                } else if value < node.value {
                    Self::search_node(&node.left, value)
                } else {
                    Self::search_node(&node.right, value)
                }
            }
        }
    }
}
