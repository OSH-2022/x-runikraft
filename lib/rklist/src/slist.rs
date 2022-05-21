use rkalloc::RKalloc;
use core::ops::{Deref, DerefMut};
use core::ptr::null_mut;
use core::marker::PhantomData;
use core::iter::{Iterator,ExactSizeIterator};

#[repr(C)]
struct Node<T> {
    element: Option<T>,     //为了pop_front方法能获取element
    next: *mut Node<T>,
}

impl<T> Node<T> {
    fn new(element: T) -> Self {
        Node { next: null_mut(), element: Some(element)}
    }
    fn null() -> Self {
        Node { next: null_mut(), element: None}
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
/// - head/head_mut         头结点
/// - insert_after          指定位置之后插入
/// - remove_after          删除指定位置之后的元素
pub struct SList<'a,T> {
    head: *mut Node<T>,
    alloc: &'a dyn RKalloc,
    marker: PhantomData<*const Node<T>>,
    size: usize,
}

/// 不可变迭代器
pub struct SListIter<'a, T:'a> {
    head: *const Node<T>,
    size: usize,
    marker: PhantomData<&'a Node<T>>,
}

/// 可变迭代器
pub struct SListIterMut<'a, T:'a> {
    head: *mut Node<T>,
    size: usize,
    marker: PhantomData<&'a Node<T>>,
}

/// 位置
pub struct SListPos<T> {
    pos: *const Node<T>
}

pub struct SListPosMut<T> {
    pos: *mut Node<T>
}

impl<T> Clone for SListPos<T> {
    fn clone(&self) -> Self {
        Self {pos: self.pos}
    }
}

impl<T> Copy for SListPos<T> {

}

impl<T> Clone for SListPosMut<T> {
    fn clone(&self) -> Self {
        Self {pos: self.pos}
    }
}

impl<T> Copy for SListPosMut<T> {

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
    pub fn iter<'b>(&'b self) -> SListIter<'b,T> {
        SListIter { head: self.head, size: self.size, marker: PhantomData }
    }

    /// 可变迭代器
    #[inline]
    pub fn iter_mut<'b>(&'b mut self) -> SListIterMut<'b,T> {
        SListIterMut { head: self.head, size: self.size, marker: PhantomData}
    }

    /// 头结点
    /// 
    /// 与`iter`不同，`head`产生的位置不会被视为self的引用
    #[inline]
    pub fn head(&self) -> SListPos<T> {
        SListPos { pos: self.head }
    }

    /// 头结点
    #[inline]
    pub fn head_mut(&mut self) -> SListPosMut<T> {
        SListPosMut { pos: self.head }
    }

    /// 在迭代器指向的位置之后插入
    /// 
    /// # 安全性
    /// 
    /// `pos`必须和`self`属于同一个链表
    pub unsafe fn insert_after(&mut self, pos: SListPosMut<T>, element: T) -> Result<(),&'static str>{
        if pos.pos.is_null() {
            return Err("invalid position");
        }
        let node = rkalloc::alloc_type(self.alloc, Node::new(element));
        if node.is_null() {return Err("fail to allocate memory");}
        (*node).next=(*pos.pos).next;
        (*(pos.pos)).next = node;
        self.size += 1;
        Ok(())
    }

    /// 在迭代器指向的位置之后删除
    /// 
    /// # 安全性
    /// 
    /// `pos`必须和`self`属于同一个链表
    pub unsafe fn remove_after(&mut self, pos: SListPosMut<T>) -> Option<T> {
        assert!(!pos.pos.is_null());
        if (*pos.pos).next.is_null() {
            None
        }
        else {
            self.size -= 1;
            let ptr = (*pos.pos).next;
            (*(pos.pos)).next = (*ptr).next;
            let old_head = ptr.replace(Node::null());
            rkalloc::dealloc_type(self.alloc, ptr);
            old_head.element
        }
    }
}

impl<'a,T> SList<'a,T> {
    fn push_front_node(&mut self, node: *mut Node<T>) -> Result<(),&'static str>{
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

impl<T> SListIter<'_,T> {
    /// 转换为`SListPos`
    pub fn as_pos(&self) -> SListPos<T> {
        SListPos { pos: self.head }
    }
}

impl<T> SListIterMut<'_,T> {
    /// 转换为`SListPosMut`
    pub fn as_pos(&self) -> SListPosMut<T> {
        SListPosMut { pos: self.head }
    }
}

impl<T> SListPos<T> {
    /// 由元素的引用创建
    pub unsafe fn from_ref(elem: &T) -> Self {
        Self { pos: elem as *const T as *const Node<T> }
    }

    /// 由元素的指针创建
    pub unsafe fn from_ptr(elem: *const T) -> Self {
        Self { pos: elem as *const Node<T> }
    }

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

impl<T> Deref for SListPos<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe {(*self.pos).element.as_ref().unwrap()}
    }
}

impl<T> SListPosMut<T> {
    /// 由元素的引用创建
    pub unsafe fn from_ref(elem: &mut T) -> Self {
        Self { pos: elem as *mut T as *mut Node<T> }
    }

    /// 由元素的指针创建
    pub unsafe fn from_ptr(elem: *mut T) -> Self {
        Self { pos: elem as *mut Node<T> }
    }

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

impl<T> Deref for SListPosMut<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe {(*self.pos).element.as_ref().unwrap()}
    }
}

impl<T> DerefMut for SListPosMut<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {(*self.pos).element.as_mut().unwrap()}
    }
}
