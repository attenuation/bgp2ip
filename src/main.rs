use std::io::prelude::*;
use std::fs::File;
use std::io::BufWriter;
use std::io::BufReader;
use std::env;
use std::net::{IpAddr,Ipv4Addr,Ipv6Addr};
use rusqlite::types::ToSql;
use rusqlite::{params, Connection, Result,Transaction};
use rusqlite::NO_PARAMS;

#[derive(Debug)]
struct AsInfo {
    as_num: i32,
    as_info: String,
    country: String,
}
#[derive(Debug)]
struct BgpInfo {
    bgproute: String,
    as_num: i32,
}

fn as_info_to_sqlite(conn: &mut Transaction,as_info: AsInfo) -> Result<()>{
    conn.execute("insert into AS_INFO (asnum,asinfo,country)
                    values (?1,?2,?3)", 
                    params![as_info.as_num,as_info.as_info,as_info.country])?;
    Ok(())
}

fn bgp_info_to_sqlite(conn: &mut Transaction,bgp_info: BgpInfo) -> Result<()>{
    conn.execute("insert into BGP_INFO (bgproute,asnum)
                    values (?1,?2)", 
                    params![bgp_info.bgproute.to_string(),bgp_info.as_num])?;
    Ok(())
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let asname = File::open(&args[1]).unwrap();
    let asname_read = BufReader::new(asname);
    let bgpdump = File::open(&args[2]).unwrap();
    let bgpdump_read = BufReader::new(bgpdump);

    let mut conn = Connection::open("./ipdb.db").unwrap();
    conn.execute(
        "CREATE TABLE AS_INFO (
                  asnum           INTEGER PRIMARY KEY,
                  asinfo          TEXT NOT NULL,
                  country         TEXT
                  )",
        NO_PARAMS,
    ).unwrap();
    conn.execute(
        "CREATE TABLE BGP_INFO (
            bgproute  TEXT PRIMARY KEY NOT NULL,
            asnum             INTEGER     NOT NULL
        )",
        NO_PARAMS,
    ).unwrap();
    let mut as_old = 0i32;
    let mut bgproute_old = "";
    
    let mut tx = conn.transaction().unwrap();
    let mut count = 0;
    for line in bgpdump_read.lines() {
        let mut line_fs = line.unwrap();
        let elems: Vec<&str> = line_fs.split("|").collect();
        // println!("{:?}", &elems);
        if elems.len() < 5 {
            continue
        }
        let bgproute = elems[5];
        let aspath: Vec<i32> = elems[6]
            .split(' ')
            .map(|x| match x.parse::<i32>() {
                Ok(asn) => asn,
                Err(_) => {
                    0
                }
            })
        .collect::<Vec<i32>>();
        let mut asn: i32 = 0;
        if aspath.len() > 0 {
            asn = aspath[aspath.len() - 1];
        };
        if asn == as_old || bgproute_old == bgproute {
            continue;
        }
        as_old = asn;
        bgproute_old = bgproute_old;
        let bgp_info = BgpInfo{
            bgproute: bgproute.to_string(),
            as_num: asn
        };
        println!("{}: {}", bgp_info.bgproute,bgp_info.as_num);
        let res = bgp_info_to_sqlite(&mut tx, bgp_info);
        match res {
            Ok(_) => {},
            Err(_) => continue
        }

    }
    tx.commit().unwrap();

    let mut tx = conn.transaction().unwrap();
    for line in asname_read.lines() {
        let mut line_fs = line.unwrap();
        let mut elems: Vec<&str> = line_fs.split("   ").filter(|&i|i != "").collect();
        let as_num = elems[0][2..].to_string();
        if elems[0].contains("--No Registry Entry--") || elems[1].contains("--No Registry Entry--") {
            continue;
        }
        let elems_r:Vec<&str> = elems[1].split(",").collect();
        let mut as_info = elems_r[0..elems_r.len()-1].concat().trim().to_string();
        let mut as_country = elems_r[elems_r.len()-1].trim().to_string();
        let as_i = AsInfo{
            as_num: as_num.parse::<i32>().unwrap(),
            as_info: as_info,
            country: as_country,
        };
        as_info_to_sqlite(&mut tx, as_i).unwrap();
        dbg!(&elems);
    }    
    tx.commit().unwrap();
}
