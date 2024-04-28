use std::thread;
use std::sync::mpsc::{self, Sender, Receiver};

use eframe::egui;
use jisp_sha2 as sha;

fn main() {
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native("Concurrent App", native_options, Box::new(|cc| Box::new(MultProgram::new(cc))))
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
                let res = sha::printer::print_blocks(&i,true);
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

fn computation(i:i32) -> i32 {
    std::thread::sleep(std::time::Duration::from_secs(5));
    i*5
}


impl eframe::App for MultProgram {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.group(|ui| {
                egui::Grid::new("unimportant").show(ui, |ui| {
                    ui.label("Sequentual calculation:");
                    ui.end_row();
                    ui.add_sized((320., 20.), egui::TextEdit::singleline(&mut self.input1).hint_text("Input Numbers..."));

                    if ui.button("Submit").clicked() {
                        match self.input1.trim().parse::<i32>() {
                            Ok(i) => {
                                self.output1 = format!("Answer: [ {} ]",computation(i).to_string());
                            }
                            Err(_) => self.output1 = "[Input a valid integer]".to_owned(),
                        }  
                    }
                    ui.end_row();
                    ui.add_space(20.);
                    ui.end_row();
                    ui.horizontal_centered(|ui| ui.add(egui::Label::new(&self.output1).selectable(true)));
                });
                
            });

            if self.thread_active {
                match self.rx.try_recv() {
                    Ok(i) => {
                        self.output2 = format!("Answer: [ {} ]", i);
                        self.thread_active = false;
                    },
                    Err(_) => (), 
                }
            }

            ui.group(|ui| {
                egui::Grid::new("stuff").spacing((10.,20.)).show(ui, |ui| {
                    ui.label("Parallel calculation:");
                    ui.end_row();
                    ui.add_sized((320., 20.), egui::TextEdit::singleline(&mut self.input2).hint_text("Input Numbers..."));
                    if ui.button("Submit").clicked() && !self.thread_active {
                        self.tx.send(self.input2.trim().to_owned()).unwrap();
                        self.thread_active = true;
                        self.output2 = "[Loading...]".to_owned();
                    }
                    ui.end_row();
                    ui.add_space(20.);
                    ui.end_row();
                    ui.horizontal_centered(|ui| ui.add(egui::Label::new(&self.output2).selectable(true)));
                });
                
            })
            

        });
    }
}
