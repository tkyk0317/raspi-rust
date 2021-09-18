use crate::{
    dev::uart::{Uart, UartObserver},
    println,
};

const BUF_SIZE: usize = 256;

pub struct Shell {
    pub buf: [u8; BUF_SIZE],
}

// UARTからの割り込み通知trait
impl UartObserver for Shell {
    // 割り込み通知
    fn notify(&self, data: u32) {
        let c = data as u8;
        if c == 0xD {
            // 改行処理
            Uart::get_instance().send(b"\n");
        } else {
            Uart::get_instance().send(&[c]);
        }
    }
}

impl Shell {
    // シェル起動
    pub fn start(&mut self) {
        Uart::get_instance().send(b"==============================\n");
        Uart::get_instance().send(b"Start Kernel\n");
        Uart::get_instance().send(b"==============================\n");

        loop {
            unsafe {
                llvm_asm!("wfi");
            }
        }
    }
}
