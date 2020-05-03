/* automatically generated by rust-bindgen */

pub type timeval = libc::timeval;

impl ftdi_chip_type {
    pub const TYPE_AM: ftdi_chip_type = ftdi_chip_type(0);
}
impl ftdi_chip_type {
    pub const TYPE_BM: ftdi_chip_type = ftdi_chip_type(1);
}
impl ftdi_chip_type {
    pub const TYPE_2232C: ftdi_chip_type = ftdi_chip_type(2);
}
impl ftdi_chip_type {
    pub const TYPE_R: ftdi_chip_type = ftdi_chip_type(3);
}
impl ftdi_chip_type {
    pub const TYPE_2232H: ftdi_chip_type = ftdi_chip_type(4);
}
impl ftdi_chip_type {
    pub const TYPE_4232H: ftdi_chip_type = ftdi_chip_type(5);
}
impl ftdi_chip_type {
    pub const TYPE_232H: ftdi_chip_type = ftdi_chip_type(6);
}
impl ftdi_chip_type {
    pub const TYPE_230X: ftdi_chip_type = ftdi_chip_type(7);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ftdi_chip_type(pub u32);
impl ftdi_parity_type {
    pub const NONE: ftdi_parity_type = ftdi_parity_type(0);
}
impl ftdi_parity_type {
    pub const ODD: ftdi_parity_type = ftdi_parity_type(1);
}
impl ftdi_parity_type {
    pub const EVEN: ftdi_parity_type = ftdi_parity_type(2);
}
impl ftdi_parity_type {
    pub const MARK: ftdi_parity_type = ftdi_parity_type(3);
}
impl ftdi_parity_type {
    pub const SPACE: ftdi_parity_type = ftdi_parity_type(4);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ftdi_parity_type(pub u32);
impl ftdi_stopbits_type {
    pub const STOP_BIT_1: ftdi_stopbits_type = ftdi_stopbits_type(0);
}
impl ftdi_stopbits_type {
    pub const STOP_BIT_15: ftdi_stopbits_type = ftdi_stopbits_type(1);
}
impl ftdi_stopbits_type {
    pub const STOP_BIT_2: ftdi_stopbits_type = ftdi_stopbits_type(2);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ftdi_stopbits_type(pub u32);
impl ftdi_bits_type {
    pub const BITS_7: ftdi_bits_type = ftdi_bits_type(7);
}
impl ftdi_bits_type {
    pub const BITS_8: ftdi_bits_type = ftdi_bits_type(8);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ftdi_bits_type(pub u32);
impl ftdi_break_type {
    pub const BREAK_OFF: ftdi_break_type = ftdi_break_type(0);
}
impl ftdi_break_type {
    pub const BREAK_ON: ftdi_break_type = ftdi_break_type(1);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ftdi_break_type(pub u32);
impl ftdi_mpsse_mode {
    pub const BITMODE_RESET: ftdi_mpsse_mode = ftdi_mpsse_mode(0);
}
impl ftdi_mpsse_mode {
    pub const BITMODE_BITBANG: ftdi_mpsse_mode = ftdi_mpsse_mode(1);
}
impl ftdi_mpsse_mode {
    pub const BITMODE_MPSSE: ftdi_mpsse_mode = ftdi_mpsse_mode(2);
}
impl ftdi_mpsse_mode {
    pub const BITMODE_SYNCBB: ftdi_mpsse_mode = ftdi_mpsse_mode(4);
}
impl ftdi_mpsse_mode {
    pub const BITMODE_MCU: ftdi_mpsse_mode = ftdi_mpsse_mode(8);
}
impl ftdi_mpsse_mode {
    pub const BITMODE_OPTO: ftdi_mpsse_mode = ftdi_mpsse_mode(16);
}
impl ftdi_mpsse_mode {
    pub const BITMODE_CBUS: ftdi_mpsse_mode = ftdi_mpsse_mode(32);
}
impl ftdi_mpsse_mode {
    pub const BITMODE_SYNCFF: ftdi_mpsse_mode = ftdi_mpsse_mode(64);
}
impl ftdi_mpsse_mode {
    pub const BITMODE_FT1284: ftdi_mpsse_mode = ftdi_mpsse_mode(128);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ftdi_mpsse_mode(pub u32);
impl ftdi_interface {
    pub const INTERFACE_ANY: ftdi_interface = ftdi_interface(0);
}
impl ftdi_interface {
    pub const INTERFACE_A: ftdi_interface = ftdi_interface(1);
}
impl ftdi_interface {
    pub const INTERFACE_B: ftdi_interface = ftdi_interface(2);
}
impl ftdi_interface {
    pub const INTERFACE_C: ftdi_interface = ftdi_interface(3);
}
impl ftdi_interface {
    pub const INTERFACE_D: ftdi_interface = ftdi_interface(4);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ftdi_interface(pub u32);
impl ftdi_module_detach_mode {
    pub const AUTO_DETACH_SIO_MODULE: ftdi_module_detach_mode = ftdi_module_detach_mode(0);
}
impl ftdi_module_detach_mode {
    pub const DONT_DETACH_SIO_MODULE: ftdi_module_detach_mode = ftdi_module_detach_mode(1);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ftdi_module_detach_mode(pub u32);
#[repr(C)]
#[derive(Debug)]
pub struct ftdi_transfer_control {
    pub completed: ::std::os::raw::c_int,
    pub buf: *mut ::std::os::raw::c_uchar,
    pub size: ::std::os::raw::c_int,
    pub offset: ::std::os::raw::c_int,
    pub ftdi: *mut ftdi_context,
    pub transfer: *mut libusb_transfer,
}
#[repr(C)]
#[derive(Debug)]
pub struct ftdi_context {
    pub usb_ctx: *mut libusb_context,
    pub usb_dev: *mut libusb_device_handle,
    pub usb_read_timeout: ::std::os::raw::c_int,
    pub usb_write_timeout: ::std::os::raw::c_int,
    pub type_: ftdi_chip_type,
    pub baudrate: ::std::os::raw::c_int,
    pub bitbang_enabled: ::std::os::raw::c_uchar,
    pub readbuffer: *mut ::std::os::raw::c_uchar,
    pub readbuffer_offset: ::std::os::raw::c_uint,
    pub readbuffer_remaining: ::std::os::raw::c_uint,
    pub readbuffer_chunksize: ::std::os::raw::c_uint,
    pub writebuffer_chunksize: ::std::os::raw::c_uint,
    pub max_packet_size: ::std::os::raw::c_uint,
    pub interface: ::std::os::raw::c_int,
    pub index: ::std::os::raw::c_int,
    pub in_ep: ::std::os::raw::c_int,
    pub out_ep: ::std::os::raw::c_int,
    pub bitbang_mode: ::std::os::raw::c_uchar,
    pub eeprom: *mut ftdi_eeprom,
    pub error_str: *const ::std::os::raw::c_char,
    pub module_detach_mode: ftdi_module_detach_mode,
}
impl ftdi_eeprom_value {
    pub const VENDOR_ID: ftdi_eeprom_value = ftdi_eeprom_value(0);
}
impl ftdi_eeprom_value {
    pub const PRODUCT_ID: ftdi_eeprom_value = ftdi_eeprom_value(1);
}
impl ftdi_eeprom_value {
    pub const SELF_POWERED: ftdi_eeprom_value = ftdi_eeprom_value(2);
}
impl ftdi_eeprom_value {
    pub const REMOTE_WAKEUP: ftdi_eeprom_value = ftdi_eeprom_value(3);
}
impl ftdi_eeprom_value {
    pub const IS_NOT_PNP: ftdi_eeprom_value = ftdi_eeprom_value(4);
}
impl ftdi_eeprom_value {
    pub const SUSPEND_DBUS7: ftdi_eeprom_value = ftdi_eeprom_value(5);
}
impl ftdi_eeprom_value {
    pub const IN_IS_ISOCHRONOUS: ftdi_eeprom_value = ftdi_eeprom_value(6);
}
impl ftdi_eeprom_value {
    pub const OUT_IS_ISOCHRONOUS: ftdi_eeprom_value = ftdi_eeprom_value(7);
}
impl ftdi_eeprom_value {
    pub const SUSPEND_PULL_DOWNS: ftdi_eeprom_value = ftdi_eeprom_value(8);
}
impl ftdi_eeprom_value {
    pub const USE_SERIAL: ftdi_eeprom_value = ftdi_eeprom_value(9);
}
impl ftdi_eeprom_value {
    pub const USB_VERSION: ftdi_eeprom_value = ftdi_eeprom_value(10);
}
impl ftdi_eeprom_value {
    pub const USE_USB_VERSION: ftdi_eeprom_value = ftdi_eeprom_value(11);
}
impl ftdi_eeprom_value {
    pub const MAX_POWER: ftdi_eeprom_value = ftdi_eeprom_value(12);
}
impl ftdi_eeprom_value {
    pub const CHANNEL_A_TYPE: ftdi_eeprom_value = ftdi_eeprom_value(13);
}
impl ftdi_eeprom_value {
    pub const CHANNEL_B_TYPE: ftdi_eeprom_value = ftdi_eeprom_value(14);
}
impl ftdi_eeprom_value {
    pub const CHANNEL_A_DRIVER: ftdi_eeprom_value = ftdi_eeprom_value(15);
}
impl ftdi_eeprom_value {
    pub const CHANNEL_B_DRIVER: ftdi_eeprom_value = ftdi_eeprom_value(16);
}
impl ftdi_eeprom_value {
    pub const CBUS_FUNCTION_0: ftdi_eeprom_value = ftdi_eeprom_value(17);
}
impl ftdi_eeprom_value {
    pub const CBUS_FUNCTION_1: ftdi_eeprom_value = ftdi_eeprom_value(18);
}
impl ftdi_eeprom_value {
    pub const CBUS_FUNCTION_2: ftdi_eeprom_value = ftdi_eeprom_value(19);
}
impl ftdi_eeprom_value {
    pub const CBUS_FUNCTION_3: ftdi_eeprom_value = ftdi_eeprom_value(20);
}
impl ftdi_eeprom_value {
    pub const CBUS_FUNCTION_4: ftdi_eeprom_value = ftdi_eeprom_value(21);
}
impl ftdi_eeprom_value {
    pub const CBUS_FUNCTION_5: ftdi_eeprom_value = ftdi_eeprom_value(22);
}
impl ftdi_eeprom_value {
    pub const CBUS_FUNCTION_6: ftdi_eeprom_value = ftdi_eeprom_value(23);
}
impl ftdi_eeprom_value {
    pub const CBUS_FUNCTION_7: ftdi_eeprom_value = ftdi_eeprom_value(24);
}
impl ftdi_eeprom_value {
    pub const CBUS_FUNCTION_8: ftdi_eeprom_value = ftdi_eeprom_value(25);
}
impl ftdi_eeprom_value {
    pub const CBUS_FUNCTION_9: ftdi_eeprom_value = ftdi_eeprom_value(26);
}
impl ftdi_eeprom_value {
    pub const HIGH_CURRENT: ftdi_eeprom_value = ftdi_eeprom_value(27);
}
impl ftdi_eeprom_value {
    pub const HIGH_CURRENT_A: ftdi_eeprom_value = ftdi_eeprom_value(28);
}
impl ftdi_eeprom_value {
    pub const HIGH_CURRENT_B: ftdi_eeprom_value = ftdi_eeprom_value(29);
}
impl ftdi_eeprom_value {
    pub const INVERT: ftdi_eeprom_value = ftdi_eeprom_value(30);
}
impl ftdi_eeprom_value {
    pub const GROUP0_DRIVE: ftdi_eeprom_value = ftdi_eeprom_value(31);
}
impl ftdi_eeprom_value {
    pub const GROUP0_SCHMITT: ftdi_eeprom_value = ftdi_eeprom_value(32);
}
impl ftdi_eeprom_value {
    pub const GROUP0_SLEW: ftdi_eeprom_value = ftdi_eeprom_value(33);
}
impl ftdi_eeprom_value {
    pub const GROUP1_DRIVE: ftdi_eeprom_value = ftdi_eeprom_value(34);
}
impl ftdi_eeprom_value {
    pub const GROUP1_SCHMITT: ftdi_eeprom_value = ftdi_eeprom_value(35);
}
impl ftdi_eeprom_value {
    pub const GROUP1_SLEW: ftdi_eeprom_value = ftdi_eeprom_value(36);
}
impl ftdi_eeprom_value {
    pub const GROUP2_DRIVE: ftdi_eeprom_value = ftdi_eeprom_value(37);
}
impl ftdi_eeprom_value {
    pub const GROUP2_SCHMITT: ftdi_eeprom_value = ftdi_eeprom_value(38);
}
impl ftdi_eeprom_value {
    pub const GROUP2_SLEW: ftdi_eeprom_value = ftdi_eeprom_value(39);
}
impl ftdi_eeprom_value {
    pub const GROUP3_DRIVE: ftdi_eeprom_value = ftdi_eeprom_value(40);
}
impl ftdi_eeprom_value {
    pub const GROUP3_SCHMITT: ftdi_eeprom_value = ftdi_eeprom_value(41);
}
impl ftdi_eeprom_value {
    pub const GROUP3_SLEW: ftdi_eeprom_value = ftdi_eeprom_value(42);
}
impl ftdi_eeprom_value {
    pub const CHIP_SIZE: ftdi_eeprom_value = ftdi_eeprom_value(43);
}
impl ftdi_eeprom_value {
    pub const CHIP_TYPE: ftdi_eeprom_value = ftdi_eeprom_value(44);
}
impl ftdi_eeprom_value {
    pub const POWER_SAVE: ftdi_eeprom_value = ftdi_eeprom_value(45);
}
impl ftdi_eeprom_value {
    pub const CLOCK_POLARITY: ftdi_eeprom_value = ftdi_eeprom_value(46);
}
impl ftdi_eeprom_value {
    pub const DATA_ORDER: ftdi_eeprom_value = ftdi_eeprom_value(47);
}
impl ftdi_eeprom_value {
    pub const FLOW_CONTROL: ftdi_eeprom_value = ftdi_eeprom_value(48);
}
impl ftdi_eeprom_value {
    pub const CHANNEL_C_DRIVER: ftdi_eeprom_value = ftdi_eeprom_value(49);
}
impl ftdi_eeprom_value {
    pub const CHANNEL_D_DRIVER: ftdi_eeprom_value = ftdi_eeprom_value(50);
}
impl ftdi_eeprom_value {
    pub const CHANNEL_A_RS485: ftdi_eeprom_value = ftdi_eeprom_value(51);
}
impl ftdi_eeprom_value {
    pub const CHANNEL_B_RS485: ftdi_eeprom_value = ftdi_eeprom_value(52);
}
impl ftdi_eeprom_value {
    pub const CHANNEL_C_RS485: ftdi_eeprom_value = ftdi_eeprom_value(53);
}
impl ftdi_eeprom_value {
    pub const CHANNEL_D_RS485: ftdi_eeprom_value = ftdi_eeprom_value(54);
}
impl ftdi_eeprom_value {
    pub const RELEASE_NUMBER: ftdi_eeprom_value = ftdi_eeprom_value(55);
}
impl ftdi_eeprom_value {
    pub const EXTERNAL_OSCILLATOR: ftdi_eeprom_value = ftdi_eeprom_value(56);
}
impl ftdi_eeprom_value {
    pub const USER_DATA_ADDR: ftdi_eeprom_value = ftdi_eeprom_value(57);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ftdi_eeprom_value(pub u32);
#[repr(C)]
#[derive(Debug)]
pub struct ftdi_device_list {
    pub next: *mut ftdi_device_list,
    pub dev: *mut libusb_device,
}
impl ftdi_cbus_func {
    pub const CBUS_TXDEN: ftdi_cbus_func = ftdi_cbus_func(0);
}
impl ftdi_cbus_func {
    pub const CBUS_PWREN: ftdi_cbus_func = ftdi_cbus_func(1);
}
impl ftdi_cbus_func {
    pub const CBUS_RXLED: ftdi_cbus_func = ftdi_cbus_func(2);
}
impl ftdi_cbus_func {
    pub const CBUS_TXLED: ftdi_cbus_func = ftdi_cbus_func(3);
}
impl ftdi_cbus_func {
    pub const CBUS_TXRXLED: ftdi_cbus_func = ftdi_cbus_func(4);
}
impl ftdi_cbus_func {
    pub const CBUS_SLEEP: ftdi_cbus_func = ftdi_cbus_func(5);
}
impl ftdi_cbus_func {
    pub const CBUS_CLK48: ftdi_cbus_func = ftdi_cbus_func(6);
}
impl ftdi_cbus_func {
    pub const CBUS_CLK24: ftdi_cbus_func = ftdi_cbus_func(7);
}
impl ftdi_cbus_func {
    pub const CBUS_CLK12: ftdi_cbus_func = ftdi_cbus_func(8);
}
impl ftdi_cbus_func {
    pub const CBUS_CLK6: ftdi_cbus_func = ftdi_cbus_func(9);
}
impl ftdi_cbus_func {
    pub const CBUS_IOMODE: ftdi_cbus_func = ftdi_cbus_func(10);
}
impl ftdi_cbus_func {
    pub const CBUS_BB_WR: ftdi_cbus_func = ftdi_cbus_func(11);
}
impl ftdi_cbus_func {
    pub const CBUS_BB_RD: ftdi_cbus_func = ftdi_cbus_func(12);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ftdi_cbus_func(pub u32);
impl ftdi_cbush_func {
    pub const CBUSH_TRISTATE: ftdi_cbush_func = ftdi_cbush_func(0);
}
impl ftdi_cbush_func {
    pub const CBUSH_TXLED: ftdi_cbush_func = ftdi_cbush_func(1);
}
impl ftdi_cbush_func {
    pub const CBUSH_RXLED: ftdi_cbush_func = ftdi_cbush_func(2);
}
impl ftdi_cbush_func {
    pub const CBUSH_TXRXLED: ftdi_cbush_func = ftdi_cbush_func(3);
}
impl ftdi_cbush_func {
    pub const CBUSH_PWREN: ftdi_cbush_func = ftdi_cbush_func(4);
}
impl ftdi_cbush_func {
    pub const CBUSH_SLEEP: ftdi_cbush_func = ftdi_cbush_func(5);
}
impl ftdi_cbush_func {
    pub const CBUSH_DRIVE_0: ftdi_cbush_func = ftdi_cbush_func(6);
}
impl ftdi_cbush_func {
    pub const CBUSH_DRIVE1: ftdi_cbush_func = ftdi_cbush_func(7);
}
impl ftdi_cbush_func {
    pub const CBUSH_IOMODE: ftdi_cbush_func = ftdi_cbush_func(8);
}
impl ftdi_cbush_func {
    pub const CBUSH_TXDEN: ftdi_cbush_func = ftdi_cbush_func(9);
}
impl ftdi_cbush_func {
    pub const CBUSH_CLK30: ftdi_cbush_func = ftdi_cbush_func(10);
}
impl ftdi_cbush_func {
    pub const CBUSH_CLK15: ftdi_cbush_func = ftdi_cbush_func(11);
}
impl ftdi_cbush_func {
    pub const CBUSH_CLK7_5: ftdi_cbush_func = ftdi_cbush_func(12);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ftdi_cbush_func(pub u32);
impl ftdi_cbusx_func {
    pub const CBUSX_TRISTATE: ftdi_cbusx_func = ftdi_cbusx_func(0);
}
impl ftdi_cbusx_func {
    pub const CBUSX_TXLED: ftdi_cbusx_func = ftdi_cbusx_func(1);
}
impl ftdi_cbusx_func {
    pub const CBUSX_RXLED: ftdi_cbusx_func = ftdi_cbusx_func(2);
}
impl ftdi_cbusx_func {
    pub const CBUSX_TXRXLED: ftdi_cbusx_func = ftdi_cbusx_func(3);
}
impl ftdi_cbusx_func {
    pub const CBUSX_PWREN: ftdi_cbusx_func = ftdi_cbusx_func(4);
}
impl ftdi_cbusx_func {
    pub const CBUSX_SLEEP: ftdi_cbusx_func = ftdi_cbusx_func(5);
}
impl ftdi_cbusx_func {
    pub const CBUSX_DRIVE_0: ftdi_cbusx_func = ftdi_cbusx_func(6);
}
impl ftdi_cbusx_func {
    pub const CBUSX_DRIVE1: ftdi_cbusx_func = ftdi_cbusx_func(7);
}
impl ftdi_cbusx_func {
    pub const CBUSX_IOMODE: ftdi_cbusx_func = ftdi_cbusx_func(8);
}
impl ftdi_cbusx_func {
    pub const CBUSX_TXDEN: ftdi_cbusx_func = ftdi_cbusx_func(9);
}
impl ftdi_cbusx_func {
    pub const CBUSX_CLK24: ftdi_cbusx_func = ftdi_cbusx_func(10);
}
impl ftdi_cbusx_func {
    pub const CBUSX_CLK12: ftdi_cbusx_func = ftdi_cbusx_func(11);
}
impl ftdi_cbusx_func {
    pub const CBUSX_CLK6: ftdi_cbusx_func = ftdi_cbusx_func(12);
}
impl ftdi_cbusx_func {
    pub const CBUSX_BAT_DETECT: ftdi_cbusx_func = ftdi_cbusx_func(13);
}
impl ftdi_cbusx_func {
    pub const CBUSX_BAT_DETECT_NEG: ftdi_cbusx_func = ftdi_cbusx_func(14);
}
impl ftdi_cbusx_func {
    pub const CBUSX_I2C_TXE: ftdi_cbusx_func = ftdi_cbusx_func(15);
}
impl ftdi_cbusx_func {
    pub const CBUSX_I2C_RXF: ftdi_cbusx_func = ftdi_cbusx_func(16);
}
impl ftdi_cbusx_func {
    pub const CBUSX_VBUS_SENSE: ftdi_cbusx_func = ftdi_cbusx_func(17);
}
impl ftdi_cbusx_func {
    pub const CBUSX_BB_WR: ftdi_cbusx_func = ftdi_cbusx_func(18);
}
impl ftdi_cbusx_func {
    pub const CBUSX_BB_RD: ftdi_cbusx_func = ftdi_cbusx_func(19);
}
impl ftdi_cbusx_func {
    pub const CBUSX_TIME_STAMP: ftdi_cbusx_func = ftdi_cbusx_func(20);
}
impl ftdi_cbusx_func {
    pub const CBUSX_AWAKE: ftdi_cbusx_func = ftdi_cbusx_func(21);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ftdi_cbusx_func(pub u32);
#[repr(C)]
pub struct size_and_time {
    pub totalBytes: u64,
    pub time: timeval,
}
#[repr(C)]
pub struct FTDIProgressInfo {
    pub first: size_and_time,
    pub prev: size_and_time,
    pub current: size_and_time,
    pub totalTime: f64,
    pub totalRate: f64,
    pub currentRate: f64,
}
pub type FTDIStreamCallback = ::std::option::Option<
    unsafe extern "C" fn(
        buffer: *mut u8,
        length: ::std::os::raw::c_int,
        progress: *mut FTDIProgressInfo,
        userdata: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int,
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ftdi_version_info {
    pub major: ::std::os::raw::c_int,
    pub minor: ::std::os::raw::c_int,
    pub micro: ::std::os::raw::c_int,
    pub version_str: *const ::std::os::raw::c_char,
    pub snapshot_str: *const ::std::os::raw::c_char,
}
extern "C" {
    pub fn ftdi_init(ftdi: *mut ftdi_context) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ftdi_new() -> *mut ftdi_context;
}
extern "C" {
    pub fn ftdi_set_interface(
        ftdi: *mut ftdi_context,
        interface: ftdi_interface,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ftdi_deinit(ftdi: *mut ftdi_context);
}
extern "C" {
    pub fn ftdi_free(ftdi: *mut ftdi_context);
}
extern "C" {
    pub fn ftdi_set_usbdev(ftdi: *mut ftdi_context, usbdev: *mut libusb_device_handle);
}
extern "C" {
    pub fn ftdi_get_library_version() -> ftdi_version_info;
}
extern "C" {
    pub fn ftdi_usb_find_all(
        ftdi: *mut ftdi_context,
        devlist: *mut *mut ftdi_device_list,
        vendor: ::std::os::raw::c_int,
        product: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ftdi_list_free(devlist: *mut *mut ftdi_device_list);
}
extern "C" {
    pub fn ftdi_list_free2(devlist: *mut ftdi_device_list);
}
extern "C" {
    pub fn ftdi_usb_get_strings(
        ftdi: *mut ftdi_context,
        dev: *mut libusb_device,
        manufacturer: *mut ::std::os::raw::c_char,
        mnf_len: ::std::os::raw::c_int,
        description: *mut ::std::os::raw::c_char,
        desc_len: ::std::os::raw::c_int,
        serial: *mut ::std::os::raw::c_char,
        serial_len: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ftdi_usb_get_strings2(
        ftdi: *mut ftdi_context,
        dev: *mut libusb_device,
        manufacturer: *mut ::std::os::raw::c_char,
        mnf_len: ::std::os::raw::c_int,
        description: *mut ::std::os::raw::c_char,
        desc_len: ::std::os::raw::c_int,
        serial: *mut ::std::os::raw::c_char,
        serial_len: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ftdi_eeprom_get_strings(
        ftdi: *mut ftdi_context,
        manufacturer: *mut ::std::os::raw::c_char,
        mnf_len: ::std::os::raw::c_int,
        product: *mut ::std::os::raw::c_char,
        prod_len: ::std::os::raw::c_int,
        serial: *mut ::std::os::raw::c_char,
        serial_len: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ftdi_eeprom_set_strings(
        ftdi: *mut ftdi_context,
        manufacturer: *mut ::std::os::raw::c_char,
        product: *mut ::std::os::raw::c_char,
        serial: *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ftdi_usb_open(
        ftdi: *mut ftdi_context,
        vendor: ::std::os::raw::c_int,
        product: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ftdi_usb_open_desc(
        ftdi: *mut ftdi_context,
        vendor: ::std::os::raw::c_int,
        product: ::std::os::raw::c_int,
        description: *const ::std::os::raw::c_char,
        serial: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ftdi_usb_open_desc_index(
        ftdi: *mut ftdi_context,
        vendor: ::std::os::raw::c_int,
        product: ::std::os::raw::c_int,
        description: *const ::std::os::raw::c_char,
        serial: *const ::std::os::raw::c_char,
        index: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ftdi_usb_open_bus_addr(
        ftdi: *mut ftdi_context,
        bus: u8,
        addr: u8,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ftdi_usb_open_dev(
        ftdi: *mut ftdi_context,
        dev: *mut libusb_device,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ftdi_usb_open_string(
        ftdi: *mut ftdi_context,
        description: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ftdi_usb_close(ftdi: *mut ftdi_context) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ftdi_usb_reset(ftdi: *mut ftdi_context) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ftdi_usb_purge_rx_buffer(ftdi: *mut ftdi_context) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ftdi_usb_purge_tx_buffer(ftdi: *mut ftdi_context) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ftdi_usb_purge_buffers(ftdi: *mut ftdi_context) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ftdi_set_baudrate(
        ftdi: *mut ftdi_context,
        baudrate: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ftdi_set_line_property(
        ftdi: *mut ftdi_context,
        bits: ftdi_bits_type,
        sbit: ftdi_stopbits_type,
        parity: ftdi_parity_type,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ftdi_set_line_property2(
        ftdi: *mut ftdi_context,
        bits: ftdi_bits_type,
        sbit: ftdi_stopbits_type,
        parity: ftdi_parity_type,
        break_type: ftdi_break_type,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ftdi_read_data(
        ftdi: *mut ftdi_context,
        buf: *mut ::std::os::raw::c_uchar,
        size: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ftdi_read_data_set_chunksize(
        ftdi: *mut ftdi_context,
        chunksize: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ftdi_read_data_get_chunksize(
        ftdi: *mut ftdi_context,
        chunksize: *mut ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ftdi_write_data(
        ftdi: *mut ftdi_context,
        buf: *const ::std::os::raw::c_uchar,
        size: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ftdi_write_data_set_chunksize(
        ftdi: *mut ftdi_context,
        chunksize: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ftdi_write_data_get_chunksize(
        ftdi: *mut ftdi_context,
        chunksize: *mut ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ftdi_readstream(
        ftdi: *mut ftdi_context,
        callback: FTDIStreamCallback,
        userdata: *mut ::std::os::raw::c_void,
        packetsPerTransfer: ::std::os::raw::c_int,
        numTransfers: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ftdi_write_data_submit(
        ftdi: *mut ftdi_context,
        buf: *mut ::std::os::raw::c_uchar,
        size: ::std::os::raw::c_int,
    ) -> *mut ftdi_transfer_control;
}
extern "C" {
    pub fn ftdi_read_data_submit(
        ftdi: *mut ftdi_context,
        buf: *mut ::std::os::raw::c_uchar,
        size: ::std::os::raw::c_int,
    ) -> *mut ftdi_transfer_control;
}
extern "C" {
    pub fn ftdi_transfer_data_done(tc: *mut ftdi_transfer_control) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ftdi_transfer_data_cancel(tc: *mut ftdi_transfer_control, to: *mut timeval);
}
extern "C" {
    pub fn ftdi_set_bitmode(
        ftdi: *mut ftdi_context,
        bitmask: ::std::os::raw::c_uchar,
        mode: ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ftdi_disable_bitbang(ftdi: *mut ftdi_context) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ftdi_read_pins(
        ftdi: *mut ftdi_context,
        pins: *mut ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ftdi_set_latency_timer(
        ftdi: *mut ftdi_context,
        latency: ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ftdi_get_latency_timer(
        ftdi: *mut ftdi_context,
        latency: *mut ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ftdi_poll_modem_status(
        ftdi: *mut ftdi_context,
        status: *mut ::std::os::raw::c_ushort,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ftdi_setflowctrl(
        ftdi: *mut ftdi_context,
        flowctrl: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ftdi_setdtr_rts(
        ftdi: *mut ftdi_context,
        dtr: ::std::os::raw::c_int,
        rts: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ftdi_setdtr(
        ftdi: *mut ftdi_context,
        state: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ftdi_setrts(
        ftdi: *mut ftdi_context,
        state: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ftdi_set_event_char(
        ftdi: *mut ftdi_context,
        eventch: ::std::os::raw::c_uchar,
        enable: ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ftdi_set_error_char(
        ftdi: *mut ftdi_context,
        errorch: ::std::os::raw::c_uchar,
        enable: ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ftdi_eeprom_initdefaults(
        ftdi: *mut ftdi_context,
        manufacturer: *mut ::std::os::raw::c_char,
        product: *mut ::std::os::raw::c_char,
        serial: *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ftdi_eeprom_build(ftdi: *mut ftdi_context) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ftdi_eeprom_decode(
        ftdi: *mut ftdi_context,
        verbose: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ftdi_get_eeprom_value(
        ftdi: *mut ftdi_context,
        value_name: ftdi_eeprom_value,
        value: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ftdi_set_eeprom_value(
        ftdi: *mut ftdi_context,
        value_name: ftdi_eeprom_value,
        value: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ftdi_get_eeprom_buf(
        ftdi: *mut ftdi_context,
        buf: *mut ::std::os::raw::c_uchar,
        size: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ftdi_set_eeprom_buf(
        ftdi: *mut ftdi_context,
        buf: *const ::std::os::raw::c_uchar,
        size: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ftdi_set_eeprom_user_data(
        ftdi: *mut ftdi_context,
        buf: *const ::std::os::raw::c_char,
        size: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ftdi_read_eeprom(ftdi: *mut ftdi_context) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ftdi_read_chipid(
        ftdi: *mut ftdi_context,
        chipid: *mut ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ftdi_write_eeprom(ftdi: *mut ftdi_context) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ftdi_erase_eeprom(ftdi: *mut ftdi_context) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ftdi_read_eeprom_location(
        ftdi: *mut ftdi_context,
        eeprom_addr: ::std::os::raw::c_int,
        eeprom_val: *mut ::std::os::raw::c_ushort,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ftdi_write_eeprom_location(
        ftdi: *mut ftdi_context,
        eeprom_addr: ::std::os::raw::c_int,
        eeprom_val: ::std::os::raw::c_ushort,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ftdi_get_error_string(ftdi: *mut ftdi_context) -> *const ::std::os::raw::c_char;
}
#[repr(C)]
#[derive(Debug)]
pub struct ftdi_eeprom {
    pub _address: u8,
}
