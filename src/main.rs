//use std::fs::metadata;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use regex::Regex;

enum Token {
    Pci,
    PciSub,
    PciSubDev,
    PciClass,
    PciSubClass,
    PciProg,
}

#[derive(Debug)]
struct Pci<'a> {
    code: u16,
    name: &'a str,
}

#[derive(Debug)]
struct PciSub<'a> {
    vendor: &'a Pci<'a>,
    dev_code: u16,
    dev_name: &'a str,
}

struct PciSubDev {
    code: u16,
    name: String,
    dev_code: u8,
    dev_name: String,
    sub_vend: u8,
    sub_code: u8,
    sub_name: String,
}


fn hex_to_num(str: &str) -> u16 {

    let v = u16::from_str_radix(str,16);
    return v.unwrap();
}

fn main() {

//    let f = File::open("/usr/share/misc/pci.ids").unwrap();
    let f = File::open("./pci.ids").unwrap();
    let reader = BufReader::new(f);
    let re_comment = Regex::new(r"^\s*(#|$)").unwrap();
    //let re_pci_dev = Regex::new(r"^([0-9a-f]{4})\s+(.*)").unwrap();
    //let re_pci_sub = Regex::new(r"^\t[0-9a-f]{4}\s+(.*)").unwrap();
    let re_pci_sdv = Regex::new(r"^\t\t[0-9a-f]{4}\s+[0-9a-f]{4}\s+(.*)").unwrap();

    let re_dev_cls = Regex::new(r"^C\s+[0-9a-f]{2}\s+(.*)").unwrap();
    let re_dev_sub = Regex::new(r"\t[0-9a-f]{2}\s+(.*)").unwrap();
    let re_dev_prg = Regex::new(r"\t\t[0-9a-f]{2}\s+(.*)").unwrap();

    println!("{}",hex_to_num(&"FF".to_string()));
    let mut cur_pci: Option<Pci> = None;

    for line in reader.lines() {
        let l = line.unwrap();
        match re_comment.find(&l) {
            Some(_) =>  continue,
            None => (),
        };

        match pci_dev(&l) {
            Some((_token,pci)) => cur_pci = Some(pci),
            _ => (),
        }        
        match re_pci_sdv.find(&l) {
            Some(_) => continue,
            _ => (),
        }

        match pci_sub(&cur_pci.unwrap(),&l) {
            Some((_token,pci_sub)) => println!("{:?}",pci_sub),
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

//        println!("{}",l);
    } 
}

fn pci_dev<'a>(str: &'a str) -> Option<(Token,Pci<'a>)> {    
    let re_pci_dev = Regex::new(r"^([0-9a-f]{4})\s+(.*)").unwrap();

    match re_pci_dev.captures(str) {

        Some(cap) => {
            let pci = Pci{ 
                code: hex_to_num(cap.get(1).map_or("", |m| m.as_str())),
                name: cap.get(2).map_or("", |m| m.as_str() ),
            };
            Some((Token::Pci, pci))
        },
        _ => None
    }
}

fn pci_sub<'a>(pci: &'a Pci, str: &'a str) -> Option<(Token,PciSub<'a>)> {

    let re_pci_sub = Regex::new(r"^\t([0-9a-f]{4})\s+(.*)").unwrap();

    match re_pci_sub.captures(str) {

        Some(cap) => {
            let pci_sub = PciSub {
                vendor: pci,
                dev_code: hex_to_num(cap.get(1).map_or("", |m| m.as_str())),
                dev_name: cap.get(2).map_or("", |m| m.as_str() ),
            };
            Some((Token::PciSub, pci_sub))
        },
        _ => None
    }
}
