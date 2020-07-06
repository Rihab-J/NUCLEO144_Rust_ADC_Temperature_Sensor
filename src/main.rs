#![no_main]
#![no_std]
#[allow(unused_extern_crates)] 
#[allow(unused_imports)]

extern crate panic_semihosting ;

//use panic_halt;

//use core::result::Result;
//use heapless::{consts, Vec}; 
extern crate embedded_hal;
extern crate stm32f4xx_hal;
use cortex_m_rt as rt;
use stm32f4xx_hal as hal;
use stm32f4xx_hal::adc::Adc;
use hal::adc::Temperature;

use cortex_m_semihosting::hprintln; 
//use hal::prelude::*; //for split et constrain and other exti




#[rt::entry]
fn main() -> ! {
   if let Some(p) = hal::stm32::Peripherals::take(){
       let  config_adc = hal::adc::config::AdcConfig::default();
       let mut temp_adc = Adc::adc1(p.ADC1,true,config_adc); //constr
       temp_adc.enable_temperature_and_vref(); // enable 
       temp_adc.start_conversion();
       //let gpiob=p.GPIOB.split();
       /*let mut led1 = gpiob.pb0.into_push_pull_output();
       let mut led2 = gpiob.pb7.into_push_pull_output();*/
      
       
     loop{
         
       let  adc_sample = temp_adc.convert(&Temperature,stm32f4xx_hal::adc::config::SampleTime::Cycles_480);
       let  millivolts = temp_adc.sample_to_millivolts(adc_sample);
       let  temp = ((millivolts - 760)as f32  / 2.5 ) + 25.0 ; 
       //led1.set_high().unwrap();
       /*if temp > 10.0 {
        led1.set_low().unwrap();
       }
       if temp <10.0 {
           led2.set_high().unwrap();
       }*/
         hprintln!("The temperature is : {} ",temp).unwrap(); 

       
 

       }
   }
loop{
    continue;
}
}