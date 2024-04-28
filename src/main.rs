use std::thread;
use std::sync::mpsc::{self, Sender, Receiver};

use eframe::egui;
use eframe::epaint::FontId;
use jisp_sha2 as sha;

fn main() {
    let mut native_options = eframe::NativeOptions::default();
    let _ = native_options.viewport.inner_size.insert((660., 480.).into());
    let _ = eframe::run_native("SHA-2", native_options, Box::new(|cc| Box::new(MultProgram::new(cc))))
        .expect("Unexpected Error");
}


enum Message {
    Hex(String),
    Hash(String)
}

struct MultProgram {
    input:String,
    hex:String,
    hash:String,
    thread_active:bool,
    tx:Sender<String>,
    rx:Receiver<Message>

}

impl MultProgram {
    fn new(cc:&eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        let (tx1, rx1) = mpsc::channel();
        let (tx2, rx2) = mpsc::channel::<Message>();
        thread::spawn(move|| {
            let tx:Sender<Message> = tx2;
            let rx:Receiver<String> = rx1;
            for s in rx.iter() {
                let i = sha::parser::sha256_preprocessing(&s);
                let hex_text = sha::printer::print_blocks(&i,true);
                tx.send(Message::Hex(hex_text)).unwrap();
                let hash = sha::sha256::sha_256(i);
                let hash_text = sha::printer::print_blocks(&vec![hash],true);
                tx.send(Message::Hash(hash_text)).unwrap();
            }
        });
        Self {
            tx: tx1, rx: rx2,
            thread_active:false,
            input: "".to_owned(),
            hex: "".to_owned(),
            hash: "".to_owned(),
        }
    }
}

impl eframe::App for MultProgram {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.thread_active {
                match self.rx.try_recv() {
                    Ok(Message::Hex(text)) => self.hex = text,
                    Ok(Message::Hash(text)) => {
                        self.hash = text;
                        self.thread_active = false;
                    },
                    Err(_) => (), 
                }
            }

            ui.group(|ui| {
                egui::Grid::new("stuff").show(ui, |ui| {
                    ui.label("[IN]:");
                    ui.add_sized((520., 20.), egui::TextEdit::singleline(&mut self.input).hint_text("Input Text..."));
                    if ui.button("Submit").clicked() && !self.thread_active {
                        self.tx.send(self.input.trim().to_owned()).unwrap();
                        self.thread_active = true;
                        self.hex = "[Loading...]".to_owned();
                        self.hash = "[Loading...]".to_owned();
                    }
                    ui.end_row();
                    ui.end_row();
                    ui.label("[HEX]:"); 
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {

                        //ui.style_mut().visuals.override_text_color = Some(egui::Color32::WHITE);
                        ui.style_mut().override_font_id = Some(FontId::monospace(12.));
                        ui.add(egui::Label::new(&self.hex).selectable(true).wrap(true));
                    });

                    ui.end_row();
                    ui.end_row();
                    ui.label("[OUT]:"); 
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {

                        //ui.style_mut().visuals.override_text_color = Some(egui::Color32::WHITE);
                        ui.style_mut().override_font_id = Some(FontId::monospace(12.));
                        ui.add(egui::Label::new(&self.hash).selectable(true).wrap(true));
                    })
                    
                    
                });
                
            })
            

        });
    }
}
