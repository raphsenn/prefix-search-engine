// import searchtree.rs mod searchtree;
mod searchtree;

use std::fs::File;
use std::io::{self, BufRead};

use fltk::{app, window::Window, prelude::*, input::Input};
use fltk_theme::{ColorTheme, color_themes, WidgetTheme, ThemeType};

use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone)]
struct SearchEngine {
    tree: searchtree::SearchTree,
}


impl SearchEngine {

    fn new() -> Self {
        SearchEngine {
            tree: searchtree::SearchTree::new(),
        }
    }
      
    fn read(&mut self, file_path: &str) -> io::Result<()> {
        let file = File::open(file_path)?;
        let reader = io::BufReader::new(file);

        for line in reader.lines() {
            let line = line?;
            self.tree.push(line.to_string());
        }
    Ok(())
    }
    
    fn search(&mut self, prefix: String) -> Vec<String> {
        let lower = prefix.to_lowercase();
        let upper = lower.clone() + "{";
        let result = self.tree.lookup(lower.to_string(), upper.to_string());
        result
    }
    
    // Create simple window and run search program
    fn run(&mut self) {
        
        // Initialize FLTK-Toolkit
        let app = app::App::default().with_scheme(app::Scheme::Gtk);
       
        // Load fltk theme
        let theme = ColorTheme::new(color_themes::GRAY_THEME);
        
        // Load fltk widget theme
        let widget_theme = WidgetTheme::new(ThemeType::Dark);
        
        // set theme and widget theme
        theme.apply();
        widget_theme.apply();

        // Create Window
        let mut wind = Window::new(100, 100, 500, 500, "search-engine");
        
        // Search input (form user)
        let mut input = Input::new(50, 20, 400, 30, "");
        input.set_trigger(fltk::enums::CallbackTrigger::Changed);
        
        // Create an Rc<RefCell<SearchEngine>> to share within the closure
        let search_engine = Rc::new(RefCell::new(self.clone()));
        
        // Create a TextEditor to display the results with scrollbars
        let mut results_editor = fltk::text::TextEditor::new(50, 80, 400, 400, "");
        results_editor.set_buffer(fltk::text::TextBuffer::default());

        // Check if something new is written in text field
        input.set_callback(move |input| {
            let text = input.value();
            let result = search_engine.borrow_mut().search(text.to_string());
            // println!("{}", text);
            // println!("{:?}", result);

            //Clear the previous results
            results_editor.buffer().unwrap().set_text("");
           
            // visualize the first 50 results
            for r in 0..50 {
                if r < result.len() {
                    results_editor.buffer().unwrap().append(&format!("{}\n", &result[r]));}
            }
        });
        // mainwindow
        wind.end();
        wind.show();

        // start fltk-mainloop
        app.run().unwrap();

    }
}


fn main() -> io::Result<()> {
    let mut search_engine = SearchEngine::new();
    search_engine.read("top-1m.csv")?;
    search_engine.run(); 
    Ok(()) 
    }
