/*
	heap
	This question requires you to implement a binary heap function
*/


use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,               // items[0] 占位，从 1 开始放元素
    comparator: fn(&T, &T) -> bool, // 返回 true 表示左边优先级更高
}

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()], // 0 号位占位
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        // 插到末尾，然后“上滤”直到父节点的优先级不低于当前节点
        self.count += 1;
        self.items.push(value);

        let mut idx = self.count;
        while idx > 1 {
            let parent_idx = self.parent_idx(idx);
            // 若当前比父亲“更优”（对最小堆是更小；对最大堆是更大），则交换
            if (self.comparator)(&self.items[idx], &self.items[parent_idx]) {
                self.items.swap(idx, parent_idx);
                idx = parent_idx;
            } else {
                break;
            }
        }
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        // 选出“更优”的子：如果只有左子就左子；若有两个，用比较器挑一个
        let left = self.left_child_idx(idx);
        let right = self.right_child_idx(idx);
        if right > self.count {
            left
        } else {
            if (self.comparator)(&self.items[left], &self.items[right]) {
                left
            } else {
                right
            }
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        // 弹出堆顶：
        // 1) 空则 None
        // 2) 否则取 items[1] 为结果，把最后一个搬到根，count--, pop()
        // 3) 从根开始“下滤”，与更优子比较，不满足堆序就交换，直到满足或无子
        if self.is_empty() {
            return None;
        }
        let top = std::mem::replace(&mut self.items[1], T::default()); // 占位
        let last = self.items.pop().unwrap(); // 取出最后一个
        self.count -= 1;

        if !self.is_empty() {
            // 把最后一个元素放到根，然后下滤
            self.items[1] = last;

            let mut idx = 1;
            while self.children_present(idx) {
                let best_child = self.smallest_child_idx(idx);
                if (self.comparator)(&self.items[best_child], &self.items[idx]) {
                    self.items.swap(idx, best_child);
                    idx = best_child;
                } else {
                    break;
                }
            }
        }

        Some(top)
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}
