mod auth;
mod error;
mod web;
mod signer;

use error::Error;
use http::uri::Uri;
use serde_json::{json, Value};
use std::env;
use std::str::FromStr;

fn post(uri: Uri, method: &str, params: Value) -> Result<(), Error> {
    let cmd = json!({
        "method": method,
        "params": params
    });
    let cmdstr = serde_json::to_string(&cmd)?;
    let value = web::post(&uri, cmdstr)?;
    println!("{}", value);
    Ok(())
}

fn main() -> Result<(), Error> {
    let mut args = env::args();
    let _ = args.next().expect("Command was not given arg[0]");
    let url_text = args
        .next()
        .expect("Usage: relay-manager <url> <command> [args] ...");
    let uri = Uri::from_str(&url_text)?;
    let command = args
        .next()
        .expect("Usage: relay-manager <url> <command> [args] ...");
    match &*command {
        "supportedmethods" | "listbannedpubkeys" | "listallowedpubkeys" | "listeventsneedingmoderation" |
        "listbannedevents" | "listallowedkinds" | "listblockedips" => post(uri, &*command, json!([]))?,
        "banpubkey" => {
            let pubkeytext = args.next().expect("Usage: relay-manager banpubkey <pubkey>");
            post(uri, &*command, json!([pubkeytext]))?
        },
        "allowpubkey" => {
            let pubkeytext = args.next().expect("Usage: relay-manager allowpubkey <pubkey>");
            post(uri, &*command, json!([pubkeytext]))?
        },
        "allowevent" => {
            let idtext = args.next().expect("Usage: relay-manager allowevent <id>");
            post(uri, &*command, json!([idtext]))?
        },
        "banevent" => {
            let idtext = args.next().expect("Usage: relay-manager banevent <id>");
            post(uri, &*command, json!([idtext]))?
        },
        "changerelayname" => {
            let newname = args.next().expect("Usage: relay-manager changerelayname <newname>");
            post(uri, &*command, json!([newname]))?
        },
        "changerelaydescription" => {
            let newdesc = args.next().expect("Usage: relay-manager changerelaydescription <newname>");
            post(uri, &*command, json!([newdesc]))?
        },
        "changerelayicon" => panic!("changerelayicon is unimplemented"),
        "allowkind" => {
            let kind = args.next().expect("Usage: relay-manager allowkind <kind>");
            post(uri, &*command, json!([kind]))?
        },
        "disallowkind" => {
            let kind = args.next().expect("Usage: relay-manager disallowkind <kind>");
            post(uri, &*command, json!([kind]))?
        },
        "blockip" => {
            let ip = args.next().expect("Usage: relay-manager blockip <ip>");
            post(uri, &*command, json!([ip]))?
        },
        "unblockip" => {
            let ip = args.next().expect("Usage: relay-manager unblockip <ip>");
            post(uri, &*command, json!([ip]))?
        },
        _ => panic!("Usage relay-manager <command> [args]...\n
Available Commands:  supportedmethods, banpubkey, listbannedpubkeys, allowpubkey, listallowedpubkeys, listeventsneedingmoderation, allowevent, banevent, listbannedevents, changerelayname, changerelaydescription, changerelayicon, allowkind, disallowkind, listallowedkinds, blockip, unblockip, listblockedips"),
    }

    Ok(())
}
