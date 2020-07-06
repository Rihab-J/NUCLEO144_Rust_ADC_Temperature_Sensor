#![no_main]
#![no_std]
#[allow(unused_extern_crates)] 
#[allow(unused_imports)]

//use serde::{Serialize, Deserialize}; // imports both the trait and the derive macro
use panic_halt;

//use core::result::Result;
//use heapless::{consts, Vec}; 
extern crate embedded_hal;
extern crate stm32f4xx_hal;
use cortex_m_rt as rt;
use stm32f4xx_hal as hal;
use stm32f4xx_hal::adc::Adc;
use hal::adc::Temperature;


#[rt::entry]
fn main() -> ! {
   if let Some(p) = hal::stm32::Peripherals::take(){
       let  config_adc = hal::adc::config::AdcConfig::default();
       let mut temp_adc = Adc::adc1(p.ADC1,true,config_adc); //constr
       temp_adc.enable_temperature_and_vref(); // enable 
       temp_adc.start_conversion();
       
       
     loop{
         
        let mut adc_sample = temp_adc.convert(&Temperature,stm32f4xx_hal::adc::config::SampleTime::Cycles_480);
        let mut millivolts = temp_adc.sample_to_millivolts(adc_sample);
        let mut temp = ((millivolts - 760)as f32  / 2.5 ) + 25.0 ; 
        println!("Temperature {}", temp);
     
       
 

       }
   }
loop{
    continue;
}
}