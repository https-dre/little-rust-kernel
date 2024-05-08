#[warn(static_mut_refs)]
use x86_64::VirtAddr;
use x86_64::structures::tss::TaskStateSegment;
use lazy_static::lazy_static;
use core::ptr::addr_of;

use x86_64::structures::gdt::{ GlobalDescriptorTable, Descriptor, SegmentSelector };

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;
// TSS = Task State Segment contém informações sobre o estado atual um Processo
// IST = Interrupt Stack Table usada para fornecer pilhas de interrupção separadas
// GDT = Global Descriptor Table

/* 
    GDT
        é uma estrutura de dados em sistemas x86 que define 
        os segmentos de memória utilizados pelo processador
        para acessar diferentes partes da memória do sistema.
        A GDT contém descritores para segmentos de código, segmentos de dados,
        segmentos de pilha e outros tipos de segmentos. Cada entrada na GDT
        define as propriedades de um segmento de memória, como seu tamanho,
        permissões de acesso e tipo. A GDT é uma parte essencial da arquitetura
        de segmentação do x86 e é usada pelo processador para controlar o acesso à memória do sistema.
*/

lazy_static! {
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            const STACK_SIZE: usize = 4096 * 5;
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

            let stack_start = VirtAddr::from_ptr(unsafe { addr_of!(STACK) });
            let stack_end = stack_start + STACK_SIZE;
            stack_end
        };
        tss
    };
}

lazy_static! {
    static ref GDT: (GlobalDescriptorTable, Selectors) = {
        let mut gdt = GlobalDescriptorTable::new();
        let code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
        let tss_selector = gdt.add_entry(Descriptor::tss_segment(&TSS));
        (gdt, Selectors { code_selector, tss_selector })
    };
}

struct Selectors {
    code_selector: SegmentSelector,
    tss_selector: SegmentSelector,
}

pub fn init() {
    use x86_64::instructions::tables::load_tss;
    use x86_64::instructions::segmentation::{ CS, Segment };

    GDT.0.load();
    unsafe {
        CS::set_reg(GDT.1.code_selector); // recarregando segmento de código
        load_tss(GDT.1.tss_selector);
    }
}
