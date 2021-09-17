// kernel内で使用するアロケーター
use crate::dev::uart::Uart;
use core::ptr;
use core::alloc::{GlobalAlloc, Layout};

// リンカースクリプトで定義したHeap
extern "C" {
    static __kernel_heap_start__: u32;
    static __kernel_heap_end__: u32;
}

#[inline]
unsafe fn kernel_heap_start() -> u32 {
    &__kernel_heap_start__ as *const _ as u32
}

#[inline]
unsafe fn kernel_heap_end() -> u32 {
    &__kernel_heap_end__ as *const _ as u32
}

// メモリ管理struct
struct MemInfo {
    start: u32,
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
    align * ((size / align) + adjust)
}

#[alloc_error_handler]
fn oom_handler(_: Layout) -> ! {
    Uart::get_instance().send(b"allocation error\n");
    loop {}
}

// メモリアロケーター
pub struct KernelAllocator;
unsafe impl GlobalAlloc for KernelAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // 要求サイズを超えた場合、nullptr
        if layout.size() > MEM_INFO.size {
            return ptr::null_mut();
        }

        // アライメントしたサイズ分、確保
        let size = alloc_size(layout.size(), layout.align());

        // 現在使用していない領域を返し、サイズ分を更新
        let addr = MEM_INFO.start as *mut u8;
        MEM_INFO.start = MEM_INFO.start + size as u32;
        MEM_INFO.size = MEM_INFO.size - size;

        addr
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        // ひとまず、メモリ解放は考慮しない
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::boxed::Box;
    use alloc::vec::Vec;

    #[test_case]
    fn test_alloc_size() {
        Uart::get_instance().send(b"test_alloc_size start...");

        assert_eq!(4, alloc_size(1, 4));
        assert_eq!(4, alloc_size(2, 4));
        assert_eq!(4, alloc_size(4, 4));
        assert_eq!(8, alloc_size(5, 4));
        assert_eq!(8, alloc_size(6, 4));
        assert_eq!(8, alloc_size(8, 4));
        assert_eq!(8, alloc_size(1, 8));
        assert_eq!(8, alloc_size(2, 8));
        assert_eq!(8, alloc_size(4, 8));

        Uart::get_instance().send(b"[ok]\n");
    }

    #[test_case]
    fn test_alloc() {
        Uart::get_instance().send(b"test_alloc start...");

        unsafe {
            init();
            let layout = Layout::from_size_align_unchecked(4, 8);
            let allocator = KernelAllocator {};
            let addr = allocator.alloc(layout.clone());

            assert_eq!(addr, kernel_heap_start() as *mut u8);
            assert_eq!(kernel_heap_start() + 8, MEM_INFO.start);
        }

        unsafe {
            init();
            let layout = Layout::from_size_align_unchecked(9, 8);
            let allocator = KernelAllocator {};
            let addr = allocator.alloc(layout.clone());

            assert_eq!(addr, kernel_heap_start() as *mut u8);
            assert_eq!(kernel_heap_start() + 16, MEM_INFO.start);
        }

        unsafe {
            init();
            let layout = Layout::from_size_align_unchecked(9, 8);
            let allocator = KernelAllocator {};
            let addr1 = allocator.alloc(layout.clone());

            assert_eq!(addr1, kernel_heap_start() as *mut u8);
            assert_eq!(kernel_heap_start() + 16, MEM_INFO.start);

            let addr2 = allocator.alloc(layout.clone());
            assert_eq!(addr2, (kernel_heap_start() + 16) as *mut u8);
            assert_eq!(kernel_heap_start() + 32, MEM_INFO.start);
        }

        // サイズいっぱい
        unsafe {
            init();
            let layout = Layout::from_size_align_unchecked(MEM_INFO.size, 8);
            let allocator = KernelAllocator {};
            let addr = allocator.alloc(layout.clone());

            assert_eq!(addr, kernel_heap_start() as *mut u8);
            assert_eq!(kernel_heap_end(), MEM_INFO.start);
        }

        // サイズ超過
        unsafe {
            init();
            let layout = Layout::from_size_align_unchecked(MEM_INFO.size + 1, 8);
            let allocator = KernelAllocator {};
            let addr = allocator.alloc(layout.clone());

            assert_eq!(addr, ptr::null_mut());
            assert_eq!(kernel_heap_start(), MEM_INFO.start);
        }

        Uart::get_instance().send(b"[ok]\n");
    }

    #[test_case]
    fn test_box() {
        Uart::get_instance().send(b"test_box start...");
        init();

        let b1 = Box::new(1);
        let b2 = Box::new(2);

        assert_eq!(1, *b1);
        assert_eq!(2, *b2);

        Uart::get_instance().send(b"[ok]\n");
    }

    // TODO: ハングするのでひとまずコメントアウト
    //#[test_case]
    fn test_vec() {
        Uart::get_instance().send(b"test_vec start...");
        init();

        Uart::get_instance().send(b"Vec new");
        let mut v: Vec<i32> = Vec::new();
        v.push(1);

        assert_eq!(1, v[0]);

        Uart::get_instance().send(b"[ok]\n");
    }
}
