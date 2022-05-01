use rkalloc::RKalloc;
use core::ptr::null_mut;
use core::marker::PhantomData;
use core::iter::{Iterator,ExactSizeIterator};

struct Node<T> {
    next: *mut Node<T>,
    element: Option<T>,     //为了pop_front方法能获取element
}

impl<T> Node<T> {
    fn new(element: T) -> Self {
        Node { next: null_mut(), element: Some(element)}
    }
}

/// 单链表
/// 
/// 支持的操作：
/// - new                   创建新链表
/// - is_empty              是否为空
/// - len                   长度
/// - front/front_mut       第一个元素
/// - contains              是否包含某个元素
/// - push_front            头插入
/// - pop_front             弹出头
/// - clear                 清空
/// - iter/iter_mut         迭代器
pub struct SList<'a,T> {
    head: *mut Node<T>,
    alloc: &'a dyn RKalloc,
    marker: PhantomData<*const Node<T>>,
    size: usize,
}

/// 不可变迭代器
#[derive(Clone, Copy)]
pub struct SListIter<'a, T:'a> {
    head: *const Node<T>,
    size: usize,
    marker: PhantomData<&'a Node<T>>,
}

/// 可变迭代器
/// 
/// 除了实现了Iter trait外，还支持：
/// - insert_after          在迭代器指向的元素的下一个元素处插入
/// - remove_after          删除迭代器指向的元素的下一个元素
#[derive(Clone, Copy)]
pub struct SListIterMut<'a, T:'a> {
    head: *mut Node<T>,
    size: usize,
    alloc: &'a dyn RKalloc,
    marker: PhantomData<&'a Node<T>>,
}

impl<'a,T> SList<'a,T> {
    /// 构造单链表
    pub fn new (alloc: &'a dyn RKalloc) -> Self {
        Self {head: null_mut(), alloc, marker:PhantomData, size: 0}
    }

    /// 链表是否为空
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.head.is_null()
    }

    /// 长度
    #[inline]
    pub fn len(&self) -> usize {
        self.size
    }

    /// 是否包含`x`
    #[inline]
    pub fn contains(&self, x: &T) -> bool
    where T: PartialEq<T>
        {
            self.iter().any(|e| e == x)
        }

    /// 链表首个元素的引用
    #[inline] #[must_use]
    pub fn front<'b>(&'b self) -> Option<&'b T> {
        unsafe {self.head.as_ref().map(|node| node.element.as_ref().unwrap())}
    }

    /// 链表首个元素的可变引用
    #[inline] #[must_use]
    pub fn front_mut<'b>(&'b mut self) -> Option<&'b mut T> {
        unsafe {self.head.as_mut().map(|node| node.element.as_mut().unwrap())}
    }

    /// 在头部插入新结点
    #[inline]
    pub fn push_front(&mut self, element: T) -> Result<(),&'static str>{
        self.push_front_node(unsafe{rkalloc::alloc_type(self.alloc,Node::new(element))})?;
        self.size += 1;
        Ok(())
    }

    /// 弹出头部的结点
    pub fn pop_front(&mut self) -> Option<T> {
        if self.head.is_null() {
            None
        }
        else {
            self.size -= 1;
            let ptr = self.head;
            unsafe{self.head = (*ptr).next;}
            let old_head = unsafe{ptr.replace(Node::<T>{next:null_mut(),element:None})};
            unsafe{rkalloc::dealloc_type(self.alloc, ptr);}
            old_head.element
        }
    }

    /// 清空链表
    pub fn clear(&mut self){
        unsafe {
            let mut ptr = self.head;
            while !ptr.is_null() {
                let next = (*ptr).next;
                rkalloc::dealloc_type(self.alloc, ptr);
                ptr = next;
            }
        }
        self.size = 0;
    }

    /// 不可变迭代器
    #[inline]
    pub fn iter<'b>(&'b self) -> SListIter<'b,T> {
        SListIter { head: self.head, size: self.size, marker: PhantomData }
    }

    /// 可变迭代器
    #[inline]
    pub fn iter_mut<'b>(&'b mut self) -> SListIterMut<'b,T> {
        SListIterMut { head: self.head, size: self.size, marker: PhantomData, alloc: self.alloc}
    }

    
}

impl<'a,T> SList<'a,T> {
    fn push_front_node(&mut self, mut node: *mut Node<T>) -> Result<(),&'static str>{
        if node.is_null() {return Err("fail to allocate memory");}
        unsafe{
            (*node).next = self.head;
            self.head = node;
        }
        Ok(())
    }
}

impl<'a,T> Drop for SList<'a,T> {
    fn drop(&mut self) {
        self.clear();
    }
}

impl<'a,T> Iterator for SListIter<'a,T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<&'a T> {
        let ret = self.head;
        if ret.is_null() {None}
        else {
            self.size -= 1;
            unsafe{
                self.head = (*self.head).next;
                (*ret).element.as_ref()
            }
        }
    }
}

impl<T> ExactSizeIterator for SListIter<'_,T> {
    fn len(&self) -> usize {
        self.size
    }
}

impl<'a,T> Iterator for SListIterMut<'a,T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<&'a mut T> {
        let ret = self.head;
        if ret.is_null() {None}
        else {
            self.size -= 1;
            unsafe{
                self.head = (*self.head).next;
                (*ret).element.as_mut()
            }
        }
    }
}

impl<T> ExactSizeIterator for SListIterMut<'_,T> {
    fn len(&self) -> usize {
        self.size
    }
}

impl<T> SListIterMut<'_,T> {
    /// 在迭代器指向的位置之后插入
    pub fn insert_after(&mut self, element: T) -> Result<(),&'static str>{
        if self.head.is_null() {
            return Err("already reached end");
        }
        unsafe {
            let node = rkalloc::alloc_type(self.alloc, Node::<T>::new(element));
            if node.is_null() {return Err("fail to allocate memory");}
            (*node).next=(*self.head).next;
            (*self.head).next = node;
        }
        self.size += 1;
        Ok(())
    }

    /// 在迭代器指向的位置之后删除
    pub fn remove_after(&mut self) -> Option<T> {
        assert!(!self.head.is_null());
        unsafe {
            if (*self.head).next.is_null() {
                None
            }
            else {
                self.size -= 1;
                let ptr = (*self.head).next;
                (*self.head).next = (*ptr).next;
                let old_head = ptr.replace(Node::<T>{next:null_mut(),element:None});
                rkalloc::dealloc_type(self.alloc, ptr);
                old_head.element
            }
        }
    }
}
