use std::{thread, fmt};
use std::sync::mpsc::{self, Sender, Receiver};

use eframe::egui;
use eframe::epaint::FontId;
use jisp_sha2 as sha;
use sha::printer::{print_blocks, print_u32_word_string};

fn main() {
    let mut native_options = eframe::NativeOptions::default();
    let _ = native_options.viewport.inner_size.insert((660., 480.).into());
    let _ = eframe::run_native("SHA-2", native_options, Box::new(|cc| Box::new(MultProgram::new(cc))))
        .expect("Unexpected Error");
}

#[derive(Clone, Copy, PartialEq)]
enum Algorithm {
    Sha256,
    Sha224,
    Sha512,
    Sha384
}

impl fmt::Display for Algorithm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Algorithm::Sha256 => write!(f,"SHA-256"),
            Algorithm::Sha512 => write!(f,"SHA-512"),
            Algorithm::Sha224 => write!(f, "SHA-224"),
            Algorithm::Sha384 => write!(f, "SHA-384")
        }
    }
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
    alg:Algorithm,

    tx:Sender<(Algorithm, String)>,
    rx:Receiver<Message>

}

fn hashing_thread(tx:Sender<Message>, rx:Receiver<(Algorithm, String)>) {
    for (a, s) in rx.iter() {
        match a {
            Algorithm::Sha256 => {
                let i = sha::preprocessing::sha256_preprocessing(&s);
                let hex_text = print_blocks(&i,true);
                tx.send(Message::Hex(hex_text)).unwrap();
        
                let hash = sha::sha256::sha_256(i);
                let hash_text = print_blocks(&vec![hash],true);
                tx.send(Message::Hash(hash_text)).unwrap();
            },

            Algorithm::Sha224 => {
                let i = sha::preprocessing::sha256_preprocessing(&s);
                let hex_text = print_blocks(&i,true);
                tx.send(Message::Hex(hex_text)).unwrap();
        
                let hash = sha::sha256::sha_224(i);
                let hash_text = print_u32_word_string(&hash.to_vec());
                tx.send(Message::Hash(hash_text)).unwrap();
            }

            Algorithm::Sha512 => {
                let i = sha::preprocessing::sha512_preprocessing(&s);
                let hex_text = print_blocks(&i,true);
                tx.send(Message::Hex(hex_text)).unwrap();
        
                let hash = sha::sha512::sha_512(i);
                let hash_text = print_blocks(&vec![hash],true);
                tx.send(Message::Hash(hash_text)).unwrap();
            },

            Algorithm::Sha384 => {
                //same preprocessing as sha512 with slightly different algorithm
                let i = sha::preprocessing::sha512_preprocessing(&s);
                let hex_text = print_blocks(&i,true);
                tx.send(Message::Hex(hex_text)).unwrap();
        
                let hash = sha::sha512::sha_384(i);
                let hash_text = print_blocks(&vec![hash],true);
                tx.send(Message::Hash(hash_text)).unwrap();
            },
        }
        
    }
}

impl MultProgram {
    fn new(cc:&eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        let (tx1, rx1) = mpsc::channel();
        let (tx2, rx2) = mpsc::channel::<Message>();
        thread::spawn(move|| hashing_thread(tx2, rx1));
        Self {
            tx: tx1, rx: rx2,
            thread_active:false,
            input: "".to_owned(),
            hex: "".to_owned(),
            hash: "".to_owned(),
            alg: Algorithm::Sha256,
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

            egui::ComboBox::from_label("")
                .selected_text(format!("{}", &self.alg))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.alg, Algorithm::Sha512, "SHA-512");
                    ui.selectable_value(&mut self.alg, Algorithm::Sha384, "SHA-384");
                    ui.selectable_value(&mut self.alg, Algorithm::Sha256, "SHA-256");
                    ui.selectable_value(&mut self.alg, Algorithm::Sha224, "SHA-224");
                    

            });

            ui.group(|ui| {
                egui::Grid::new("stuff").show(ui, |ui| {
                    ui.label("[IN]:");
                    ui.add_sized((520., 20.), egui::TextEdit::singleline(&mut self.input).hint_text("Input Text..."));
                    if ui.button("Submit").clicked() && !self.thread_active {
                        let s = self.input.trim().to_owned();
                        self.tx.send((self.alg, s)).unwrap();
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
