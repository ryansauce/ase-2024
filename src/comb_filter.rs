use std::time::Duration;
use std::thread;
pub struct CombFilter  {
    filter: Vec<f32>,
    f_type: FilterType,
    max_delay: usize,
    gain: f32,
    delay: usize,
    head: usize,
    tail: usize,
    channels: usize
}

#[derive(Debug, Clone, Copy)]
pub enum FilterType {
    FIR,
    IIR,
}

#[derive(Debug, Clone, Copy)]
pub enum FilterParam {
    Gain,
    Delay,
}

#[derive(Debug, Clone)]
pub enum Error {
    InvalidValue { param: FilterParam, value: f32 }
}

impl CombFilter {
    pub fn new(filter_type: FilterType, max_delay_secs: f32, sample_rate_hz: f32, num_channels: usize) -> Self {
        let max_delay_samps = (max_delay_secs * sample_rate_hz * num_channels as f32) as usize;
        CombFilter {
            filter:  vec![f32::default(); max_delay_samps],
            f_type: filter_type,
            max_delay: max_delay_samps,
            gain: 1.0,
            delay: max_delay_samps - 1,
            head: 0,
            tail: 1, //delay is defaulting to the max
            channels: num_channels
        }
    }

    pub fn reset(&mut self) {
        self.filter.fill(f32::default());
        self.head = 0;
        self.tail = 0;
    }

    pub fn process(&mut self, input: &[&[f32]], output: &mut [&mut [f32]], channel_num: usize) {
        
        let outer_length = input[0].len();
        for i in (channel_num..outer_length / 2).step_by(self.channels) {
            
            
            for j in 0..self.channels {
                dbg!((i+1)*(j+1)-1);
                let write_val = input[0][(i+1)*(j+1)-1];
                let read_val = self.filter[self.tail];
                //self[i*self.channels + j] = write_val;

                let out_val = write_val + (read_val * self.gain);
                output[0][(i+1)*(j+1)-1] = out_val;
                //dbg!(input);
                
                match self.f_type {
                    FilterType::FIR => self.filter[self.head] = write_val,
                    FilterType::IIR => self.filter[self.head] = out_val
                }
                self.head = self.head + 1 % self.max_delay;
                self.tail += 1;
            }
            //dbg!(&output);
            //thread::sleep(Duration::from_secs(1));
        }
    }
    /* 

    pub fn set_param(&mut self, param: FilterParam, value: f32) -> Result<(), Error> {
        match param {
            FilterParam::Gain => self.gain = param,
            FilterParam::Delay => self.delay = param
        }
    }

    pub fn get_param(&self, param: FilterParam) -> f32 {
        match param {
            FilterParam::Gain => self.gain,
            FilterParam::Delay => self.delay
        }
    }
    */

    // TODO: feel free to define other functions for your own use
}

// TODO: feel free to define other types (here or in other modules) for your own use
