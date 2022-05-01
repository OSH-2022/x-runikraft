use super::*;
use core::fmt;

impl fmt::Debug for RKallocBuddy<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "free_list_head:\n")?;
        for i in 4..=self.root_order {
            write!(f, "#{}: ", i)?;
            let mut ptr = self.free_list_head[i - MIN_ORDER];
            if ptr.is_null() {
                write!(f, "(empty)\n")?;
            } else {
                write!(f, "[ {:?}", unsafe { ptr.offset_from(self.base as *mut Node) })?;
                unsafe { ptr = (*ptr).next; }
                let mut j = 0;
                while ptr != self.free_list_head[i - MIN_ORDER] {
                    write!(f, ", {:?}", unsafe { ptr.offset_from(self.base as *mut Node) })?;
                    unsafe { ptr = (*ptr).next; }
                    j += 1;
                    if j > self.total_size() {
                        panic!("链表错误");
                    }
                }
                write!(f, " ]\n")?;
            }
        }
        Ok(())
    }
}
