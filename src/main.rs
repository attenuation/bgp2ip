use std::io::prelude::*;
use std::fs::File;
use std::io::BufWriter;
use std::io::BufReader;
use std::env;
use std::net::{IpAddr,Ipv4Addr,Ipv6Addr};
use rusqlite::types::ToSql;
use rusqlite::{params, Connection, Result};
use rusqlite::NO_PARAMS;


struct AsInfo {
    as_num: i32,
    as_name: String,
    org_name: String,
    country: String,
}

struct BgpInfo {
    bgproute: String,
    as_num: i32,
}

fn as_info_to_sqlite(conn: &mut Connection,as_info: AsInfo) -> Result<()>{
    conn.execute("insert into AS_INFO (asnum,asname,orgname,country)
                    values (?1,?2,?3,?4)", 
                    params![as_info.as_num,as_info.as_name,as_info.org_name,as_info.country])?;
    Ok(())
}

fn bgp_info_to_sqlite(conn: &mut Connection,bgp_info: BgpInfo) -> Result<()>{
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
    // conn.execute(
    //     "CREATE TABLE AS_INFO (
    //               asnum           INTEGER PRIMARY KEY,
    //               asname          TEXT NOT NULL,
    //               orgname         TEXT NOT NULL,
    //               counrty         TEXT
    //               )",
    //     NO_PARAMS,
    // ).unwrap();
    // conn.execute(
    //     "CREATE TABLE BGP_INFO (
    //         id       INTEGER PRIMARY KEY AUTOINCREMENT,
    //         bgproute TEXT  NOT NULL,
    //         asnum             INTEGER     NOT NULL
    //     )",
    //     NO_PARAMS,
    // ).unwrap();


    // for line in bgpdump_read.lines() {
    //     let mut line_fs = line.unwrap();
    //     let elems: Vec<&str> = line_fs.split("|").collect();
    //     let bgproute = elems[5];
    //     print!("bgproute: {}", bgproute);
    //     let aspath: Vec<i32> = elems[6]
    //         .split(' ')
    //         .map(|x| match x.parse::<i32>() {
    //             Ok(asn) => asn,
    //             Err(_) => {
    //                 0
    //             }
    //         })
    //     .collect::<Vec<i32>>();
    //     let mut asn: i32 = 0;
    //     if aspath.len() > 0 {
    //         asn = aspath[aspath.len() - 1];
    //         println!(" asn: {}",asn)
    //     };
    //     let bgp_info = BgpInfo{
    //         bgproute: bgproute.to_string(),
    //         as_num: asn
    //     };
    //     bgp_info_to_sqlite(&mut conn, bgp_info).unwrap();
    // }
    for line in asname_read.lines() {
        let mut line_fs = line.unwrap();
        let elems: Vec<&str> = line_fs.split("  ").collect();
        dbg!(elems);
    }    
}
