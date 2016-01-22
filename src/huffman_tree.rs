extern crate binary_heap;
use self::binary_heap::binary_heap::BinaryHeap;
use std::ops::Add;
use std::hash::Hash;


pub trait CodecElement: Copy{
    type WeightType: Copy + Ord + Add<Self::WeightType, Output = Self::WeightType>;
    type ElementType: Copy + Hash + Eq;
    fn get_weight(&self) -> Self::WeightType;
    fn get_element(&self) -> &Self::ElementType;
}

enum HuffmanTreeNode<T: CodecElement>{
    Internal(HuffmanTreeInternal<T>),
    Leaf(T)
}

impl<T:CodecElement> HuffmanTreeNode<T>{
    fn get_weight(&self)-> T::WeightType{
        match *self{
            HuffmanTreeNode::Leaf(ref v) => { v.get_weight() },
            HuffmanTreeNode::Internal(ref v) => { v.weight }
        }
    }
}

use std::cmp::Ord;
use std::cmp::Ordering;
impl<T:CodecElement> PartialEq for HuffmanTreeNode<T>{
    fn eq(&self, other: &Self) -> bool{
        self.get_weight() == other.get_weight()
    }
}

impl<T:CodecElement> Eq for HuffmanTreeNode<T>{}

impl<T:CodecElement> PartialOrd for HuffmanTreeNode<T>{
    fn partial_cmp(&self, other:&Self) -> Option<Ordering>{
        self.get_weight().partial_cmp(&other.get_weight())
    }
}
impl<T:CodecElement> Ord for HuffmanTreeNode<T>{
    fn cmp(&self, other:&Self) -> Ordering{
        self.get_weight().cmp(&other.get_weight())
    }
}

struct HuffmanTreeInternal<T: CodecElement>{
    left: Box<HuffmanTreeNode<T>>,
    right: Box<HuffmanTreeNode<T>>,
    weight: T::WeightType
}

use std::collections::HashMap;
pub struct CodecTable<T: CodecElement>{
    root: HuffmanTreeNode<T>,
    trans_table: HashMap<T::ElementType, BitString>
}

use bit_string::BitString;
use std::slice::Iter;

impl<T: CodecElement> CodecTable<T>{
    fn gen_table(root: &HuffmanTreeNode<T>, trans_table: &mut  HashMap<T:: ElementType, BitString>, current_path: &mut BitString){
        match *root{
            HuffmanTreeNode::Leaf(ref v) => {
                trans_table.insert(*v.get_element(), current_path.clone());
            },
            HuffmanTreeNode::Internal(ref node) =>{
                current_path.push_bit(false);
                CodecTable::gen_table(&*node.left, trans_table, current_path);
                current_path.pop_bit();
                current_path.push_bit(true);
                CodecTable::gen_table(&*node.right, trans_table, current_path);
                current_path.pop_bit();
            }            
        }
    }
    pub fn new(elements: &Vec<T>)-> Self{
        let mut heap =  BinaryHeap::<HuffmanTreeNode<T>>::new();
        for i in elements{
            heap.push(HuffmanTreeNode::Leaf(*i));
        }
        
        //Build huffman tree
        while heap.len() > 1{
            let a = heap.pop().unwrap();
            let b = heap.pop().unwrap();
            let w:T::WeightType = (a.get_weight() + b.get_weight()) as T::WeightType;

            heap.push(HuffmanTreeNode::Internal(HuffmanTreeInternal{
                left: Box::new(a),
                right: Box::new(b),
                weight: w
            }));
        }

        //Generate translation table
        let mut trans_table:HashMap<T::ElementType, BitString> = HashMap::new();
        let root = heap.pop().unwrap();
        let mut path = BitString::new();
        Self::gen_table(&root, &mut trans_table, &mut path);
        
        CodecTable{
            root: root,
            trans_table: trans_table
        }
    }

    pub fn encode(&self, i: &T::ElementType) -> Option<BitString>{
        self.trans_table.get(i).cloned()
    }

    pub fn decode(&self, in_str:&BitString) -> Vec<T::ElementType>{
        let mut res = Vec::<T::ElementType>::new();

        let mut a = &self.root;
        for i in 0 .. in_str.len(){
            let next = match *a{
                HuffmanTreeNode::Leaf(_) => {None}
                HuffmanTreeNode::Internal(ref node) => {
                    if in_str.get_bit(i) {
                        Some(&node.right)
                    }
                    else
                    {
                        Some(&node.left)
                    }
                }
            }.map(|x|{
                match **x{
                    HuffmanTreeNode::Leaf(ref e) => {res.push(e.get_element().clone());&self.root}
                    HuffmanTreeNode::Internal(_) => {&**x}
                }
            });
            match next{
                None => {return res;}
                Some(t) => {a = t;}
            }
        }

        res
    }
    
    pub fn encode_iter(&self, iter: Iter<T::ElementType>) -> BitString{
        iter.fold(BitString::new(), |acc, x| acc + self.encode(&x).unwrap())
    }

}

#[cfg(test)]
mod test{
    #[derive(Copy)]
    struct CodecUnit{
        c: char,
        w: u32
    }
    use super::{CodecTable, CodecElement, HuffmanTreeNode};

    impl Clone for CodecUnit{
        fn clone(&self) -> Self{
            CodecUnit{c:self.c, w:self.w}
        }
    }

    impl CodecElement for CodecUnit{
        type WeightType = u32;
        type ElementType = char;
        fn get_weight(&self) -> u32 { self.w }
        fn get_element(&self) -> &char { &self.c }
    }

    fn print_tree(t: &HuffmanTreeNode<CodecUnit>) {
        match *t{
            HuffmanTreeNode::Leaf(ref l) => {print!("{} ", l.w);}
            HuffmanTreeNode::Internal(ref node) =>{
                print!("({} ", node.weight);
                print_tree(&*node.left);
                print_tree(&*node.right);
                print!(")");
            }
        }
    }
    
    #[test]
    fn test_new(){
        let a = CodecTable::<CodecUnit>::new(&vec![
            CodecUnit{c: 'a', w: 1},
            CodecUnit{c: 'b', w: 2},
            CodecUnit{c: 'c', w: 3},
            CodecUnit{c: 'd', w: 4},
            CodecUnit{c: 'e', w: 5},
            ]);
        print_tree(&a.root);
        println!("");

        for (k, v) in &a.trans_table{
            println!("{} {:?}", k, v);
        }
        println!("{:?}", a.encode(&'c'));
    }

    #[test]
    fn encode_seq(){
        let a = vec!['a','b','c','d','e'];
        let ct = CodecTable::<CodecUnit>::new(&vec![
            CodecUnit{c: 'a', w: 1},
            CodecUnit{c: 'b', w: 2},
            CodecUnit{c: 'c', w: 3},
            CodecUnit{c: 'd', w: 4},
            CodecUnit{c: 'e', w: 5},
            ]);

        let cr = ct.encode_iter(a.iter());
        println!("{:?}",cr);
        println!("{:?}", ct.decode(&cr));
    }
}
