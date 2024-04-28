use std::thread;
use std::sync::mpsc::{self, Sender, Receiver};

use eframe::egui;
use jisp_sha2 as sha;

fn main() {
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native("SHA-2", native_options, Box::new(|cc| Box::new(MultProgram::new(cc))))
        .expect("Unexpected Error");
}

struct MultProgram {
    input1:String,
    output1:String,
    input2:String,
    output2:String,
    thread_active:bool,
    tx:Sender<String>,
    rx:Receiver<String>

}

impl MultProgram {
    fn new(cc:&eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        let (tx1, rx1) = mpsc::channel();
        let (tx2, rx2) = mpsc::channel();
        thread::spawn(move|| {
            let tx:Sender<String> = tx2;
            let rx:Receiver<String> = rx1;
            for s in rx.iter() {
                let i = sha::parser::sha256_preprocessing(&s);
                let res = sha::printer::print_blocks(&i,false);
                tx.send(res).unwrap();
            }
        });
        Self {
            tx: tx1, rx: rx2,
            thread_active:false,
            input1: "".to_owned(),
            input2: "".to_owned(),
            output1: "".to_owned(),
            output2: "".to_owned(),
        }
    }
}

impl eframe::App for MultProgram {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.thread_active {
                match self.rx.try_recv() {
                    Ok(s) => {
                        self.output2 = s;
                        self.thread_active = false;
                    },
                    Err(_) => (), 
                }
            }

            ui.group(|ui| {
                egui::Grid::new("stuff").show(ui, |ui| {
                    ui.label("[IN]:");
                    ui.add_sized((500., 20.), egui::TextEdit::singleline(&mut self.input2).hint_text("Input Text..."));
                    if ui.button("Submit").clicked() && !self.thread_active {
                        self.tx.send(self.input2.trim().to_owned()).unwrap();
                        self.thread_active = true;
                        self.output2 = "[Loading...]".to_owned();
                    }
                    ui.end_row();
                    ui.end_row();
                    ui.label("[OUT]:"); 
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {

                        //ui.style_mut().visuals.override_text_color = Some(egui::Color32::WHITE);
                        ui.add(egui::Label::new(&self.output2).selectable(true).wrap(true));
                    })
                    
                });
                
            })
            

        });
    }
}
