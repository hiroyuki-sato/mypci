//use std::fs::metadata;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use regex::Regex;

fn main() {

    let f = File::open("/usr/share/misc/pci.ids").unwrap();
    let reader = BufReader::new(f);
    let re_comment = Regex::new("^\\s*(#|$)").unwrap();
    let re_pci_dev = Regex::new("^[0-9a-f]{4}\\s+(.*)").unwrap();
    let re_pci_sub = Regex::new("^\\t[0-9a-f]{4}\\s+(.*)").unwrap();
    let re_pci_sdv = Regex::new("^\\t\\t[0-9a-f]{4}\\s+[0-9a-f]{4}\\s+(.*)").unwrap();

    let re_dev_cls = Regex::new("^C\\s+[0-9a-f]{2}\\s+(.*)").unwrap();
    let re_dev_sub = Regex::new("\\t[0-9a-f]{2}\\s+(.*)").unwrap();
    let re_dev_prg = Regex::new("\\t\\t[0-9a-f]{2}\\s+(.*)").unwrap();

    for line in reader.lines() {
        let l = line.unwrap();
        match re_comment.find(&l) {
            Some(_) =>  continue,
            None => (),
        };
        match re_pci_dev.find(&l) {
//            Some(_) => println!("{}",l),
            Some(_) => continue,
            _ => (),
        }
        match re_pci_sdv.find(&l) {
//            Some(_) => println!("{}",l),
            Some(_) => continue,
            _ => (),
        }
        match re_pci_sub.find(&l) {
            //Some(_) => println!("{}",l),
            Some(_) => continue,
            _ => (),
        }

        match re_dev_cls.find(&l) {
            //Some(_) => println!("{}",l),
            Some(_) => continue,
            _ => (),
        }
        match re_dev_sub.find(&l) {
            //Some(_) => println!("{}",l),
            Some(_) => continue,
            _ => (),
        }
        match re_dev_prg.find(&l) {
            //Some(_) => println!("{}",l),
            Some(_) => continue,
            _ => (),
        }

        println!("{}",l);
    } 
}
