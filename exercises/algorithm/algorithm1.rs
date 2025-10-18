/*
	single linked list merge
	This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/


use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;
use std::vec::*;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,  // NonNull<Node<T>> 是一个非空原始指针（内部等价于 *mut Node<T> 但带了“绝不为 null”的类型承诺）。 ：NonNull<T> 不携带生命周期、没有所有权信息，不会自动释放。它只是一个裸地址——需要你自己保证有效性（这就是为什么链表实现里会有 unsafe）。
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            next: None,
        }
    }
}
#[derive(Debug)]
struct LinkedList<T> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }

    pub fn add(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        // Box::into_raw(node)：把 Box 转成裸指针（*mut T）。注意：调用这个函数后，Box 不再拥有这个堆内存，所以不会自动释放它，必须手动释放以避免内存泄漏。
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) }); // NonNull::new_unchecked(...)：把裸指针包成 NonNull（承诺“非空”）。这里用 unchecked 的理由是：Box::into_raw 永不返回空指针，所以“非空”成立。
        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
        }
        self.end = node_ptr;
        self.length += 1;
    }

    pub fn get(&mut self, index: i32) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }

    fn get_ith_node(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }
	pub fn merge(list_a:LinkedList<T>,list_b:LinkedList<T>) -> Self
    where
        T: Clone + PartialOrd,
	{
		//TODO
        let mut a_node = list_a.start;
        let mut b_node = list_b.start;
        let mut merged_list = LinkedList::<T>::new();
        while a_node.is_some() && b_node.is_some() {
            let a_val = unsafe { &(*a_node.unwrap().as_ptr()).val };  //去掉&，由于是克隆性质，能编译，但每次访问都会复制值，造成性能损失
            //换种写法
            // let a_val = unsafe{
            //     let a_node_ref = a_node.unwrap().as_ref();
            //     &a_node_ref.val
            // };
            // let a_val: &T = unsafe { &a_node.unwrap().as_ref().val };
            let b_val = unsafe { &(*b_node.unwrap().as_ptr()).val };
            if a_val <= b_val {
                merged_list.add(a_val.clone());
                a_node = unsafe { (*a_node.unwrap().as_ptr()).next };
            } else {
                merged_list.add(b_val.clone());
                b_node = unsafe { (*b_node.unwrap().as_ptr()).next };
            }
        }
        while a_node.is_some() {
            let a_val = unsafe { &(*a_node.unwrap().as_ptr()).val };
            merged_list.add(a_val.clone());
            a_node = unsafe { (*a_node.unwrap().as_ptr()).next };
        }
        while b_node.is_some() {
            let b_val = unsafe { &(*b_node.unwrap().as_ptr()).val };
            merged_list.add(b_val.clone());
            b_node = unsafe { (*b_node.unwrap().as_ptr()).next };
        }
        merged_list

	}
}

impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.start {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.next {
            Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.val),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        println!("Linked List is {}", list);
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        list_str.add("C".to_string());
        println!("Linked List is {}", list_str);
        assert_eq!(3, list_str.length);
    }

    #[test]
    fn test_merge_linked_list_1() {
		let mut list_a = LinkedList::<i32>::new();
		let mut list_b = LinkedList::<i32>::new();
		let vec_a = vec![1,3,5,7];
		let vec_b = vec![2,4,6,8];
		let target_vec = vec![1,2,3,4,5,6,7,8];
		
		for i in 0..vec_a.len(){
			list_a.add(vec_a[i]);
		}
		for i in 0..vec_b.len(){
			list_b.add(vec_b[i]);
		}
		println!("list a {} list b {}", list_a,list_b);
		let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
		println!("merged List is {}", list_c);
		for i in 0..target_vec.len(){
			assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
		}
	}
	#[test]
	fn test_merge_linked_list_2() {
		let mut list_a = LinkedList::<i32>::new();
		let mut list_b = LinkedList::<i32>::new();
		let vec_a = vec![11,33,44,88,89,90,100];
		let vec_b = vec![1,22,30,45];
		let target_vec = vec![1,11,22,30,33,44,45,88,89,90,100];

		for i in 0..vec_a.len(){
			list_a.add(vec_a[i]);
		}
		for i in 0..vec_b.len(){
			list_b.add(vec_b[i]);
		}
		println!("list a {} list b {}", list_a,list_b);
		let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
		println!("merged List is {}", list_c);
		for i in 0..target_vec.len(){
			assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
		}
	}
}