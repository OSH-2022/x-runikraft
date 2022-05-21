use rkalloc::RKalloc;
use core::ops::{Deref, DerefMut};
use core::ptr::null_mut;
use core::marker::PhantomData;
use core::iter::{Iterator,ExactSizeIterator};

#[repr(C)]
struct Node<T> {
    element: Option<T>,     //为了pop_front方法能获取element
    prev: *mut Node<T>,
    next: *mut Node<T>,
}

impl<T> Node<T> {
    fn new(element: T) -> Self {
        Node { prev: null_mut(), next: null_mut(), element: Some(element)}
    }
    fn null() -> Self {
        Node {prev: null_mut(), next: null_mut(), element: None}
    }
}

/// 双向尾队列
/// 
/// 支持的操作：
/// - new                   创建新链表
/// - is_empty              是否为空
/// - len                   长度
/// - front/front_mut       第一个元素
/// - last/last_mut         最后一个元素
/// - contains              是否包含某个元素
/// - push_front            头插入
/// - pop_front             弹出头
/// - push_back             尾插入
/// - pop_back              弹出尾
/// - clear                 清空
/// - iter/iter_mut         迭代器
/// - iter_rev/iter_rev_mut 反向迭代器
/// - head/head_mut         头结点
/// - tail/tail_mut         尾结点
/// - insert_before         在指定位置前插入
/// - insert_after          指定位置之后插入
/// - remove_before         删除指定位置之前的元素
/// - remove                删除指定位置的元素
/// - remove_after          删除指定位置之后的元素
pub struct Tailq<'a,T> {
    head: *mut Node<T>,
    tail: *mut Node<T>,
    alloc: &'a dyn RKalloc,
    marker: PhantomData<*const Node<T>>,
    size: usize,
}

/// 不可变迭代器
pub struct TailqIter<'a, T:'a> {
    head: *const Node<T>,
    size: usize,
    marker: PhantomData<&'a Node<T>>,
}

/// 可变迭代器
pub struct TailqIterMut<'a, T:'a> {
    head: *mut Node<T>,
    size: usize,
    marker: PhantomData<&'a Node<T>>,
}

/// 不可变反向迭代器
pub struct TailqIterRev<'a, T:'a> {
    tail: *const Node<T>,
    size: usize,
    marker: PhantomData<&'a Node<T>>,
}

/// 可变反向迭代器
pub struct TailqIterRevMut<'a, T:'a> {
    tail: *mut Node<T>,
    size: usize,
    marker: PhantomData<&'a Node<T>>,
}

/// 位置
pub struct TailqPos<T> {
    pos: *const Node<T>
}

pub struct TailqPosMut<T> {
    pos: *mut Node<T>
}

impl<T> Clone for TailqPos<T> {
    fn clone(&self) -> Self {
        Self {pos: self.pos}
    }
}

impl<T> Copy for TailqPos<T> {

}

impl<T> Clone for TailqPosMut<T> {
    fn clone(&self) -> Self {
        Self {pos: self.pos}
    }
}

impl<T> Copy for TailqPosMut<T> {

}

impl<'a,T> Tailq<'a,T> {
    /// 构造双向尾队列
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
            unsafe{
                self.head = (*ptr).next;
                // 删除最后一个结点后，head is null
                if !self.head.is_null() {
                    debug_assert!(!(*self.head).prev.is_null());
                    (*self.head).prev = null_mut();
                }
                else {
                    self.tail = null_mut();
                }
            }
            let old_head = unsafe{ptr.replace(Node::null())};
            unsafe{rkalloc::dealloc_type(self.alloc, ptr);}
            old_head.element
        }
    }

    /// 在尾部插入新结点
    #[inline]
    pub fn push_back(&mut self, element: T) -> Result<(),&'static str> {
        self.push_back_node(unsafe{rkalloc::alloc_type(self.alloc,Node::new(element))})?;
        self.size += 1;
        Ok(())
    }

    /// 弹出头部的结点
    pub fn pop_back(&mut self) -> Option<T> {
        if self.tail.is_null() {
            return None;
        }
        let ptr = self.tail;
        unsafe {
            self.tail = (*self.tail).prev;
            if !self.tail.is_null() {
                (*self.tail).next = null_mut();
            }
            else {
                self.head = null_mut();
            }
        }
        self.size -= 1;
        let old_tail = unsafe{ptr.replace(Node::null())};
        unsafe{rkalloc::dealloc_type(self.alloc,ptr);}
        old_tail.element
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
    pub fn iter<'b>(&'b self) -> TailqIter<'b,T> {
        TailqIter { head: self.head, size: self.size, marker: PhantomData }
    }

    /// 可变迭代器
    #[inline]
    pub fn iter_mut<'b>(&'b mut self) -> TailqIterMut<'b,T> {
        TailqIterMut { head: self.head, size: self.size, marker: PhantomData}
    }

    /// 不可变反向迭代器
    #[inline]
    pub fn iter_rev<'b>(&'b self) -> TailqIterRev<'b,T> {
        TailqIterRev { tail: self.tail, size: self.size, marker: PhantomData }
    }

    /// 可变反向迭代器
    #[inline]
    pub fn iter_rev_mut<'b>(&'b mut self) -> TailqIterRevMut<'b,T> {
        TailqIterRevMut { tail: self.tail, size: self.size, marker: PhantomData}
    }

    /// 头结点
    /// 
    /// 与`iter`不同，`head`产生的位置不会被视为self的引用
    #[inline]
    pub fn head(&self) -> TailqPos<T> {
        TailqPos { pos: self.head }
    }

    /// 头结点
    #[inline]
    pub fn head_mut(&mut self) -> TailqPosMut<T> {
        TailqPosMut { pos: self.head }
    }

    /// 尾
    #[inline]
    pub fn tail(&self) -> TailqPos<T> {
        TailqPos { pos: self.tail }
    }

    /// 尾结点
    #[inline]
    pub fn tail_mut(&mut self) -> TailqPosMut<T> {
        TailqPosMut { pos: self.tail }
    }

    /// 在迭代器指向的位置之前插入
    /// 
    /// # 安全性
    /// 
    /// `pos`必须和`self`属于同一个链表
    pub unsafe fn insert_before(&mut self, pos: TailqPosMut<T>, element: T) -> Result<(),&'static str>{
        if pos.pos.is_null() {
            return Err("invalid position");
        }
        let node = rkalloc::alloc_type(self.alloc, Node::new(element));
        if node.is_null() {return Err("fail to allocate memory");}
        (*node).prev=(*pos.pos).prev;
        (*node).next=pos.pos;
        (*(pos.pos)).prev = node;
        if !(*node).prev.is_null() {
            (*(*node).prev).next=node;
        }
        else {
            debug_assert!(self.head == pos.pos);
            self.head = node;
        }
        self.size += 1;
        Ok(())
    }

    /// 在迭代器指向的位置之后插入
    /// 
    /// # 安全性
    /// 
    /// `pos`必须和`self`属于同一个链表
    pub unsafe fn insert_after(&mut self, pos: TailqPosMut<T>, element: T) -> Result<(),&'static str>{
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
        else {
            self.tail = node;
        }
        self.size += 1;
        Ok(())
    }

    /// 在迭代器指向的位置之前删除
    /// 
    /// # 安全性
    /// 
    /// `pos`必须和`self`属于同一个链表
    pub unsafe fn remove_before(&mut self, pos: TailqPosMut<T>) -> Option<T> {
        assert!(!pos.pos.is_null());
        if (*pos.pos).prev.is_null() {
            None
        }
        else {
            self.size -= 1;
            let ptr = (*pos.pos).prev;
            (*(pos.pos)).prev = (*ptr).prev;
            if !(*ptr).prev.is_null() {
                (*(*ptr).prev).next = pos.pos;
            }
            else {
                debug_assert!(self.head == ptr);
                self.head = pos.pos;
            }
            let old_head = ptr.replace(Node::null());
            rkalloc::dealloc_type(self.alloc, ptr);
            old_head.element
        }
    }

    /// 删除迭代器指向的位置
    /// 
    /// 返回值：(被删除的元素, pos.next()或pos.prev()或空位置)
    /// 
    /// # 安全性
    /// 
    /// - `pos`必须和`self`属于同一个链表
    /// - 成功删除后，ListPosMut<T>将无效
    pub unsafe fn remove(&mut self, pos: TailqPosMut<T>) -> (T,TailqPosMut<T>) {
        assert!(!pos.pos.is_null());
        self.size -= 1;
        let ptr = pos.pos;
        if !(*ptr).next.is_null() {
            (*(*ptr).next).prev = (*ptr).prev;
            if !(*ptr).prev.is_null() {
                (*(*ptr).prev).next = (*ptr).next;
            }
            else {
                debug_assert!(self.head == ptr);
                self.head = (*ptr).next;
            }
            let old_head = ptr.replace(Node::null());
            rkalloc::dealloc_type(self.alloc, ptr);
            (old_head.element.unwrap(), TailqPosMut{pos: old_head.next})
        }
        else if !(*ptr).prev.is_null() {
            self.tail = (*ptr).prev;
            (*(*ptr).prev).next = (*ptr).next;
            let old_head = ptr.replace(Node::null());
            rkalloc::dealloc_type(self.alloc, ptr);
            (old_head.element.unwrap(), TailqPosMut{pos: old_head.prev})
        }
        else {
            self.head = null_mut();
            self.tail = null_mut();
            let old_head = ptr.replace(Node::null());
            (old_head.element.unwrap(), TailqPosMut{pos: null_mut()})
        }
    }

    /// 在迭代器指向的位置之后删除
    /// 
    /// # 安全性
    /// 
    /// `pos`必须和`self`属于同一个链表
    pub unsafe fn remove_after(&mut self, pos: TailqPosMut<T>) -> Option<T> {
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
            else {
                self.tail = ptr;
            }
            let old_head = ptr.replace(Node::null());
            rkalloc::dealloc_type(self.alloc, ptr);
            old_head.element
        }
    }
}

impl<'a,T> Tailq<'a,T> {
    fn push_front_node(&mut self, node: *mut Node<T>) -> Result<(),&'static str>{
        if node.is_null() {return Err("fail to allocate memory");}
        unsafe{
            debug_assert!((*node).prev.is_null());
            (*node).next = self.head;
            if !self.head.is_null() {
                (*self.head).prev = node;
            }
            else {
                self.tail = node;
            }
            self.head = node;
        }
        Ok(())
    }

    fn push_back_node(&mut self, node: *mut Node<T>) -> Result<(),&'static str>{
        if node.is_null() {return Err("fail to allocate memory");}
        unsafe{
            debug_assert!((*node).prev.is_null());
            debug_assert!((*node).next.is_null());
            if self.tail.is_null() {
                debug_assert!(self.head.is_null());
                self.tail=node;
                self.head=node;
            }
            else {
                (*self.tail).next = node;
                (*node).prev = self.tail;
                self.tail = node;
            }
        }
        Ok(())
    }
}

impl<'a,T> Drop for Tailq<'a,T> {
    fn drop(&mut self) {
        self.clear();
    }
}

impl<'a,T> Iterator for TailqIter<'a,T> {
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

impl<'a,T> TailqIter<'a,T> {
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

impl<T> ExactSizeIterator for TailqIter<'_,T> {
    fn len(&self) -> usize {
        self.size
    }
}

impl<'a,T> Iterator for TailqIterRev<'a,T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<&'a T> {
        let ret = self.tail;
        if ret.is_null() {None}
        else {
            self.size -= 1;
            unsafe{
                self.tail = (*self.tail).prev;
                (*ret).element.as_ref()
            }
        }
    }
}

impl<'a,T> TailqIterRev<'a,T> {
    pub fn prev(&mut self) -> Option<&'a T> {
        let ret = self.tail;
        if ret.is_null() {None}
        else {
            self.size += 1;
            unsafe{
                self.tail = (*self.tail).next;
                (*ret).element.as_ref()
            }
        }
    }
}

impl<T> ExactSizeIterator for TailqIterRev<'_,T> {
    fn len(&self) -> usize {
        self.size
    }
}

impl<'a,T> Iterator for TailqIterMut<'a,T> {
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

impl<'a,T> TailqIterMut<'a,T> {
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

impl<T> ExactSizeIterator for TailqIterMut<'_,T> {
    fn len(&self) -> usize {
        self.size
    }
}

impl<'a,T> Iterator for TailqIterRevMut<'a,T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<&'a mut T> {
        let ret = self.tail;
        if ret.is_null() {None}
        else {
            self.size -= 1;
            unsafe{
                self.tail = (*self.tail).prev;
                (*ret).element.as_mut()
            }
        }
    }
}

impl<'a,T> TailqIterRevMut<'a,T> {
    pub fn prev(&mut self) -> Option<&'a mut T> {
        let ret = self.tail;
        if ret.is_null() {None}
        else {
            self.size += 1;
            unsafe{
                self.tail = (*self.tail).next;
                (*ret).element.as_mut()
            }
        }
    }
}

impl<T> ExactSizeIterator for TailqIterRevMut<'_,T> {
    fn len(&self) -> usize {
        self.size
    }
}

impl<T> TailqIter<'_,T> {
    /// 转换为`ListPos`
    pub fn as_pos(&self) -> TailqPos<T> {
        TailqPos { pos: self.head }
    }
}

impl<T> TailqIterMut<'_,T> {
    /// 转换为`ListPosMut`
    pub fn as_pos(&self) -> TailqPosMut<T> {
        TailqPosMut { pos: self.head }
    }
}

impl<T> TailqIterRev<'_,T> {
    /// 转换为`ListPos`
    pub fn as_pos(&self) -> TailqPos<T> {
        TailqPos { pos: self.tail }
    }
}

impl<T> TailqIterRevMut<'_,T> {
    /// 转换为`ListPosMut`
    pub fn as_pos(&self) -> TailqPosMut<T> {
        TailqPosMut { pos: self.tail }
    }
}

impl<T> TailqPos<T> {
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
    /// 移动到上一个位置
    pub fn prev(&mut self)->Result<(),()>{
        if self.pos.is_null() {return Err(());}
        unsafe {
            self.pos = (*self.pos).prev;
            Ok(())
        }
    }
    /// 移动多个位置
    pub fn advance(&mut self, dis: isize) -> Result<(),()> {
        if dis > 0 {
            for _ in 0..dis {
                self.next()?
            }
        }
        else if dis < 0 {
            for _ in 0..-dis {
                self.prev()?
            }
        }
        Ok(())
    }

    pub fn is_head(&self) -> bool {
        unsafe {(*self.pos).prev.is_null()}
    }

    pub fn is_tail(&self) -> bool {
        unsafe {(*self.pos).next.is_null()}
    }
    
    pub fn is_null(&self) -> bool{
        self.pos.is_null()
    }
}

impl<T> Deref for TailqPos<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe {(*self.pos).element.as_ref().unwrap()}
    }
}

impl<T> TailqPosMut<T> {
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
    /// 移动到上一个位置
    pub fn prev(&mut self)->Result<(),()>{
        if self.pos.is_null() {return Err(());}
        unsafe {
            self.pos = (*self.pos).prev;
            Ok(())
        }
    }
    /// 移动多个位置
    pub fn advance(&mut self, dis: isize) -> Result<(),()> {
        if dis > 0 {
            for _ in 0..dis {
                self.next()?
            }
        }
        else if dis < 0 {
            for _ in 0..-dis {
                self.prev()?
            }
        }
        Ok(())
    }

    pub fn is_head(&self) -> bool {
        unsafe {(*self.pos).prev.is_null()}
    }

    pub fn is_tail(&self) -> bool {
        unsafe {(*self.pos).next.is_null()}
    }

    pub fn is_null(&self) -> bool{
        self.pos.is_null()
    }
}

impl<T> Deref for TailqPosMut<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe {(*self.pos).element.as_ref().unwrap()}
    }
}

impl<T> DerefMut for TailqPosMut<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {(*self.pos).element.as_mut().unwrap()}
    }
}
