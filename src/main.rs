use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Mehsul{
    ad: String,
    qiymet: f64,
}

#[derive(Parser)]
struct Arqumentler{
    #[command(subcommand)]
    emr: Emr,
}

#[derive(Subcommand)]
enum Emr{
    Elave{
        #[arg(long)]
        ad: String,

        #[arg(long)]
        qiymet: f64,
    },
    Siyahi,
    Sil{
        #[arg(long)]
        ad: String,
    },
    Deyis{
        #[arg(long)]
        ad: String,

        #[arg(long)]
        qiymet: f64,
    },
}
fn oxu() ->Vec<Mehsul> {
    let metn = match std::fs::read_to_string("mehsullar.json"){
        Ok(t) => t,
        Err(_) => return Vec::new(),
    };
    serde_json::from_str(&metn).unwrap_or_default()
}
fn yaz(siyahi: &Vec<Mehsul>) ->Result<(), String>{
    let metn = serde_json::to_string_pretty(siyahi).map_err(|e| e.to_string())?;
    std::fs::write("mehsullar.json", metn).map_err(|e| e.to_string())?;
    Ok(())
}
fn main() {

    let arq = Arqumentler::parse();

    match arq.emr{
        Emr::Elave { ad, qiymet } =>{
            let mut siyahi = oxu();
            siyahi.push(Mehsul { ad, qiymet });
            match yaz(&siyahi) {
                Ok(_) =>println!("Elave olundu. Cemi {} mehsul var.", siyahi.len()),
                Err(e) => println!("Xeta: {e}"),
            }
        }
        Emr::Siyahi =>{
            let siyahi = oxu();
            if siyahi.is_empty() {
                println!("Siyahi bosdur");
            }else {
                for m in &siyahi{
                    println!("{} - {} AZN", m.ad, m.qiymet);
                }
            }
        }
        Emr::Sil { ad } =>{
            let mut siyahi = oxu();
            let evvel = siyahi.len();
            siyahi.retain(|m| m.ad.to_lowercase() != ad.to_lowercase());
            let sonra = siyahi.len();

            if evvel == sonra {
                println!("{} adli mehsul tapilmadi.",ad);
            }else {
                match yaz(&siyahi) {
                    Ok(_) =>println!("Silindi {ad}"),
                    Err(e) =>println!("Xeta: {e}"),
                }
            }
        }
        Emr::Deyis { ad, qiymet } =>{
            let mut siyahi = oxu();
            let tapildi = match siyahi
               .iter_mut()
               .find(|m| m.ad.to_lowercase() == ad.to_lowercase())
               {
                Some(m) =>{
                    m.qiymet = qiymet;
                    true
                }
                None => false
               };
               if tapildi {
                match yaz(&siyahi) {
                    Ok(_) =>println!("Qiymet deyisdi: {} - {} AZN", ad, qiymet),
                    Err(e) => println!("Xeta: {e}"),
                }
               }else {
                   println!("{} adli mehsul tapilmadi.", ad);
               }
        }
    }
}