// kernel内で使用するアロケーター
use crate::dev::uart;
use core::alloc::{GlobalAlloc, Layout};

// リンカースクリプトで定義したHeap
extern "C" {
    static __kernel_heap_start__: u64;
    static __kernel_heap_end__: u64;
}

#[inline]
unsafe fn kernel_heap_start() -> u64 {
    &__kernel_heap_start__ as *const _ as u64
}

#[inline]
unsafe fn kernel_heap_end() -> u64 {
    &__kernel_heap_end__ as *const _ as u64
}

// メモリ管理struct
struct MemInfo {
    start: u64,
    size: usize,
}
static mut MEM_INFO: MemInfo = MemInfo { start: 0, size: 0 };

// メモリ初期化
pub fn init() {
    unsafe {
        MEM_INFO.start = kernel_heap_start();
        MEM_INFO.size = (kernel_heap_end() - kernel_heap_start()) as usize;
    }
}

// アロケーションサイズ算出
fn alloc_size(size: usize, align: usize) -> usize {
    let adjust = if size % align > 0 { 1 } else { 0 };
    align * (size / align + adjust)
}

#[alloc_error_handler]
fn oom_handler(_: Layout) -> ! {
    uart::Uart::get_instance().send(b"allocation error\n");
    loop {}
}

// メモリアロケーター
pub struct KernelAllocator;
unsafe impl GlobalAlloc for KernelAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // アライメントしたサイズ分、確保
        let size = alloc_size(layout.size(), layout.align());

        // 現在使用していない領域を返し、サイズ分を更新
        let addr = MEM_INFO.start as *mut u8;
        MEM_INFO.start = MEM_INFO.start + size as u64;
        MEM_INFO.size = MEM_INFO.size - size;

        addr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        // ひとまず、メモリ解放は考慮しない
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_case]
    fn alloc_size() {
        assert_eq!(4, alloc_size(1, 4));
        assert_eq!(4, alloc_size(2, 4));
        assert_eq!(4, alloc_size(4, 4));
        assert_eq!(8, alloc_size(5, 4));
        assert_eq!(8, alloc_size(6, 4));
        assert_eq!(8, alloc_size(8, 4));
    }
}
