use super::*;
use crate::binary::{
    c_types,
    include::{
        esp_bt_controller_config_t, esp_bt_mode_t, esp_bt_mode_t_ESP_BT_MODE_BLE,
        esp_bt_mode_t_ESP_BT_MODE_BTDM, esp_bt_mode_t_ESP_BT_MODE_CLASSIC_BT,
        esp_bt_mode_t_ESP_BT_MODE_IDLE,
    },
};

pub static mut ISR_INTERRUPT_5: (
    *mut crate::binary::c_types::c_void,
    *mut crate::binary::c_types::c_void,
) = (core::ptr::null_mut(), core::ptr::null_mut());

pub static mut ISR_INTERRUPT_8: (
    *mut crate::binary::c_types::c_void,
    *mut crate::binary::c_types::c_void,
) = (core::ptr::null_mut(), core::ptr::null_mut());

pub static mut ISR_INTERRUPT_7: (
    *mut crate::binary::c_types::c_void,
    *mut crate::binary::c_types::c_void,
) = (core::ptr::null_mut(), core::ptr::null_mut());

#[repr(C)]
pub(super) struct osi_funcs_s {
    version: u32,
    set_isr: Option<unsafe extern "C" fn(i32, unsafe extern "C" fn(), *const ()) -> i32>,
    ints_on: Option<unsafe extern "C" fn(u32)>,
    interrupt_disable: Option<unsafe extern "C" fn() -> ()>,
    interrupt_restore: Option<unsafe extern "C" fn() -> ()>,
    task_yield: Option<unsafe extern "C" fn() -> ()>,
    task_yield_from_isr: Option<unsafe extern "C" fn() -> ()>,
    semphr_create: Option<unsafe extern "C" fn(u32, u32) -> *const ()>,
    semphr_delete: Option<unsafe extern "C" fn(*const ()) -> ()>,
    semphr_take_from_isr: Option<unsafe extern "C" fn(*const (), *const ()) -> i32>,
    semphr_give_from_isr: Option<unsafe extern "C" fn(*const (), *const ()) -> i32>,
    semphr_take: Option<unsafe extern "C" fn(*const (), u32) -> i32>,
    semphr_give: Option<unsafe extern "C" fn(*const ()) -> i32>,
    mutex_create: Option<unsafe extern "C" fn() -> *const ()>,
    mutex_delete: Option<unsafe extern "C" fn(*const ()) -> ()>,
    mutex_lock: Option<unsafe extern "C" fn(*const ()) -> i32>,
    mutex_unlock: Option<unsafe extern "C" fn(*const ()) -> i32>,
    queue_create: Option<unsafe extern "C" fn(u32, u32) -> *const ()>,
    queue_delete: Option<unsafe extern "C" fn(*const ()) -> ()>,
    queue_send: Option<unsafe extern "C" fn(*const (), *const (), u32) -> i32>,
    queue_send_from_isr: Option<unsafe extern "C" fn(*const (), *const (), *const ()) -> i32>,
    queue_recv: Option<unsafe extern "C" fn(*const (), *const (), u32) -> i32>,
    queue_recv_from_isr: Option<unsafe extern "C" fn(*const (), *const (), *const ()) -> i32>,
    task_create: Option<
        unsafe extern "C" fn(
            *mut crate::binary::c_types::c_void,
            *const u8,
            u32,
            *mut crate::binary::c_types::c_void,
            u32,
            *mut crate::binary::c_types::c_void,
            u32,
        ) -> i32,
    >,
    task_delete: Option<unsafe extern "C" fn(*const ()) -> ()>,
    is_in_isr: Option<unsafe extern "C" fn() -> i32>,
    cause_sw_intr_to_core: Option<unsafe extern "C" fn(i32, i32) -> i32>,
    malloc: Option<unsafe extern "C" fn(u32) -> *const ()>,
    malloc_internal: Option<unsafe extern "C" fn(u32) -> *const ()>,
    free: Option<unsafe extern "C" fn(*const ()) -> ()>,
    read_efuse_mac: Option<unsafe extern "C" fn(*const ()) -> i32>,
    srand: Option<unsafe extern "C" fn(u32) -> ()>,
    rand: Option<unsafe extern "C" fn() -> i32>,
    btdm_lpcycles_2_hus: Option<unsafe extern "C" fn(u32, u32) -> u32>,
    btdm_hus_2_lpcycles: Option<unsafe extern "C" fn(u32) -> u32>,
    btdm_sleep_check_duration: Option<unsafe extern "C" fn(i32) -> i32>,
    btdm_sleep_enter_phase1: Option<unsafe extern "C" fn(i32) -> ()>,
    btdm_sleep_enter_phase2: Option<unsafe extern "C" fn() -> ()>,
    btdm_sleep_exit_phase1: Option<unsafe extern "C" fn() -> ()>,
    btdm_sleep_exit_phase2: Option<unsafe extern "C" fn() -> ()>,
    btdm_sleep_exit_phase3: Option<unsafe extern "C" fn() -> ()>,
    coex_bt_wakeup_request: Option<unsafe extern "C" fn() -> bool>,
    coex_bt_wakeup_request_end: Option<unsafe extern "C" fn() -> ()>,
    coex_bt_request: Option<unsafe extern "C" fn(u32, u32, u32) -> i32>,
    coex_bt_release: Option<unsafe extern "C" fn(u32) -> i32>,
    coex_register_bt_cb: Option<unsafe extern "C" fn(unsafe extern "C" fn()) -> i32>,
    coex_bb_reset_lock: Option<unsafe extern "C" fn() -> u32>,
    coex_bb_reset_unlock: Option<unsafe extern "C" fn(u32)>,
    coex_schm_register_btdm_callback: Option<unsafe extern "C" fn(unsafe extern "C" fn()) -> i32>,
    coex_schm_status_bit_clear: Option<unsafe extern "C" fn(i32, i32) -> ()>,
    coex_schm_status_bit_set: Option<unsafe extern "C" fn(i32, i32) -> ()>,
    coex_schm_interval_get: Option<unsafe extern "C" fn() -> u32>,
    coex_schm_curr_period_get: Option<unsafe extern "C" fn() -> u8>,
    coex_schm_curr_phase_get: Option<unsafe extern "C" fn() -> *const ()>,
    coex_wifi_channel_get: Option<unsafe extern "C" fn(*mut u8, *mut u8) -> i32>,
    coex_register_wifi_channel_change_callback:
        Option<unsafe extern "C" fn(unsafe extern "C" fn()) -> i32>,
    set_isr13: Option<unsafe extern "C" fn(i32, unsafe extern "C" fn(), *const ()) -> i32>,
    interrupt_l3_disable: Option<unsafe extern "C" fn() -> ()>,
    interrupt_l3_restore: Option<unsafe extern "C" fn() -> ()>,
    custom_queue_create:
        Option<unsafe extern "C" fn(u32, u32) -> *mut crate::binary::c_types::c_void>,
    magic: u32,
}

pub(super) static G_OSI_FUNCS: osi_funcs_s = osi_funcs_s {
    version: 0x00010003,
    set_isr: Some(ble_os_adapter_chip_specific::set_isr),
    ints_on: Some(ble_os_adapter_chip_specific::ints_on),
    interrupt_disable: Some(interrupt_disable),
    interrupt_restore: Some(interrupt_enable),
    task_yield: Some(task_yield),
    task_yield_from_isr: Some(task_yield_from_isr),
    semphr_create: Some(semphr_create),
    semphr_delete: Some(semphr_delete),
    semphr_take_from_isr: Some(crate::common_adapter::semphr_take_from_isr),
    semphr_give_from_isr: Some(crate::common_adapter::semphr_give_from_isr),
    semphr_take: Some(semphr_take),
    semphr_give: Some(semphr_give),
    mutex_create: Some(mutex_create),
    mutex_delete: Some(mutex_delete),
    mutex_lock: Some(mutex_lock),
    mutex_unlock: Some(mutex_unlock),
    queue_create: Some(queue_create),
    queue_delete: Some(queue_delete),
    queue_send: Some(queue_send),
    queue_send_from_isr: Some(queue_send_from_isr),
    queue_recv: Some(queue_recv),
    queue_recv_from_isr: Some(queue_recv_from_isr),
    task_create: Some(task_create),
    task_delete: Some(task_delete),
    is_in_isr: Some(is_in_isr),
    cause_sw_intr_to_core: Some(cause_sw_intr_to_core),
    malloc: Some(malloc),
    malloc_internal: Some(malloc_internal),
    free: Some(free),
    read_efuse_mac: Some(read_efuse_mac),
    srand: Some(crate::ble::btdm::srand),
    rand: Some(crate::ble::btdm::rand),
    btdm_lpcycles_2_hus: Some(btdm_lpcycles_2_hus),
    btdm_hus_2_lpcycles: Some(btdm_hus_2_lpcycles),
    btdm_sleep_check_duration: Some(btdm_sleep_check_duration),
    btdm_sleep_enter_phase1: Some(btdm_sleep_enter_phase1),
    btdm_sleep_enter_phase2: Some(btdm_sleep_enter_phase2),
    btdm_sleep_exit_phase1: Some(btdm_sleep_exit_phase1),
    btdm_sleep_exit_phase2: Some(btdm_sleep_exit_phase2),
    btdm_sleep_exit_phase3: Some(btdm_sleep_exit_phase3),
    coex_bt_wakeup_request: Some(ble_os_adapter_chip_specific::coex_bt_wakeup_request),
    coex_bt_wakeup_request_end: Some(ble_os_adapter_chip_specific::coex_bt_wakeup_request_end),
    coex_bt_request: Some(ble_os_adapter_chip_specific::coex_bt_request),
    coex_bt_release: Some(ble_os_adapter_chip_specific::coex_bt_release),
    coex_register_bt_cb: Some(ble_os_adapter_chip_specific::coex_register_bt_cb_wrapper),
    coex_bb_reset_lock: Some(ble_os_adapter_chip_specific::coex_bb_reset_lock),
    coex_bb_reset_unlock: Some(ble_os_adapter_chip_specific::coex_bb_reset_unlock),
    coex_schm_register_btdm_callback: Some(
        ble_os_adapter_chip_specific::coex_schm_register_btdm_callback_wrapper,
    ),
    coex_schm_status_bit_clear: Some(coex_schm_status_bit_clear),
    coex_schm_status_bit_set: Some(coex_schm_status_bit_set),
    coex_schm_interval_get: Some(ble_os_adapter_chip_specific::coex_schm_interval_get),
    coex_schm_curr_period_get: Some(ble_os_adapter_chip_specific::coex_schm_curr_period_get),
    coex_schm_curr_phase_get: Some(ble_os_adapter_chip_specific::coex_schm_curr_phase_get),
    coex_wifi_channel_get: Some(ble_os_adapter_chip_specific::coex_wifi_channel_get),
    coex_register_wifi_channel_change_callback: Some(
        ble_os_adapter_chip_specific::coex_register_wifi_channel_change_callback,
    ),
    set_isr13: Some(set_isr13),
    interrupt_l3_disable: Some(interrupt_l3_disable),
    interrupt_l3_restore: Some(interrupt_l3_restore),
    custom_queue_create: Some(custom_queue_create),
    magic: 0xfadebead,
};

#[repr(C)]
struct btdm_dram_available_region_t {
    mode: esp_bt_mode_t,
    start: u32, // ptr
    end: u32,   // ptr
}

const SOC_MEM_BT_DATA_START: u32 = 0x3ffae6e0;
const SOC_MEM_BT_DATA_END: u32 = 0x3ffaff10;
const SOC_MEM_BT_EM_BTDM0_START: u32 = 0x3ffb0000;
const SOC_MEM_BT_EM_BTDM0_END: u32 = 0x3ffb09a8;
const SOC_MEM_BT_EM_BLE_START: u32 = 0x3ffb09a8;
const SOC_MEM_BT_EM_BLE_END: u32 = 0x3ffb1ddc;
const SOC_MEM_BT_EM_BTDM1_START: u32 = 0x3ffb1ddc;
const SOC_MEM_BT_EM_BTDM1_END: u32 = 0x3ffb2730;
const SOC_MEM_BT_EM_BREDR_START: u32 = 0x3ffb2730;
const SOC_MEM_BT_BSS_START: u32 = 0x3ffb8000;
const SOC_MEM_BT_BSS_END: u32 = 0x3ffb9a20;
const SOC_MEM_BT_MISC_START: u32 = 0x3ffbdb28;
const SOC_MEM_BT_MISC_END: u32 = 0x3ffbdb5c;

const SOC_MEM_BT_EM_BREDR_REAL_END: u32 = 0x3ffb6388; //  (SOC_MEM_BT_EM_BREDR_NO_SYNC_END + CONFIG_BTDM_CTRL_BR_EDR_MAX_SYNC_CONN_EFF * SOC_MEM_BT_EM_PER_SYNC_SIZE);

static BTDM_DRAM_AVAILABLE_REGION: [btdm_dram_available_region_t; 7] = [
    /* following is .data */
    btdm_dram_available_region_t {
        mode: esp_bt_mode_t_ESP_BT_MODE_BTDM,
        start: SOC_MEM_BT_DATA_START,
        end: SOC_MEM_BT_DATA_END,
    },
    /* following is memory which HW will use */
    btdm_dram_available_region_t {
        mode: esp_bt_mode_t_ESP_BT_MODE_BTDM,
        start: SOC_MEM_BT_EM_BTDM0_START,
        end: SOC_MEM_BT_EM_BTDM0_END,
    },
    btdm_dram_available_region_t {
        mode: esp_bt_mode_t_ESP_BT_MODE_BLE,
        start: SOC_MEM_BT_EM_BLE_START,
        end: SOC_MEM_BT_EM_BLE_END,
    },
    btdm_dram_available_region_t {
        mode: esp_bt_mode_t_ESP_BT_MODE_BTDM,
        start: SOC_MEM_BT_EM_BTDM1_START,
        end: SOC_MEM_BT_EM_BTDM1_END,
    },
    btdm_dram_available_region_t {
        mode: esp_bt_mode_t_ESP_BT_MODE_CLASSIC_BT,
        start: SOC_MEM_BT_EM_BREDR_START,
        end: SOC_MEM_BT_EM_BREDR_REAL_END,
    },
    /* following is .bss */
    btdm_dram_available_region_t {
        mode: esp_bt_mode_t_ESP_BT_MODE_BTDM,
        start: SOC_MEM_BT_BSS_START,
        end: SOC_MEM_BT_BSS_END,
    },
    btdm_dram_available_region_t {
        mode: esp_bt_mode_t_ESP_BT_MODE_BTDM,
        start: SOC_MEM_BT_MISC_START,
        end: SOC_MEM_BT_MISC_END,
    },
];

pub(crate) fn create_ble_config() -> esp_bt_controller_config_t {
    esp_bt_controller_config_t {
        controller_task_stack_size: 4096,
        controller_task_prio: 110,
        hci_uart_no: 1,
        hci_uart_baudrate: 921600,
        scan_duplicate_mode: 0,
        scan_duplicate_type: 0,
        normal_adv_size: 200,
        mesh_adv_size: 0,
        send_adv_reserved_size: 1000,
        controller_debug_flag: 0 << 0,
        mode: 0x01, // BLE
        ble_max_conn: 3,
        bt_max_acl_conn: 0,
        bt_sco_datapath: 0,
        auto_latency: false,
        bt_legacy_auth_vs_evt: false,
        bt_max_sync_conn: 1,
        ble_sca: 1,
        pcm_role: 0,
        pcm_polar: 0,
        hli: false,
        dup_list_refresh_period: 0,
        magic: 0x20221207,
    }
}

pub(crate) fn btdm_controller_mem_init() {
    extern "C" {
        static _data_start_btdm: u32;
        static _data_start_btdm_rom: u32;
        static _data_end_btdm: u32;
    }

    // initialise .data section
    unsafe {
        let len = (&_data_end_btdm as *const _ as *const u8 as usize)
            - (&_data_start_btdm as *const _ as *const u8 as usize);

        core::ptr::copy_nonoverlapping(
            _data_start_btdm_rom as *const u8,
            &_data_start_btdm as *const _ as *mut u8,
            len,
        );

        log::debug!(
            "btdm_controller_mem_init {:p} {:p} {}",
            _data_start_btdm_rom as *const u8,
            &_data_start_btdm as *const _ as *mut u8,
            len,
        );
    }

    // initialize em, .bss section
    let btdm_dram_regions = BTDM_DRAM_AVAILABLE_REGION.len();

    for i in 1..btdm_dram_regions {
        if BTDM_DRAM_AVAILABLE_REGION[i].mode != esp_bt_mode_t_ESP_BT_MODE_IDLE {
            unsafe {
                core::ptr::write_bytes(
                    BTDM_DRAM_AVAILABLE_REGION[i].start as *mut u8,
                    0x0,
                    (BTDM_DRAM_AVAILABLE_REGION[i].end - BTDM_DRAM_AVAILABLE_REGION[i].start)
                        as usize,
                );
            }
            log::debug!(
                ".bss initialise {:x} - {:x}\n",
                BTDM_DRAM_AVAILABLE_REGION[i].start,
                BTDM_DRAM_AVAILABLE_REGION[i].end
            );
        }
    }

    log::debug!("btdm_controller_mem_init done");
}

pub(crate) fn bt_periph_module_enable() {
    unsafe {
        const DR_REG_DPORT_BASE: u32 = 0x3ff00000;
        const DPORT_WIFI_CLK_EN_REG: u32 = DR_REG_DPORT_BASE + 0x0CC;
        const DPORT_CORE_RST_EN_REG: u32 = DR_REG_DPORT_BASE + 0x0D0;
        const DPORT_WIFI_CLK_BT_EN: u32 = 0x30800;

        let ptr = DPORT_WIFI_CLK_EN_REG as *mut u32;
        let old = ptr.read_volatile();
        ptr.write_volatile(old | DPORT_WIFI_CLK_BT_EN);

        let ptr = DPORT_CORE_RST_EN_REG as *mut u32;
        let old = ptr.read_volatile();
        ptr.write_volatile(old | 0);
    }
    // bt_periph_module_enable(PERIPH_BT_MODULE);
    // modifyreg32(get_clk_en_reg(periph), 0, get_clk_en_mask(periph));
    // modifyreg32(get_rst_en_reg(periph), get_rst_en_mask(periph, true), 0);
}

pub(crate) fn disable_sleep_mode() {
    extern "C" {
        fn btdm_controller_set_sleep_mode(mode: u8);
    }

    const BTDM_MODEM_SLEEP_MODE_NONE: u8 = 0;

    unsafe {
        btdm_controller_set_sleep_mode(BTDM_MODEM_SLEEP_MODE_NONE);
    }
}

pub(crate) unsafe extern "C" fn coex_bt_wakeup_request() -> bool {
    log::debug!("coex_bt_wakeup_request");
    #[cfg(coex)]
    return async_wakeup_request(BTDM_ASYNC_WAKEUP_REQ_COEX);

    #[cfg(not(coex))]
    true
}

pub(crate) unsafe extern "C" fn coex_bt_wakeup_request_end() {
    log::warn!("coex_bt_wakeup_request_end");

    #[cfg(coex)]
    async_wakeup_request_end(BTDM_ASYNC_WAKEUP_REQ_COEX);
}

#[allow(unused_variables)]
pub(crate) unsafe extern "C" fn coex_bt_request(event: u32, latency: u32, duration: u32) -> i32 {
    log::debug!("coex_bt_request");
    extern "C" {
        #[cfg(coex)]
        fn coex_bt_request(event: u32, latency: u32, duration: u32) -> i32;
    }

    #[cfg(coex)]
    return coex_bt_request(event, latency, duration);

    #[cfg(not(coex))]
    0
}

#[allow(unused_variables)]
pub(crate) unsafe extern "C" fn coex_bt_release(event: u32) -> i32 {
    log::debug!("coex_bt_release");
    extern "C" {
        #[cfg(coex)]
        fn coex_bt_release(event: u32) -> i32;
    }

    #[cfg(coex)]
    return coex_bt_release(event);

    #[cfg(not(coex))]
    0
}

pub(crate) unsafe extern "C" fn coex_register_bt_cb_wrapper(
    callback: unsafe extern "C" fn(),
) -> i32 {
    log::warn!("coex_register_bt_cb {:p}", callback);
    extern "C" {
        #[cfg(coex)]
        fn coex_register_bt_cb(callback: unsafe extern "C" fn()) -> i32;
    }

    #[cfg(coex)]
    return coex_register_bt_cb(callback);

    #[cfg(not(coex))]
    0
}

pub(crate) unsafe extern "C" fn coex_bb_reset_lock() -> u32 {
    log::debug!("coex_bb_reset_lock");
    extern "C" {
        #[cfg(coex)]
        fn coex_bb_reset_lock() -> u32;
    }

    #[cfg(coex)]
    return coex_bb_reset_lock();

    #[cfg(not(coex))]
    0
}

#[allow(unused_variables)]
pub(crate) unsafe extern "C" fn coex_bb_reset_unlock(event: u32) {
    log::debug!("coex_bb_reset_unlock");
    extern "C" {
        #[cfg(coex)]
        fn coex_bb_reset_unlock(event: u32);
    }

    #[cfg(coex)]
    coex_bb_reset_unlock(event);
}

pub(crate) unsafe extern "C" fn coex_schm_register_btdm_callback_wrapper(
    callback: unsafe extern "C" fn(),
) -> i32 {
    log::warn!("coex_schm_register_btdm_callback {:p}", callback);
    extern "C" {
        #[cfg(coex)]
        fn coex_schm_register_btdm_callback(callback: unsafe extern "C" fn()) -> i32;
    }

    #[cfg(coex)]
    return coex_schm_register_btdm_callback(callback);

    #[cfg(not(coex))]
    0
}

pub(crate) unsafe extern "C" fn coex_schm_interval_get() -> u32 {
    log::debug!("coex_schm_interval_get");

    #[cfg(coex)]
    return crate::binary::include::coex_schm_interval_get();

    #[cfg(not(coex))]
    0
}

pub(crate) unsafe extern "C" fn coex_schm_curr_period_get() -> u8 {
    log::debug!("coex_schm_curr_period_get");
    // BEWARE: One might expect to call coex_schm_curr_period_get
    //crate::binary::include::coex_schm_curr_period_get()

    #[cfg(coex)]
    return crate::binary::include::coex_schm_interval_get() as u8;

    #[cfg(not(coex))]
    0
}

pub(crate) unsafe extern "C" fn coex_schm_curr_phase_get() -> *const () {
    log::debug!("coex_schm_curr_phase_get");

    #[cfg(coex)]
    return crate::binary::include::coex_schm_curr_phase_get() as *const ();

    #[cfg(not(coex))]
    return 0 as *const ();
}

#[allow(unused_variables)]
pub(crate) unsafe extern "C" fn coex_wifi_channel_get(primary: *mut u8, secondary: *mut u8) -> i32 {
    log::warn!("coex_wifi_channel_get");
    extern "C" {
        #[cfg(coex)]
        fn coex_wifi_channel_get(primary: *mut u8, secondary: *mut u8) -> i32;
    }

    #[cfg(coex)]
    return coex_wifi_channel_get(primary, secondary);

    #[cfg(not(coex))]
    -1
}

#[allow(unused_variables)]
pub(crate) unsafe extern "C" fn coex_register_wifi_channel_change_callback(
    callback: unsafe extern "C" fn(),
) -> i32 {
    log::warn!("coex_register_wifi_channel_change_callback");
    extern "C" {
        #[cfg(coex)]
        fn coex_register_wifi_channel_change_callback(callback: unsafe extern "C" fn()) -> i32;
    }

    #[cfg(coex)]
    return coex_register_wifi_channel_change_callback(callback);

    #[cfg(not(coex))]
    0
}

pub(crate) unsafe extern "C" fn set_isr(n: i32, f: unsafe extern "C" fn(), arg: *const ()) -> i32 {
    log::trace!("set_isr called {} {:p} {:p}", n, f, arg);

    match n {
        5 => {
            ISR_INTERRUPT_5 = (f as *mut c_types::c_void, arg as *mut c_types::c_void);
        }
        7 => {
            ISR_INTERRUPT_7 = (f as *mut c_types::c_void, arg as *mut c_types::c_void);
        }
        8 => {
            ISR_INTERRUPT_8 = (f as *mut c_types::c_void, arg as *mut c_types::c_void);
        }
        _ => panic!("set_isr - unsupported interrupt number {}", n),
    }

    0
}

pub(crate) unsafe extern "C" fn ints_on(mask: u32) {
    log::trace!("chip_ints_on esp32 {:b}", mask);
    xtensa_lx::interrupt::enable_mask(mask);
}

#[cfg(coex)]
const BTDM_ASYNC_WAKEUP_REQ_HCI: i32 = 0;

#[cfg(coex)]
const BTDM_ASYNC_WAKEUP_REQ_COEX: i32 = 1;

//const BTDM_ASYNC_WAKEUP_REQMAX: i32 = 2;

/****************************************************************************
 * Name: async_wakeup_request
 *
 * Description:
 *   Request the BLE Controller to wakeup
 *
 * Input Parameters:
 *   event - the event that triggered the wakeup
 *
 * Returned Value:
 *   true if request lock is needed, false otherwise
 *
 ****************************************************************************/

#[cfg(coex)]
fn async_wakeup_request(event: i32) -> bool {
    let request_lock: bool;
    let mut do_wakeup_request = false;

    match event {
        BTDM_ASYNC_WAKEUP_REQ_HCI => {
            request_lock = true;
        }
        BTDM_ASYNC_WAKEUP_REQ_COEX => {
            request_lock = false;
        }
        _ => {
            return false;
        }
    }

    extern "C" {
        fn btdm_power_state_active() -> bool;
        fn btdm_wakeup_request(request_lock: bool);
    }

    if !unsafe { btdm_power_state_active() } {
        do_wakeup_request = true;
        unsafe { btdm_wakeup_request(request_lock) };
    }

    return do_wakeup_request;
}

/****************************************************************************
 * Name: async_wakeup_request_end
 *
 * Description:
 *   Finish a wakeup request
 *
 * Input Parameters:
 *   event - the event that triggered the wakeup
 *
 * Returned Value:
 *   true if request lock is needed, false otherwise
 *
 ****************************************************************************/

#[cfg(coex)]
fn async_wakeup_request_end(event: i32) {
    let request_lock: bool;

    match event {
        BTDM_ASYNC_WAKEUP_REQ_HCI => {
            request_lock = true;
        }
        BTDM_ASYNC_WAKEUP_REQ_COEX => {
            request_lock = false;
        }
        _ => {
            return;
        }
    }

    extern "C" {
        // this isn't found anywhere ... not a ROM function
        // not in any of the libs - but the code will never call this anyway

        // fn btdm_wakeup_request_end();
    }
    if request_lock {
        // unsafe { btdm_wakeup_request_end() };
    }

    return;
}
