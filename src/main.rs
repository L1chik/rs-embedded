#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]



use core::sync::atomic::{AtomicU32, Ordering};
use core::fmt::Write;
use defmt::{*, panic};
use futures::future::join;
use heapless::String;

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};

use embassy_stm32::{exti::ExtiInput, gpio::{Output, AnyPin, Input, Pull, Level, Speed}, Peripheral};
use embassy_stm32::time::mhz;
use embassy_stm32::{interrupt};
use embassy_stm32::can::TxPin;
use embassy_stm32::dma::NoDma;
use embassy_stm32::gpio::Pin;
use embassy_stm32::usart::{UartTx, Config};
use embassy_stm32::usb_otg::{Driver, Instance};

use embassy_usb::class::cdc_acm::{CdcAcmClass, State};
use embassy_usb::driver::EndpointError;
use embassy_usb::Builder;

use {defmt_rtt as _, panic_probe as _};

static BLINK_MS: AtomicU32 = AtomicU32::new(0);

#[embassy_executor::task]
async fn blink(mut led: AnyPin) {
    let mut led = Output::new(led, Level::Low,Speed::Low);

    loop {
        let del = BLINK_MS.load(Ordering::Relaxed);
        Timer::after(Duration::from_millis(del.into())).await;
        led.toggle();
    }
}

struct Disconnect {}

impl From<EndpointError> for Disconnect {
    fn from(val: EndpointError) -> Self {
        match val {
            EndpointError::BufferOverflow => panic!("Buffer overflow"),
            EndpointError::Disabled => Disconnect {  },
        }
    }
}

async fn echo<'d, T: Instance + 'd>(class: &mut CdcAcmClass<'d, Driver<'d, T>>) -> Result<(), Disconnect> {
    let mut buf = [0; 64];

    loop {
        let n = class.read_packet(&mut buf).await?;
        let data = &buf[..n];
        info!("data: {:x}", data);
        class.write_packet(data).await?;
    }
}


#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("Hey there");

    // let mut config = embaConfig::default();
    // config.rcc.hse = Some(mhz(8));
    // config.rcc.pll48 = true;
    // config.rcc.sys_ck = Some(mhz(200));

    // let p = embassy_stm32::init(config);
    let p = embassy_stm32::init(Default::default());

    // let irq = interrupt::take!(OTG_FS);
    // let mut buffer = [0u8; 256];
    // let driver = Driver::new_fs(p.USB_OTG_FS, irq, p.PA12, p.PA11, &mut buffer);

    // let mut config = embassy_usb::Config::new(0xc0de, 0xcafe);
    // config.manufacturer = Some("MTUCI");
    // config.product = Some("Drone");
    // config.serial_number = Some("1");
    //
    // let mut device_descriptor = [0; 256];
    // let mut config_descriptor = [0; 256];
    // let mut bos_descriptor = [0; 256];
    // let mut control_buf = [0; 64];
    //
    // let mut state = State::new();
    //
    // let mut builder = Builder::new(
    //     driver,
    //     config,
    //     &mut device_descriptor,
    //     &mut config_descriptor,
    //     &mut bos_descriptor,
    //     &mut control_buf,
    // );
    //
    // let mut class = CdcAcmClass::new(&mut builder, &mut state, 64);
    //
    // let mut usb = builder.build();
    //
    // let mut usb_fut = usb.run();
    //
    // let echo_fut = async {
    //     loop {
    //         class.wait_connection().await;
    //         info!("Connected");
    //         let _ = echo(&mut class).await;
    //         info!("Disconnected");
    //     }
    // };

    let mut button = ExtiInput::new(
        Input::new(p.PC13, Pull::Down), p.EXTI13);

    let mut usart = UsartTx::new(p.USART2, p.PA2, NoDma,Config::default());

    info!("Before join");

    let mut del_var = 2000;
    BLINK_MS.store(del_var, Ordering::Relaxed);

    spawner.spawn(blink(p.PB7.degrade())).unwrap();

    let mut val: u8 = 0;
    let mut msg: String<8> = String::new();

    loop {
        button.wait_for_rising_edge().await;

        del_var = del_var - 300;

        if del_var < 500 {
            del_var = 2000;
        }

        BLINK_MS.store(del_var, Ordering::Relaxed);
        core::writeln!(&mut msg, "{:02}", val).unwrap();

        usart.blocking_write(msg.as_bytes()).unwrap();

        val = val.wrapping_add(1);
        msg.clear();
    }


    // join(usb_fut, echo_fut).await;

//    let blue = Output::new(p.PB7, Level::Low, Speed::Low).degrade();
//    spawner.spawn(blink(blue)).unwrap();
//
//    let mut button = ExtiInput::new(Input::new(p.PC13, Pull::Down), p.EXTI13);
//
//
//    loop {
//        button.wait_for_falling_edge().await;
//        info!("*click*");
//    }
}