#![no_std]

/// Runikraft的内存分配器API
/// `RKalloc`没有模仿`uk_alloc`，而是模仿了`alloc::alloc::GlobalAlloc`，
/// 因为`GlobalAlloc`要求实现的函数更少，而且`dealloc`函数要求提供分配时
/// 的`size`和`align`，实现起来更容易
///
/// # 安全性
///
/// `self`具有内部可变性，但是RKalloc是底层的分配器，不能在分配时加锁，所以
/// 调用者必须保证，调用环境不能出现竞争
pub unsafe trait RKalloc {
    /// 分配大小为`size`，对齐要求为`align`(必须是2^n)的连续内存空间
    ///
    /// 成功时返回非空指针，失败时返回空指针
    unsafe fn alloc(&self, size: usize, align: usize) -> *mut u8;

    /// 解分配内存
    ///
    /// # 安全性
    ///
    /// - `ptr` 必须由同一个分配器分配
    /// - `size`和`align`必须和`alloc`时一致
    unsafe fn dealloc(&self, ptr: *mut u8, size: usize, align: usize);

    /// 与`alloc`类似，但在分配后清空内存
    unsafe fn alloc_zeroed(&self, size: usize, align: usize) -> *mut u8 {
        let ptr = self.alloc(size, align);
        if !ptr.is_null() {
            ptr.write_bytes(0, size);
        }
        ptr
    }

    /// 重新分配内存，将原有内存区域的数据照原样复制到新的内存区域，
    /// 成功时返回新内存区域的地址，并自动释放原有的空间
    /// 失败时返回空指针，原本的内存空间保持不变
    ///
    /// # 安全性
    ///
    /// - `old_ptr` 必须由同一个分配器分配
    /// - `old_size`和`align`必须和`alloc`时一致
    unsafe fn realloc(&self, old_ptr: *mut u8, old_size: usize, new_size: usize, align: usize) -> *mut u8 {
        if new_size == old_size {
            return old_ptr;
        }
        let new_ptr = self.alloc(new_size, align);
        if !new_ptr.is_null() {
            new_ptr.copy_from_nonoverlapping(old_ptr, old_size);
            self.dealloc(old_ptr, old_size, align);
        }
        new_ptr
    }
}

/// 为C接口拓展的内存分配器，相比Rkalloc，它支持运行时不提供size、alloc
pub unsafe trait RKallocExt: RKalloc {
    /// 解分配内存
    ///
    /// # 安全性
    ///
    /// - `ptr` 必须由同一个分配器分配
    unsafe fn dealloc_ext(&self, ptr: *mut u8);

    /// 重新分配内存，将原有内存区域的数据照原样复制到新的内存区域，
    /// 成功时返回新内存区域的地址，并自动释放原有的空间
    /// 失败时返回空指针，原本的内存空间保持不变
    ///
    /// # 安全性
    ///
    /// - `old_ptr` 必须由同一个分配器分配
    unsafe fn realloc_ext(&self, old_ptr: *mut u8, new_size: usize) -> *mut u8;
}

/// 分配器的状态信息
pub trait RKallocState {
    /// 总空间
    fn total_size(&self) -> usize;
    /// 可用空间
    ///
    /// **注意**: 
    /// - total_size-free_size不等于请求分配的空间的总大小，因为 1. 为了满足对齐要求和分配器
    /// 实现定义的最小分配空间要求，实际分配的空间大于等于请求分配的空间；2. 分配器的元数据需要
    /// 占用内存空间。
    /// - free_size大于等于请求分配的空间时不一定能分配成功，因为可能无法找到足够大的连续内存空间。
    fn free_size(&self) -> usize;
}
