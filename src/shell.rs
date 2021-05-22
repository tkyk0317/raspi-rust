use crate::dev::uart;

const BUF_SIZE: usize = 256;

pub struct Shell {
    pub buf: [u8; BUF_SIZE],
}

// UARTからの割り込み通知trait
impl uart::UartObserver for Shell {
    // 割り込み通知
    fn notify(&self, data: u32) {
        let c = data as u8;
        if c == 0xD {
            // 改行処理
            uart::Uart::get_instance().send(b"\n");
        } else {
            uart::Uart::get_instance().send(&[c]);
        }
    }
}

impl Shell {
    pub fn new() -> Self {
        Shell { buf: [0; BUF_SIZE] }
    }

    // シェル起動
    pub fn start(&mut self) {
        uart::Uart::get_instance().send(b"==============================\n");
        uart::Uart::get_instance().send(b"Start Kernel\n");
        uart::Uart::get_instance().send(b"==============================\n");

        loop {
            unsafe { llvm_asm!("wfi"); }
        }
    }
}
