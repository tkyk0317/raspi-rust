// レジスタアドレス
const GPIO_BASE: isize = 0x3F200000;
const UART0_BASE: isize = 0x3F201000;
const IRQ_BASE: isize = 0x3F00B200;

pub enum Register {
    // GPU
    GpuInterruptsRouting = 0x4000000C,

    // GPIO
    GpioUd = GPIO_BASE + 0x94,
    GpioUdclk0 = GPIO_BASE + 0x98,

    // UART0
    Uart0Dr = UART0_BASE + 0x00,
    Uart0Rsrecr = UART0_BASE + 0x04,
    Uart0Fr = UART0_BASE + 0x18,
    Uart0Ilpr = UART0_BASE + 0x20,
    Uart0Ibrd = UART0_BASE + 0x24,
    Uart0Fbrd = UART0_BASE + 0x28,
    Uart0Lcrh = UART0_BASE + 0x2C,
    Uart0Cr = UART0_BASE + 0x30,
    Uart0Ifls = UART0_BASE + 0x34,
    Uart0Imsc = UART0_BASE + 0x38,
    Uart0Ris = UART0_BASE + 0x3C,
    Uart0Mis = UART0_BASE + 0x40,
    Uart0Icr = UART0_BASE + 0x44,
    Uart0Dmacr = UART0_BASE + 0x48,
    Uart0Itcr = UART0_BASE + 0x80,
    Uart0Itip = UART0_BASE + 0x84,
    Uart0Itop = UART0_BASE + 0x88,
    Uart0Tdr = UART0_BASE + 0x8C,

    // CORE0
    Core0TimerInterruptCtl = 0x40000040,
    Core0InterruptSource = 0x40000060,

    // IRQ
    IrqBasic = IRQ_BASE + 0x00,
    IrqPend1 = IRQ_BASE + 0x04,
    IrqPend2 = IRQ_BASE + 0x08,
    IrqFiq = IRQ_BASE + 0x0C,
    IrqEnable1 = IRQ_BASE + 0x10,
    IrqEnable2 = IRQ_BASE + 0x14,
    IrqEnableBasic = IRQ_BASE + 0x18,
    IrqDisable1 = IRQ_BASE + 0x1C,
    IrqDisable2 = IRQ_BASE + 0x20,
    IrqDisableBasic = IRQ_BASE + 0x24,
}

macro_rules! pointer {
    ($reg: ident) => {
        ($reg as u32) as *const u32
    };
}
macro_rules! pointer_mut {
    ($reg: ident) => {
        ($reg as u32) as *mut u32
    };
}

// レジスタリード
pub fn read(reg: Register) -> u32 {
    let p = pointer!(reg);
    unsafe { *p }
}

// レジスタ書き込み
pub fn write(reg: Register, val: u32) {
    let p = pointer_mut!(reg);
    unsafe { *p = val }
}
