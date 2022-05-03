use rkalloc::RKalloc;
use core::ops::{Deref, DerefMut};
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
    fn null() -> Self {
        Node { next: null_mut(), element: None}
    }
}

/// 单尾队列
/// 
/// 支持的操作：
/// - new                   创建新链表
/// - is_empty              是否为空
/// - len                   长度
/// - front/front_mut       第一个元素
/// - back/back_mut         最后一个元素
/// - contains              是否包含某个元素
/// - push_front            头插入
/// - pop_front             弹出头
/// - push_back             尾插入
/// - clear                 清空
/// - iter/iter_mut         迭代器
/// - head/head_mut         头结点
/// - tail/tail_mut         尾结点
/// - insert_after          指定位置之后插入
/// - remove_after          删除指定位置之后的元素
pub struct STailq<'a,T> {
    head: *mut Node<T>,
    tail: *mut Node<T>,
    alloc: &'a dyn RKalloc,
    marker: PhantomData<*const Node<T>>,
    size: usize,
}

/// 不可变迭代器
pub struct STailqIter<'a, T:'a> {
    head: *const Node<T>,
    size: usize,
    marker: PhantomData<&'a Node<T>>,
}

/// 可变迭代器
pub struct STailqIterMut<'a, T:'a> {
    head: *mut Node<T>,
    size: usize,
    marker: PhantomData<&'a Node<T>>,
}

/// 位置
pub struct STailqPos<T> {
    pos: *const Node<T>
}

pub struct STailqPosMut<T> {
    pos: *mut Node<T>
}

impl<T> Clone for STailqPos<T> {
    fn clone(&self) -> Self {
        Self {pos: self.pos}
    }
}

impl<T> Copy for STailqPos<T> {

}

impl<T> Clone for STailqPosMut<T> {
    fn clone(&self) -> Self {
        Self {pos: self.pos}
    }
}

impl<T> Copy for STailqPosMut<T> {

}

impl<'a,T> STailq<'a,T> {
    /// 构造单链表
    pub fn new (alloc: &'a dyn RKalloc) -> Self {
        Self {head: null_mut(), tail: null_mut(), alloc, marker:PhantomData, size: 0}
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

    /// 链表末个元素的引用
    #[inline] #[must_use]
    pub fn back<'b>(&'b self) -> Option<&'b T> {
        unsafe {self.tail.as_ref().map(|node| node.element.as_ref().unwrap())}
    }

    /// 链表末个元素的可变引用
    #[inline] #[must_use]
    pub fn back_mut<'b>(&'b mut self) -> Option<&'b mut T> {
        unsafe {self.tail.as_mut().map(|node| node.element.as_mut().unwrap())}
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
            if self.head.is_null() {
                debug_assert_eq!(ptr,self.tail);
                self.tail = null_mut();
            }
            let old_head = unsafe{ptr.replace(Node::null())};
            unsafe{rkalloc::dealloc_type(self.alloc, ptr);}
            old_head.element
        }
    }

    /// 在尾部插入新结点
    #[inline]
    pub fn push_back(&mut self, element: T) -> Result<(),&'static str>{
        self.push_back_node(unsafe{rkalloc::alloc_type(self.alloc,Node::new(element))})?;
        self.size += 1;
        Ok(())
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
        self.tail = null_mut();
        self.size = 0;
    }

    /// 不可变迭代器
    #[inline]
    pub fn iter<'b>(&'b self) -> STailqIter<'b,T> {
        STailqIter { head: self.head, size: self.size, marker: PhantomData }
    }

    /// 可变迭代器
    #[inline]
    pub fn iter_mut<'b>(&'b mut self) -> STailqIterMut<'b,T> {
        STailqIterMut { head: self.head, size: self.size, marker: PhantomData}
    }

    /// 头结点
    /// 
    /// 与`iter`不同，`head`产生的位置不会被视为self的引用
    #[inline]
    pub fn head(&self) -> STailqPos<T> {
        STailqPos { pos: self.head }
    }

    /// 头结点
    #[inline]
    pub fn head_mut(&mut self) -> STailqPosMut<T> {
        STailqPosMut { pos: self.head }
    }

    /// 尾
    #[inline]
    pub fn tail(&self) -> STailqPos<T> {
        STailqPos { pos: self.tail }
    }

    /// 尾结点
    #[inline]
    pub fn tail_mut(&mut self) -> STailqPosMut<T> {
        STailqPosMut { pos: self.tail }
    }

    /// 在迭代器指向的位置之后插入
    /// 
    /// # 安全性
    /// 
    /// `pos`必须和`self`属于同一个链表
    pub unsafe fn insert_after(&mut self, pos: STailqPosMut<T>, element: T) -> Result<(),&'static str>{
        if pos.pos.is_null() {
            return Err("invalid position");
        }
        let node = rkalloc::alloc_type(self.alloc, Node::new(element));
        if node.is_null() {return Err("fail to allocate memory");}
        (*node).next=(*pos.pos).next;
        if (*node).next.is_null() {self.tail = node;}
        (*(pos.pos)).next = node;
        self.size += 1;
        Ok(())
    }

    /// 在迭代器指向的位置之后删除
    /// 
    /// # 安全性
    /// 
    /// `pos`必须和`self`属于同一个链表
    pub unsafe fn remove_after(&mut self, pos: STailqPosMut<T>) -> Option<T> {
        assert!(!pos.pos.is_null());
        if (*pos.pos).next.is_null() {
            None
        }
        else {
            self.size -= 1;
            let ptr = (*pos.pos).next;
            if ptr.is_null() {self.tail = pos.pos;}
            (*(pos.pos)).next = (*ptr).next;
            let old_head = ptr.replace(Node::null());
            rkalloc::dealloc_type(self.alloc, ptr);
            old_head.element
        }
    }
}

impl<'a,T> STailq<'a,T> {
    fn push_front_node(&mut self, node: *mut Node<T>) -> Result<(),&'static str>{
        if node.is_null() {return Err("fail to allocate memory");}
        unsafe{
            if self.head.is_null() {
                debug_assert!(self.tail.is_null());
                self.tail = node;
            }
            (*node).next = self.head;
            self.head = node;
        }
        Ok(())
    }
    
    fn push_back_node(&mut self, node: *mut Node<T>) -> Result<(),&'static str>{
        if node.is_null() {return Err("fail to allocate memory");}
        unsafe{
            if self.head.is_null() {
                debug_assert!(self.tail.is_null());
                self.tail = node;
                self.head = node;
            }
            else {
                debug_assert!(!self.tail.is_null());
                (*self.tail).next = node;
                self.tail = node;
            }
        }
        Ok(())
    }
}

impl<'a,T> Drop for STailq<'a,T> {
    fn drop(&mut self) {
        self.clear();
    }
}

impl<'a,T> Iterator for STailqIter<'a,T> {
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

impl<T> ExactSizeIterator for STailqIter<'_,T> {
    fn len(&self) -> usize {
        self.size
    }
}

impl<'a,T> Iterator for STailqIterMut<'a,T> {
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

impl<T> ExactSizeIterator for STailqIterMut<'_,T> {
    fn len(&self) -> usize {
        self.size
    }
}

impl<T> STailqIter<'_,T> {
    /// 转换为`SListPos`
    pub fn as_pos(&self) -> STailqPos<T> {
        STailqPos { pos: self.head }
    }
}

impl<T> STailqIterMut<'_,T> {
    /// 转换为`SListPos`
    pub fn as_pos(&self) -> STailqPos<T> {
        STailqPos { pos: self.head }
    }
}

impl<T> STailqPos<T> {
    /// 移动到下一个位置
    pub fn next(&mut self)->Result<(),()>{
        if self.pos.is_null() {return Err(());}
        unsafe {
            self.pos = (*self.pos).next;
            Ok(())
        }
    }
    /// 移动多个位置
    pub fn advance(&mut self, dis: isize) -> Result<(),()> {
        if dis < 0 {
            return Err(());
        }
        for _ in 0..dis {
            self.next()?
        }
        Ok(())
    }

    pub fn is_tail(&self) -> bool {
        unsafe {(*self.pos).next.is_null()}
    }

    pub fn is_null(&self) -> bool{
        self.pos.is_null()
    }
}

impl<T> Deref for STailqPos<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe {(*self.pos).element.as_ref().unwrap()}
    }
}

impl<T> STailqPosMut<T> {
    /// 移动到下一个位置
    pub fn next(&mut self)->Result<(),()>{
        if self.pos.is_null() {return Err(());}
        unsafe {
            self.pos = (*self.pos).next;
            Ok(())
        }
    }
    /// 移动多个位置
    pub fn advance(&mut self, dis: isize) -> Result<(),()> {
        if dis < 0 {
            return Err(());
        }
        for _ in 0..dis {
            self.next()?
        }
        Ok(())
    }

    pub fn is_tail(&self) -> bool {
        unsafe {(*self.pos).next.is_null()}
    }

    pub fn is_null(&self) -> bool{
        self.pos.is_null()
    }
}

impl<T> Deref for STailqPosMut<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe {(*self.pos).element.as_ref().unwrap()}
    }
}

impl<T> DerefMut for STailqPosMut<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {(*self.pos).element.as_mut().unwrap()}
    }
}
