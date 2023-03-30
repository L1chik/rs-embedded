#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::{*, panic};
use futures::future::join;

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};

use embassy_stm32::{
    exti::ExtiInput,
    gpio::{Output, AnyPin, Input, Pull, Level, Speed},
};
use embassy_stm32::time::mhz;
use embassy_stm32::{interrupt, Config};
use embassy_stm32::usb_otg::{Driver, Instance};

use embassy_usb::class::cdc_acm::{CdcAcmClass, State};
use embassy_usb::driver::EndpointError;
use embassy_usb::Builder;

use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::task]
async fn blink(mut led: Output<'static, AnyPin>) {
    loop {
        led.set_high();
        info!("*blink*");
        Timer::after(Duration::from_millis(800)).await;
        led.set_low();
        Timer::after(Duration::from_millis(800)).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("Hey there");

    let mut config = Config::default();
    config.rcc.hse = Some(mhz(8));
    config.rcc.pll48 = true;
    config.rcc.sys_ck = Some(mhz(200));

    let p = embassy_stm32::init(config);

    let irq = interrupt::take!(OTG_FS);
    let mut buffer = [0u8; 256];
    let driver = Driver::new_fs(p.USB_OTG_FS, irq, p.PA12, p.PA11, &mut buffer);

    let mut config = embassy_usb::Config::new(0xc0de, 0xcafe);
    config.manufacturer = Some("MTUCI");
    config.product = Some("Drone V1");
    config.serial_number = Some("1");

    let mut sevice_descriptor = [0; 256];
    let mut config_descriptor = [0; 256];
    let mut bos_descriptor = [0; 256];
    let mut control_buf = [0; 64];

    let mut state = State::new();

    let mut builder = Builder::new(
        driver,
        config,
        &mut sevice_descriptor,
        &mut config_descriptor,
        &mut bos_descriptor,
        &mut control_buf,
    );

    let mut class = CdcAcmClass::new(&mut builder, &mut state, 64);

    let mut usb = builder.build();

    let mut usb_fut = usb.run();

    let echo_fut = async {
        loop {
            class.wait_connection().await;
            info!("Connected");
            let _ = echo(&mut class).await;
            info!("Disconnected");
        }
    };

    info!("Before join");

    join(usb_fut, echo_fut).await;

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