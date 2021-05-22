use crate::dev::regs;
use alloc::boxed::Box;
use alloc::vec::Vec;

// UART MIS
const UART_MIS_RXMIS: u32 = 1 << 4; // RXマスク

// UART割り込み通知trait
pub trait UartObserver {
    fn notify(&self, data: u32);
}

// UARTデバイス
pub struct Uart<'a> {
    observer: Option<Box<&'a mut (dyn UartObserver + 'a)>>,
    //observers: Vec<Box<&'a mut (dyn UartObserver + 'a)>>
}
static mut UART: Uart = Uart { observer: None };
//static mut UART: Uart = Uart { observer: Vec::new() };

impl<'a> Uart<'a> {
    //  インスタンス取得
    pub fn get_instance() -> &'static mut Self {
        unsafe { &mut UART }
    }

    pub fn subscribe(&mut self, observer: Box<&'a mut dyn UartObserver>) {
        self.observer = Some(observer);
        //self.observers.push(observer);
    }

    // 初期化
    pub fn init(&self) {
        // UART0無効化
        regs::write(regs::Register::Uart0Cr, 0x00000000);

        // GPIO pull/upの無効にし、150サイクル待つ
        regs::write(regs::Register::GpioUd, 0x00000000);
        //CPU::delay(150);

        // 14/15pinを無効にし、150サイクル待つ
        regs::write(regs::Register::GpioUdclk0, (1 << 14) | (1 << 15));
        //CPU::delay(150);

        // GPPUDCLK0を0に設定
        regs::write(regs::Register::GpioUdclk0, 0x00000000);

        // Interrupt Clear Register割り込みの無効化
        regs::write(regs::Register::Uart0Icr, 0x7FF);

        // ボーレート(シリアル通信に使う)の整数部・小数部の計算
        //
        // Divider = UART_CLOCK/(16 * Baud)
        // Fraction part register = (Fractional part * 64) + 0.5
        // UART_CLOCK = 3000000; Baud = 115200.
        //
        // Divider = 3000000/(16 * 115200) = 1.627 = ~1.
        // Fractional part register = (.627 * 64) + 0.5 = 40.6 = ~40.
        regs::write(regs::Register::Uart0Ibrd, 1);
        regs::write(regs::Register::Uart0Fbrd, 40);

        // FIFO，8bit通信を有効にする．
        // 4bit目: FIFOを有効に
        // 5,6bit目: 1フレームの通信量 11なら8 bit/フレーム
        regs::write(regs::Register::Uart0Lcrh, (1 << 4) | (1 << 5) | (1 << 6));

        // 全ての割り込みを有効に
        // 1:有効 0:無効
        // 1bit目: uUARTCTS modern 割り込み
        // 4bit目: 受信割り込み
        // 5bit目: 送信割り込み
        // 6bit目: 受信タイムアウト割り込み
        // 7bit目: フレームエラー割り込み
        // 8bit目: パリティエラー割り込み
        // 9bit目: Breakエラー割り込み
        // 10bit目: オーバーランエラー割り込み
        regs::write(regs::Register::Uart0Imsc, UART_MIS_RXMIS); // 受信割り込みのみ
                                                                //(1 << 1) | (1 << 4) | (1 << 5) | (1 << 6) | (1 << 7) | (1 << 8) | (1 << 9) | (1 << 10)

        // 0bit目: UARTを有効に
        // 8bit目: 送信を有効に
        // 9bit目: 受信を有効に
        regs::write(regs::Register::Uart0Cr, (1 << 0) | (1 << 8) | (1 << 9));
    }

    // 割り込み処理
    pub fn interrupt(&self) {
        if UART_MIS_RXMIS == regs::read(regs::Register::Uart0Mis) & UART_MIS_RXMIS {
            // 受信したデータを通知
            let data = self.receive();
            if let Some(cb) = &self.observer {
                cb.notify(data);
            }
            //self.observer.iter().for_each(|o| o.notify(data));
        }
    }

    // 文字列送信
    pub fn send(&self, str: &[u8]) {
        str.iter().for_each(|x| self.send_char(x));
    }

    // 文字送信
    fn send_char(&self, c: &u8) {
        while (1 << 5) == regs::read(regs::Register::Uart0Fr) & (1 << 5) {}
        regs::write(regs::Register::Uart0Dr, *c as u32);
    }

    // 受信処理
    fn receive(&self) -> u32 {
        // データ受信待ち
        while (1 << 4) == regs::read(regs::Register::Uart0Fr) & (1 << 4) {}
        return regs::read(regs::Register::Uart0Dr);
    }

    // 数値送信
    pub fn send_hex(&self, num: u32) {
        let hexdigits = b"0123456789ABCDEF";
        let mut i = 28;
        while i >= 0 {
            self.send_char(&hexdigits[(num >> i) as usize]);
            i -= 4;
        }
    }
}
