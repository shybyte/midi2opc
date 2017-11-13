#![allow(unused_mut)]

#[macro_use]
extern crate chan;
extern crate portmidi as pm;
extern crate chan_signal;
extern crate midi_message;


mod color;
mod color_strip; //
mod opc_strip;
mod midi_light_strip;


use midi_light_strip::MidiLightConfig;
use pm::PortMidi;

use chan_signal::Signal;

use std::thread;
use std::time::Duration;

fn print_devices(pm: &PortMidi) {
    for dev in pm.devices().unwrap() {
        println!("{}", dev);
    }
}

const BUF_LEN: usize = 1024;

const LED_COUNT: usize = 30; // t


fn main() {
    println!("Start blinking...");

    let context = pm::PortMidi::new().unwrap();
    print_devices(&context);

    let (tx, rx) = chan::sync(0);
    let os_signal = chan_signal::notify(&[Signal::INT, Signal::TERM]);

    let in_devices: Vec<pm::DeviceInfo> = context.devices()
        .unwrap()
        .into_iter()
        .filter(|dev| dev.is_input())
        .collect();
    let in_ports: Vec<pm::InputPort> = in_devices.into_iter()
        .filter_map(|dev| {
            context.input_port(dev, BUF_LEN)
                .ok()
        })
        .collect();

    thread::spawn(move || {
        let timeout = Duration::from_millis(10);
        loop {
            for port in &in_ports {
                if let Ok(Some(events)) = port.read_n(BUF_LEN) {
                    tx.send((port.device(), events));
                }
            }
            thread::sleep(timeout);
        }
    });

    let midi_light_strip = midi_light_strip::MidiLightStrip::start(MidiLightConfig {
        led_count: LED_COUNT,
        blink: true,
        flash: true,
        stream: true,
        max_note: 128,
        ..Default::default()
    }).unwrap();

    loop {
        chan_select! {
            rx.recv() -> midi_events => {
                let (_device, events) = midi_events.unwrap();
                for event in events {
                    match event.message.status {
                        248 => continue,
                        _ => {
                            println!("event = {:?}", event);
                            midi_light_strip.on_raw_midi_message(event.message.status,event.message.data1,event.message.data2);
                        }
                    }
                }
            },
            os_signal.recv() -> os_sig => {
                println!("received os signal: {:?}", os_sig);
                if os_sig == Some(Signal::INT) {
                    break;
                }
            }
        }
    }
}
