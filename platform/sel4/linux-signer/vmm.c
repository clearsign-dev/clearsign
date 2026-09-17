/*
 * Copyright 2023, UNSW
 *
 * SPDX-License-Identifier: BSD-2-Clause
 */
#include <stddef.h>
#include <stdint.h>
#include <stdbool.h>
#include <microkit.h>
#include <libvmm/libvmm.h>

/*
 * As this is just an example, for simplicity we just make the size of the
 * guest's "RAM" the same for all platforms. For just booting Linux with a
 * simple user-space, 0x10000000 bytes (256MB) is plenty.
 */
#define GUEST_RAM_SIZE 0x10000000

/* For ARM, these constants depends on what's defined in your DTB. */
#if defined(BOARD_qemu_virt_aarch64)
#define GUEST_RAM_START_GPA 0x40000000
#define GUEST_DTB_GPA 0x4f000000
#define GUEST_INIT_RAM_DISK_GPA 0x4d000000
#elif defined(BOARD_odroidc4)
#define GUEST_RAM_START_GPA 0x20000000
#define GUEST_DTB_GPA 0x2f000000
#define GUEST_INIT_RAM_DISK_GPA 0x2d000000
#elif defined(BOARD_maaxboard)
#define GUEST_RAM_START_GPA 0x40000000
#define GUEST_DTB_GPA 0x4f000000
#define GUEST_INIT_RAM_DISK_GPA 0x4c000000
#elif defined(BOARD_x86_64_generic_vtx)
#define TIMER_DRV_CH 10
#define GUEST_RAM_START_GPA LOW_RAM_START_GPA
#define GUEST_CMDLINE_HW "earlyprintk=serial,0x3f8,115200 debug console=ttyS0,115200 earlycon=serial,0x3f8,115200 loglevel=8"
#define GUEST_CMDLINE_QEMU "earlyprintk=serial,0x3f8,115200 debug console=ttyS0,115200 earlycon=serial,0x3f8,115200 loglevel=8 tsc=nowatchdog"
#elif defined(BOARD_zcu102)
#define GUEST_RAM_START_GPA     0x800000000
#define GUEST_DTB_GPA           0x80E000000
#define GUEST_INIT_RAM_DISK_GPA 0x80D000000
#else
#error Need to define guest kernel image address and DTB address
#endif

/* For simplicity we just enforce the serial IRQ channel number to be the same
 * across platforms. */
#define SERIAL_IRQ_CH 1

#if defined(BOARD_qemu_virt_aarch64)
#define SERIAL_IRQ 33
#elif defined(BOARD_odroidc4)
#define SERIAL_IRQ 225
#elif defined(BOARD_maaxboard)
#define SERIAL_IRQ 58
#elif defined(BOARD_x86_64_generic_vtx)
#define COM1_IOAPIC_CHIP 0
#define COM1_IOAPIC_PIN 4
#define COM1_IO_PORT_ID 0
#define COM1_IO_PORT_ADDR 0x3F8
#define COM1_IO_PORT_SIZE 8
#elif defined(BOARD_zcu102)
#define SERIAL_IRQ 53
#else
#error Need to define serial interrupt
#endif

/* Data for the guest's kernel image. */
extern char _guest_kernel_image[];
extern char _guest_kernel_image_end[];
/* Data for the initial RAM disk to be passed to the kernel. */
extern char _guest_initrd_image[];
extern char _guest_initrd_image_end[];

#if defined(CONFIG_ARCH_X86_64)
/* Data for the guest's ACPI Differentiated System Description Table (DSDT). */
extern char _guest_dsdt_aml[];
extern char _guest_dsdt_aml_end[];
#else
/* Data for the device tree to be passed to the kernel. */
extern char _guest_dtb_image[];
extern char _guest_dtb_image_end[];
#endif

/* Microkit will set this variable to the start of the guest RAM memory region. */
uintptr_t guest_ram_vaddr;

/* ---- Signer compartment integration ------------------------------------------
 * The guest (Linux, untrusted) writes a request into a shared region, then writes
 * to an unmapped "doorbell" address. That write traps into this VMM, which
 * notifies the signer protection domain. The VMM never reads or edits the request
 * or the review: it only forwards the signal. */
#define SIGNER_CH 2
#define SIGNER_DOORBELL_GPA 0x60200000
#define SIGNER_DOORBELL_SIZE 0x1000

/* ---- The guest's console --------------------------------------------------
 * The guest used to have the serial device mapped into it, which meant the
 * untrusted side could write anything to the same screen the signer reports on,
 * including lines that look exactly like the signer's. A review a person reads
 * from a channel the attacker can write is not a review.
 *
 * The device now belongs to this VMM. The guest's view of it is unmapped, so
 * its writes trap here and are relayed one line at a time behind a prefix it
 * cannot produce. What the signer prints, only the signer can print. */
#define GUEST_UART_GPA 0x9000000
#define GUEST_UART_SIZE 0x1000
#define UART_DR 0x00   /* data register: writes are characters */
#define UART_FR 0x18   /* flag register: reads tell the driver it may send */
#define UART_FR_RXFE (1 << 4) /* receive FIFO empty: always, the guest has no input here */
#define UART_FR_TXFE (1 << 7) /* transmit FIFO empty: always, since we never queue */
/* The AMBA bus identifies a device by eight ID registers at the end of its page.
 * Without them the PL011 driver does not recognise what it is talking to, gives
 * up, and Linux falls back to a console that goes nowhere — which is how this
 * change first showed up: the guest booted in silence. These are the values
 * QEMU's own PL011 reports. */
#define UART_ID_FIRST 0xfe0
static const uint8_t uart_id[8] = { 0x11, 0x10, 0x14, 0x00, 0x0d, 0xf0, 0x05, 0xb1 };
#define GUEST_LINE_MAX 240

static char guest_line[GUEST_LINE_MAX];
static size_t guest_line_len;

static void guest_console_flush(void)
{
    if (guest_line_len == 0) {
        return;
    }
    guest_line[guest_line_len] = 0;
    microkit_dbg_puts("GUEST|");
    microkit_dbg_puts(guest_line);
    microkit_dbg_puts("\n");
    guest_line_len = 0;
}

static void guest_console_putchar(char c)
{
    if (c == '\n' || c == '\r') {
        guest_console_flush();
        return;
    }
    /* Anything that is not plainly printable is shown as a dot rather than sent
     * to the terminal: the guest must not be able to move the cursor, clear the
     * screen, or start a colour sequence on a display the signer shares. */
    char safe = (c >= 0x20 && c < 0x7f) ? c : '.';
    if (guest_line_len + 1 >= GUEST_LINE_MAX) {
        guest_console_flush();
    }
    guest_line[guest_line_len++] = safe;
}

static bool guest_uart(size_t vcpu_id, size_t offset, size_t fsr, seL4_UserContext *regs, void *data)
{
    if (fault_is_write(fsr)) {
        if (offset == UART_DR) {
            guest_console_putchar((char)(fault_get_data(regs, fsr) & 0xff));
        }
        /* Every other register write is accepted and dropped: the guest may
         * configure a UART it does not have. */
        return true;
    }
    /* Reads: a transmitter that is always ready, nothing to receive, and the
     * identification the driver needs to bind at all. Zero elsewhere, so it
     * never waits for a device that is not there. */
    uint64_t value = 0;
    if (offset == UART_FR) {
        value = UART_FR_TXFE | UART_FR_RXFE;
    } else if (offset >= UART_ID_FIRST) {
        /* One register every four bytes, eight of them, ending the page. */
        size_t index = (offset - UART_ID_FIRST) / 4;
        if (index < sizeof(uart_id)) {
            value = uart_id[index];
        }
    }
    return fault_advance(vcpu_id, regs, GUEST_UART_GPA + offset, fsr, value);
}

static bool signer_doorbell(size_t vcpu_id, size_t offset, size_t fsr, seL4_UserContext *regs, void *data)
{
    if (fault_is_write(fsr)) {
        LOG_VMM("guest rang the signer doorbell; notifying signer compartment\n");
        microkit_notify(SIGNER_CH);
    }
    return true; /* ignore reads; always resume the guest past the faulting instruction */
}

void init(void)
{
    /* Initialise the VMM, the VCPU(s), and start the guest */
    LOG_VMM("starting \"%s\"\n", microkit_name);

    arch_guest_init_t args = {
        .pci_init.mmio_aperature_size = 0, /* Disable the virtual PCI bus */
#if defined(CONFIG_ARCH_X86_64)
        .bsp = true,
        .timer_ch = TIMER_DRV_CH,
#elif defined(CONFIG_ARCH_ARM)
        .num_vcpus = 1,
#endif
        .num_guest_ram_regions = 1,
        .guest_ram_regions = { (struct guest_ram_region) {
            .gpa_start = GUEST_RAM_START_GPA, .size = GUEST_RAM_SIZE, .vmm_vaddr = (void *)guest_ram_vaddr } }
    };
    bool success = guest_init(args);
    if (!success) {
        LOG_VMM_ERR("Failed to initialise guest\n");
        return;
    }

    if (!fault_register_vm_exception_handler(SIGNER_DOORBELL_GPA, SIGNER_DOORBELL_SIZE, signer_doorbell, NULL)) {
        LOG_VMM_ERR("Failed to register signer doorbell\n");
        return;
    }
    if (!fault_register_vm_exception_handler(GUEST_UART_GPA, GUEST_UART_SIZE, guest_uart, NULL)) {
        LOG_VMM_ERR("Failed to take the serial device away from the guest\n");
        return;
    }

    /* Place all the binaries in the right locations before starting the guest */
    size_t kernel_size = _guest_kernel_image_end - _guest_kernel_image;
    size_t initrd_size = _guest_initrd_image_end - _guest_initrd_image;

    if (!kernel_size) {
        LOG_VMM_ERR("Kernel image is empty\n");
        return;
    }
    if (!initrd_size) {
        LOG_VMM_ERR("Initial ramdisk image is empty\n");
        return;
    }

#if defined(CONFIG_ARCH_X86_64)
    size_t dsdt_aml_size = _guest_dsdt_aml_end - _guest_dsdt_aml;

    if (!dsdt_aml_size) {
        LOG_VMM_ERR("DSDT AML image is empty\n");
        return;
    }

    seL4_VCPUContext initial_regs;
    linux_x86_setup_ret_t linux_setup;
    char *cmdline;
    /* Reading a TSC does not cause a VM Exit, but the HPET does. On QEMU, the nested virtualisation overhead
     * is too high, leading to Linux's watchdog timer complaining:
     * [    3.568257] clocksource: Watchdog hpet read timed out. Readout sequence took: 57800ns
     * [    4.064535] clocksource: Watchdog hpet read timed out. Readout sequence took: 62000ns
     * [    9.064695] watchdog_print_freq_timeout: 7 callbacks suppressed
     * [    9.066802] clocksource: Watchdog hpet read timed out. Readout sequence took: 217500ns
     * [    9.568591] clocksource: Watchdog hpet read timed out. Readout sequence took: 72900ns
     *
     * So lets turn off the TSC watchdog if the VMM detects that it is running on QEMU. This is sound because
     * the virtual HPET time is derived from the TSC, and this problem doesn't manifest on hardware.
     */
    if (hypervisor_present()) {
        cmdline = GUEST_CMDLINE_QEMU;
    } else {
        cmdline = GUEST_CMDLINE_HW;
    }

    if (!linux_setup_images((uintptr_t)_guest_kernel_image, kernel_size, (uintptr_t)_guest_initrd_image, initrd_size,
                            _guest_dsdt_aml, dsdt_aml_size, cmdline, &initial_regs, &linux_setup)) {
        LOG_VMM_ERR("Failed to initialise guest images\n");
        return;
    }

    /* Pass through COM1 serial port */
    microkit_vcpu_x86_enable_ioport(GUEST_BOOT_VCPU_ID, COM1_IO_PORT_ID, COM1_IO_PORT_ADDR, COM1_IO_PORT_SIZE);
    microkit_irq_ack(SERIAL_IRQ_CH);

    /* Pass through serial IRQs */
    assert(virq_register_passthrough(X86_IOAPIC_IRQ_ROUTE(COM1_IOAPIC_CHIP, COM1_IOAPIC_PIN), SERIAL_IRQ_CH));

    guest_start_long_mode(linux_setup.kernel_entry_gpa, linux_setup.pml4_gpa, linux_setup.gdt_gpa,
                          linux_setup.gdt_limit, &initial_regs);
#else
    size_t dtb_size = _guest_dtb_image_end - _guest_dtb_image;
    if (!dtb_size) {
        LOG_VMM_ERR("DTB image is empty\n");
        return;
    }

    uintptr_t kernel_pc = linux_setup_images(GUEST_RAM_START_GPA, (uintptr_t)_guest_kernel_image, kernel_size,
                                             (uintptr_t)_guest_dtb_image, GUEST_DTB_GPA, dtb_size,
                                             (uintptr_t)_guest_initrd_image, GUEST_INIT_RAM_DISK_GPA, initrd_size);
    if (!kernel_pc) {
        LOG_VMM_ERR("Failed to initialise guest images\n");
        return;
    }

#if defined(BOARD_zcu102)
    /* Initialise the SMC SIP Handler */
    success = smc_register_sip_handler(smc_sip_forward);
    if (!success) {
        LOG_VMM_ERR("Failed to initialise SMC SIP Handler\n");
        return;
    }
#endif

    success = virq_register_passthrough(ARM_GIC_IRQ_ROUTE(GUEST_BOOT_VCPU_ID, SERIAL_IRQ), SERIAL_IRQ_CH);
    assert(success);
    /* Finally start the guest */
    guest_start(kernel_pc, GUEST_DTB_GPA, GUEST_INIT_RAM_DISK_GPA);
#endif
}

void notified(microkit_channel ch)
{
    switch (ch) {
    case SERIAL_IRQ_CH: {
        bool success = virq_handle_passthrough(ch);
        if (!success) {
            LOG_VMM_ERR("Serial IRQ dropped\n");
        }
        break;
    }
    case SIGNER_CH:
        LOG_VMM("signer compartment finished a review\n");
        break;
    default:
        printf("Unexpected channel, ch: 0x%x\n", ch);
    }
}

/*
 * The primary purpose of the VMM after initialisation is to act as a fault-handler.
 * Whenever our guest causes an exception, it gets delivered to this entry point for
 * the VMM to handle.
 */
seL4_Bool fault(microkit_child child, microkit_msginfo msginfo, microkit_msginfo *reply_msginfo)
{
    bool success = fault_handle(child, msginfo);
    if (success) {
        /* Now that we have handled the fault successfully, we reply to it so
         * that the guest can resume execution. */
        *reply_msginfo = microkit_msginfo_new(0, 0);
        return seL4_True;
    }

    return seL4_False;
}
