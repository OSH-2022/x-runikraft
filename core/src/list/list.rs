use rkalloc::RKalloc;
use core::ops::{Deref, DerefMut};
use core::ptr::null_mut;
use core::marker::PhantomData;
use core::iter::{Iterator,ExactSizeIterator};

struct Node<T> {
    prev: *mut Node<T>,
    next: *mut Node<T>,
    element: Option<T>,     //为了pop_front方法能获取element
}

impl<T> Node<T> {
    fn new(element: T) -> Self {
        Node { prev: null_mut(), next: null_mut(), element: Some(element)}
    }
    fn null() -> Self {
        Node {prev: null_mut(), next: null_mut(), element: None}
    }
}

/// 双链表
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
/// - head/head_mut         头结点
/// - insert_before         在指定位置前插入
/// - insert_after          指定位置之后插入
/// - remove_before         删除指定位置之前的元素
/// - remove                删除指定位置的元素
/// - remove_after          删除指定位置之后的元素
pub struct List<'a,T> {
    head: *mut Node<T>,
    alloc: &'a dyn RKalloc,
    marker: PhantomData<*const Node<T>>,
    size: usize,
}

/// 不可变迭代器
pub struct ListIter<'a, T:'a> {
    head: *const Node<T>,
    size: usize,
    marker: PhantomData<&'a Node<T>>,
}

/// 可变迭代器
pub struct ListIterMut<'a, T:'a> {
    head: *mut Node<T>,
    size: usize,
    marker: PhantomData<&'a Node<T>>,
}

/// 位置
pub struct ListPos<T> {
    pos: *const Node<T>
}

pub struct ListPosMut<T> {
    pos: *mut Node<T>
}

impl<T> Clone for ListPos<T> {
    fn clone(&self) -> Self {
        Self {pos: self.pos}
    }
}

impl<T> Copy for ListPos<T> {

}

impl<T> Clone for ListPosMut<T> {
    fn clone(&self) -> Self {
        Self {pos: self.pos}
    }
}

impl<T> Copy for ListPosMut<T> {

}

impl<'a,T> List<'a,T> {
    /// 构造双链表
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
            unsafe{
                self.head = (*ptr).next;
                debug_assert!(!(*self.head).prev.is_null());
                (*self.head).prev = null_mut();
            }
            let old_head = unsafe{ptr.replace(Node::null())};
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
    pub fn iter<'b>(&'b self) -> ListIter<'b,T> {
        ListIter { head: self.head, size: self.size, marker: PhantomData }
    }

    /// 可变迭代器
    #[inline]
    pub fn iter_mut<'b>(&'b mut self) -> ListIterMut<'b,T> {
        ListIterMut { head: self.head, size: self.size, marker: PhantomData}
    }

    /// 头结点
    /// 
    /// 与`iter`不同，`head`产生的位置不会被视为self的引用
    #[inline]
    pub fn head(&self) -> ListPos<T> {
        ListPos { pos: self.head }
    }

    /// 头结点
    #[inline]
    pub fn head_mut(&mut self) -> ListPosMut<T> {
        ListPosMut { pos: self.head }
    }

    /// 在迭代器指向的位置之后插入
    /// 
    /// # 安全性
    /// 
    /// `pos`必须和`self`属于同一个链表
    pub unsafe fn insert_after(&mut self, pos: ListPosMut<T>, element: T) -> Result<(),&'static str>{
        if pos.pos.is_null() {
            return Err("invalid position");
        }
        let node = rkalloc::alloc_type(self.alloc, Node::new(element));
        if node.is_null() {return Err("fail to allocate memory");}
        (*node).next=(*pos.pos).next;
        (*node).prev=pos.pos;
        (*(pos.pos)).next = node;
        if !(*node).next.is_null() {
            (*(*node).next).prev=node;
        }
        self.size += 1;
        Ok(())
    }

    /// 在迭代器指向的位置之后删除
    /// 
    /// # 安全性
    /// 
    /// `pos`必须和`self`属于同一个链表
    pub unsafe fn remove_after(&mut self, pos: ListPosMut<T>) -> Option<T> {
        assert!(!pos.pos.is_null());
        if (*pos.pos).next.is_null() {
            None
        }
        else {
            self.size -= 1;
            let ptr = (*pos.pos).next;
            (*(pos.pos)).next = (*ptr).next;
            if !(*ptr).next.is_null() {
                (*(*ptr).next).prev = pos.pos;
            }
            let old_head = ptr.replace(Node::null());
            rkalloc::dealloc_type(self.alloc, ptr);
            old_head.element
        }
    }
}

impl<'a,T> List<'a,T> {
    fn push_front_node(&mut self, mut node: *mut Node<T>) -> Result<(),&'static str>{
        if node.is_null() {return Err("fail to allocate memory");}
        unsafe{
            debug_assert!((*node).prev.is_null());
            (*node).next = self.head;
            self.head = node;
        }
        Ok(())
    }
}

impl<'a,T> Drop for List<'a,T> {
    fn drop(&mut self) {
        self.clear();
    }
}

impl<'a,T> Iterator for ListIter<'a,T> {
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

impl<'a,T> ListIter<'a,T> {
    /// 与next相对，返回当前元素，并把迭代器向靠近表头的方向移动
    pub fn prev(&mut self) -> Option<&'a T> {
        let ret = self.head;
        if ret.is_null() {None}
        else {
            self.size += 1;
            unsafe{
                self.head = (*self.head).prev;
                (*ret).element.as_ref()
            }
        }
    }
}

impl<T> ExactSizeIterator for ListIter<'_,T> {
    fn len(&self) -> usize {
        self.size
    }
}

impl<'a,T> Iterator for ListIterMut<'a,T> {
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

impl<'a,T> ListIterMut<'a,T> {
    /// 与next相对，返回当前元素，并把迭代器向靠近表头的方向移动
    pub fn prev(&mut self) -> Option<&'a mut T> {
        let ret = self.head;
        if ret.is_null() {None}
        else {
            self.size += 1;
            unsafe{
                self.head = (*self.head).prev;
                (*ret).element.as_mut()
            }
        }
    }
}

impl<T> ExactSizeIterator for ListIterMut<'_,T> {
    fn len(&self) -> usize {
        self.size
    }
}

impl<T> ListIter<'_,T> {
    /// 转换为`ListPos`
    pub fn as_pos(&self) -> ListPos<T> {
        ListPos { pos: self.head }
    }
}

impl<T> ListIterMut<'_,T> {
    /// 转换为`SListPos`
    pub fn as_pos(&self) -> ListPos<T> {
        ListPos { pos: self.head }
    }
}

impl<T> ListPos<T> {
    /// 移动到下一个位置
    pub fn next(&mut self)->Result<(),()>{
        if self.pos.is_null() {return Err(());}
        unsafe {
            self.pos = (*self.pos).next;
            Ok(())
        }
    }
    /// 移动多个位置
    pub fn advance(&mut self, dis: usize) -> Result<(),()> {
        for _ in 0..dis {
            self.next()?
        }
        Ok(())
    }
    pub fn is_null(&self) -> bool{
        self.pos.is_null()
    }
}

impl<T> Deref for ListPos<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe {(*self.pos).element.as_ref().unwrap()}
    }
}

impl<T> ListPosMut<T> {
    /// 移动到下一个位置
    pub fn next(&mut self)->Result<(),()>{
        if self.pos.is_null() {return Err(());}
        unsafe {
            self.pos = (*self.pos).next;
            Ok(())
        }
    }
    /// 移动多个位置
    pub fn advance(&mut self, dis: usize) -> Result<(),()> {
        for _ in 0..dis {
            self.next()?
        }
        Ok(())
    }
    pub fn is_null(&self) -> bool{
        self.pos.is_null()
    }
}

impl<T> Deref for ListPosMut<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe {(*self.pos).element.as_ref().unwrap()}
    }
}

impl<T> DerefMut for ListPosMut<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {(*self.pos).element.as_mut().unwrap()}
    }
}
