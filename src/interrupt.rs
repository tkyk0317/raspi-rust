use crate::dev::regs;
use crate::dev::uart::Uart;

const CORE0_IRQ_GPU_INTERRUPT: u32 = 1 << 8; // GPU割り込み
const CORE0_TIMER_IRQ_ENABLE: u32 = 1 << 3; // IRQタイマー割り込み
const CORE0_IRQ_TIMER_INTERRUPT: u32 = 1 << 3; // IRQタイマー割り込み
const IRQ_BASIC_PENDING2: u32 = 1 << 9; // PENDING2割り込み
const IRQ_PENDING2_UART: u32 = 1 << 25; // UART割り込み
const IRQ_DISABLE2_UART: u32 = 1 << 25; // UART割り込み
const IRQ_ENABLE2_UART: u32 = 1 << 25; // UART割り込み

// アセンブラ
extern "C" {
    fn enable_irq();
}

// タイマー周期設定
unsafe fn timer_freq() {
    let mut freq = 0;
    llvm_asm!("mrs $0, cntfrq_el0" :"=r"(freq) ::: "volatile");
    llvm_asm!("msr cntv_tval_el0, $0" :: "r" (freq / 10000) :: "volatile");
    regs::write(regs::Register::Core0TimerInterruptCtl, CORE0_TIMER_IRQ_ENABLE);
}

pub fn init() {
    // UART割り込み有効
    regs::write(regs::Register::IrqEnable2, IRQ_ENABLE2_UART);

    // Core0につなぐ
    regs::write(regs::Register::GpuInterruptsRouting, 0x0);

    // UART初期化
    Uart::get_instance().init();

    // システムタイマー設定
    unsafe {
        timer_freq();
        llvm_asm!("msr cntv_ctl_el0, $0" :: "r" (1) :: "volatile");
    }

    // 割り込み有効化
    unsafe {
        enable_irq();
    }
}

#[no_mangle]
pub extern "C" fn __irq_handler() {
    // GPU割り込みチェック
    if CORE0_IRQ_GPU_INTERRUPT
        == regs::read(regs::Register::Core0InterruptSource) & CORE0_IRQ_GPU_INTERRUPT
    {
        // UART0の受信割り込みチェック
        if IRQ_BASIC_PENDING2 == regs::read(regs::Register::IrqBasic) & IRQ_BASIC_PENDING2 {
            if IRQ_PENDING2_UART == regs::read(regs::Register::IrqPend2) & IRQ_PENDING2_UART {
                // UART割り込みを無効化
                regs::write(regs::Register::IrqDisable2, IRQ_DISABLE2_UART);

                // UART割り込み
                Uart::get_instance().interrupt();

                // 割り込み有効化
                regs::write(regs::Register::IrqEnable2, IRQ_ENABLE2_UART);
            }
        }
    }
    // Core0 Timer割り込みチェック
    if CORE0_IRQ_TIMER_INTERRUPT == regs::read(regs::Register::Core0InterruptSource) & CORE0_IRQ_TIMER_INTERRUPT {
        unsafe { timer_freq(); }
    }
}
